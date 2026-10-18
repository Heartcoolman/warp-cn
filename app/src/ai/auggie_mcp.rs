//! Stdio MCP client for the user's `auggie` (Augment context engine) binary.
//!
//! Spawns `auggie --mcp --mcp-auto-workspace` as a child process and exposes a
//! typed `codebase_retrieval` method. The child inherits Warp's process env
//! (incl. any `AUGMENT_SESSION_AUTH`); we never set it explicitly.

use std::path::PathBuf;
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use regex::Regex;
use rmcp::{transport::ConfigureCommandExt as _, ServiceExt as _};
use serde_json::json;
use tokio::io::AsyncBufReadExt as _;
use tokio::sync::{Mutex, OnceCell};
use warpui::{Entity, ModelContext, SingletonEntity};

const CODEBASE_RETRIEVAL_TOOL_NAME: &str = "codebase-retrieval";

type AuggieRunningService = rmcp::service::RunningService<
    rmcp::RoleClient,
    Box<dyn rmcp::service::DynService<rmcp::RoleClient>>,
>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AuggieExcerpt {
    pub path: PathBuf,
    pub line_start: usize,
    pub line_end: usize,
    pub content: String,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum AuggieMcpError {
    #[error("auggie MCP server failed to spawn: {0}")]
    Spawn(String),
    #[error("auggie MCP transport closed after retry")]
    TransportClosed,
    #[error("auggie MCP server does not expose codebase-retrieval tool")]
    ToolNotFound,
    #[error("auggie response parse failure: {0}")]
    Parse(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[async_trait]
pub(crate) trait AuggieMcpClient: Send + Sync + 'static {
    async fn codebase_retrieval(
        &self,
        query: String,
        directory: PathBuf,
    ) -> Result<Vec<AuggieExcerpt>, AuggieMcpError>;
}

pub(crate) struct AuggieMcpClientModel {
    client: Arc<OnceCell<Arc<dyn AuggieMcpClient>>>,
    last_spawn_failed: Arc<AtomicBool>,
}

impl AuggieMcpClientModel {
    pub(crate) fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self {
            client: Arc::new(OnceCell::const_new()),
            last_spawn_failed: Arc::new(AtomicBool::new(false)),
        }
    }

    #[cfg(test)]
    pub(crate) fn new_for_test(client: Arc<dyn AuggieMcpClient>) -> Self {
        let cell = Arc::new(OnceCell::const_new());
        let _ = cell.set(client);
        Self {
            client: cell,
            last_spawn_failed: Arc::new(AtomicBool::new(false)),
        }
    }

    #[cfg(test)]
    pub(crate) fn new_for_test_unavailable() -> Self {
        Self {
            client: Arc::new(OnceCell::const_new()),
            last_spawn_failed: Arc::new(AtomicBool::new(true)),
        }
    }

    /// Returns an `Arc<dyn AuggieMcpClient>` whose first `codebase_retrieval`
    /// call lazily spawns the auggie MCP service. Suitable for handing to
    /// `AuggieStoreClient`, which needs an owned client without holding a
    /// `ModelContext`.
    pub(crate) fn client_handle(&self) -> Arc<dyn AuggieMcpClient> {
        Arc::new(LazyAuggieMcpClient {
            client: self.client.clone(),
            last_spawn_failed: self.last_spawn_failed.clone(),
        })
    }

    /// Synchronous availability snapshot for settings UI. `true` once a spawn
    /// attempt has failed; flips back to `false` on the next successful spawn.
    ///
    /// `Ordering::Relaxed` is sound: the flag is an independent advisory bool
    /// with no companion data that needs synchronization, and the UI only
    /// renders an eventually-correct tooltip.
    pub(crate) fn is_unavailable(&self) -> bool {
        self.last_spawn_failed.load(Ordering::Relaxed)
    }

    pub(crate) async fn client(&self) -> Result<Arc<dyn AuggieMcpClient>, AuggieMcpError> {
        Self::client_from_cell(&self.client, &self.last_spawn_failed).await
    }

    async fn client_from_cell(
        client: &OnceCell<Arc<dyn AuggieMcpClient>>,
        last_spawn_failed: &Arc<AtomicBool>,
    ) -> Result<Arc<dyn AuggieMcpClient>, AuggieMcpError> {
        client
            .get_or_try_init(|| async {
                let service = AuggieMcpService::spawn_with_flag(last_spawn_failed.clone()).await?;
                Ok(Arc::new(service) as Arc<dyn AuggieMcpClient>)
            })
            .await
            .cloned()
    }
}

impl Entity for AuggieMcpClientModel {
    type Event = ();
}

impl SingletonEntity for AuggieMcpClientModel {}

struct LazyAuggieMcpClient {
    client: Arc<OnceCell<Arc<dyn AuggieMcpClient>>>,
    last_spawn_failed: Arc<AtomicBool>,
}

#[async_trait]
impl AuggieMcpClient for LazyAuggieMcpClient {
    async fn codebase_retrieval(
        &self,
        query: String,
        directory: PathBuf,
    ) -> Result<Vec<AuggieExcerpt>, AuggieMcpError> {
        AuggieMcpClientModel::client_from_cell(&self.client, &self.last_spawn_failed)
            .await?
            .codebase_retrieval(query, directory)
            .await
    }
}

struct AuggieMcpConnection {
    service: AuggieRunningService,
    tools: Vec<rmcp::model::Tool>,
}

pub(crate) struct AuggieMcpService {
    connection: Mutex<Option<AuggieMcpConnection>>,
    /// Shared with `AuggieMcpClientModel`; flipped on every (re)connect outcome
    /// so the settings UI can surface an "auggie unavailable" tooltip.
    last_spawn_failed: Arc<AtomicBool>,
}

impl AuggieMcpService {
    async fn spawn_with_flag(last_spawn_failed: Arc<AtomicBool>) -> Result<Self, AuggieMcpError> {
        match spawn_connection().await {
            Ok(connection) => {
                last_spawn_failed.store(false, Ordering::Relaxed);
                Ok(Self {
                    connection: Mutex::new(Some(connection)),
                    last_spawn_failed,
                })
            }
            Err(err) => {
                last_spawn_failed.store(true, Ordering::Relaxed);
                Err(err)
            }
        }
    }

    async fn reconnect(&self) -> Result<(), AuggieMcpError> {
        let mut connection = self.connection.lock().await;
        if let Some(old_connection) = connection.take() {
            cancel_connection(old_connection);
        }
        match spawn_connection().await {
            Ok(new_connection) => {
                *connection = Some(new_connection);
                self.last_spawn_failed.store(false, Ordering::Relaxed);
                Ok(())
            }
            Err(err) => {
                self.last_spawn_failed.store(true, Ordering::Relaxed);
                Err(err)
            }
        }
    }

    async fn peer_for_codebase_retrieval(
        &self,
    ) -> Result<rmcp::Peer<rmcp::RoleClient>, AuggieMcpError> {
        let connection = self.connection.lock().await;
        let connection = connection.as_ref().ok_or(AuggieMcpError::TransportClosed)?;
        if !connection
            .tools
            .iter()
            .any(|tool| tool.name == CODEBASE_RETRIEVAL_TOOL_NAME)
        {
            return Err(AuggieMcpError::ToolNotFound);
        }
        Ok(connection.service.peer().clone())
    }

    async fn codebase_retrieval_once(
        &self,
        query: String,
        directory: PathBuf,
    ) -> Result<Vec<AuggieExcerpt>, AuggieMcpError> {
        let peer = self.peer_for_codebase_retrieval().await?;
        let result = peer
            .call_tool(
                rmcp::model::CallToolRequestParams::new(CODEBASE_RETRIEVAL_TOOL_NAME)
                    .with_arguments(rmcp::model::object(json!({
                        "information_request": query,
                        "directory_path": directory.to_string_lossy(),
                    }))),
            )
            .await
            .map_err(map_service_error)?;

        if result.is_error == Some(true) {
            return Err(AuggieMcpError::Other(anyhow!(
                "auggie codebase-retrieval returned an error: {}",
                text_from_call_tool_result(&result)
            )));
        }

        parse_excerpts(&text_from_call_tool_result(&result))
    }
}

#[async_trait]
impl AuggieMcpClient for AuggieMcpService {
    async fn codebase_retrieval(
        &self,
        query: String,
        directory: PathBuf,
    ) -> Result<Vec<AuggieExcerpt>, AuggieMcpError> {
        match self
            .codebase_retrieval_once(query.clone(), directory.clone())
            .await
        {
            Err(AuggieMcpError::TransportClosed) => {
                log::debug!("auggie MCP transport closed; reconnecting once");
                self.reconnect().await?;
                self.codebase_retrieval_once(query, directory).await
            }
            result => result,
        }
    }
}

impl Drop for AuggieMcpService {
    fn drop(&mut self) {
        if let Some(connection) = self.connection.get_mut().take() {
            cancel_connection(connection);
        }
    }
}

async fn spawn_connection() -> Result<AuggieMcpConnection, AuggieMcpError> {
    let command = tokio::process::Command::new("auggie").configure(|cmd| {
        cmd.args(["--mcp", "--mcp-auto-workspace"]);

        #[cfg(windows)]
        cmd.creation_flags(windows::Win32::System::Threading::CREATE_NO_WINDOW.0);
    });

    let (transport, stderr) = rmcp::transport::TokioChildProcess::builder(command)
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| AuggieMcpError::Spawn(err.to_string()))?;

    let pid = transport
        .id()
        .map(|pid| pid.to_string())
        .unwrap_or_else(|| "??".to_string());

    if let Some(stderr) = stderr {
        tokio::spawn(async move {
            let mut reader = tokio::io::BufReader::new(stderr);
            let mut buf = String::new();
            loop {
                buf.clear();
                match reader.read_line(&mut buf).await {
                    Ok(0) => return,
                    Ok(_) => log::debug!("auggie MCP [pid: {pid}] stderr: {}", buf.trim_end()),
                    Err(err) => {
                        log::debug!("failed to read auggie MCP stderr: {err}");
                        return;
                    }
                }
            }
        });
    }

    let service = make_client_info()
        .into_dyn()
        .serve(transport)
        .await
        .map_err(|err| AuggieMcpError::Spawn(err.to_string()))?;

    let tools = match service.list_all_tools().await {
        Ok(tools) => tools,
        Err(rmcp::ServiceError::McpError(rmcp::model::ErrorData { code, .. }))
            if code == rmcp::model::ErrorCode::METHOD_NOT_FOUND =>
        {
            vec![]
        }
        Err(err) => return Err(map_service_error(err)),
    };

    if !tools
        .iter()
        .any(|tool| tool.name == CODEBASE_RETRIEVAL_TOOL_NAME)
    {
        return Err(AuggieMcpError::ToolNotFound);
    }

    Ok(AuggieMcpConnection { service, tools })
}

fn cancel_connection(connection: AuggieMcpConnection) {
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        handle.spawn(async move {
            let _ = connection.service.cancel().await;
        });
    } else {
        connection.service.cancellation_token().cancel();
    }
}

fn make_client_info() -> rmcp::model::ClientInfo {
    rmcp::model::InitializeRequestParams::new(
        Default::default(),
        rmcp::model::Implementation::new(
            warp_core::channel::ChannelState::app_id().to_string(),
            warp_core::channel::ChannelState::app_version()
                .map(|version| version.to_string())
                .unwrap_or_default(),
        ),
    )
}

fn map_service_error(err: rmcp::ServiceError) -> AuggieMcpError {
    match err {
        rmcp::ServiceError::TransportClosed => AuggieMcpError::TransportClosed,
        other => AuggieMcpError::Other(anyhow!(other)),
    }
}

fn text_from_call_tool_result(result: &rmcp::model::CallToolResult) -> String {
    let mut text_blocks = Vec::new();
    for content in &result.content {
        match content {
            rmcp::model::ContentBlock::Text(text_content) => {
                text_blocks.push(text_content.text.clone());
            }
            other => {
                log::debug!("ignoring non-text auggie MCP content block: {other:?}");
            }
        }
    }
    text_blocks.join("\n")
}

#[derive(Default)]
struct ExcerptBuilder {
    path: Option<PathBuf>,
    line_start: Option<usize>,
    line_end: Option<usize>,
    content: Vec<String>,
}

impl ExcerptBuilder {
    fn finish(self) -> Option<AuggieExcerpt> {
        Some(AuggieExcerpt {
            path: self.path?,
            line_start: self.line_start?,
            line_end: self.line_end?,
            content: self.content.join("\n").trim().to_string(),
        })
    }
}

fn parse_excerpts(response: &str) -> Result<Vec<AuggieExcerpt>, AuggieMcpError> {
    let path_re =
        Regex::new(r"^\s*(?:[#>*-]\s*)*Path:\s*(.+?)\s*$").expect("auggie path regex is valid");
    let line_re = Regex::new(
        r"^\s*(?:\.\.\.\s*)?(?:L)?(\d+)(?:\s*(?:-|:|,|\.\.)\s*(?:L)?(\d+))?\s*(?:[:|]\s*)?(.*)$",
    )
    .expect("auggie line regex is valid");

    let mut excerpts = Vec::new();
    let mut current = ExcerptBuilder::default();

    for line in response.lines() {
        if let Some(captures) = path_re.captures(line) {
            if let Some(excerpt) = current.finish() {
                excerpts.push(excerpt);
            }
            let path = captures
                .get(1)
                .map(|m| m.as_str().trim().trim_matches('`'))
                .unwrap_or_default();
            current = ExcerptBuilder {
                path: Some(PathBuf::from(path)),
                ..Default::default()
            };
            continue;
        }

        if current.path.is_none() {
            continue;
        }

        if let Some(captures) = line_re.captures(line) {
            let Some(line_start) = captures
                .get(1)
                .and_then(|m| m.as_str().parse::<usize>().ok())
            else {
                continue;
            };
            let line_end = captures
                .get(2)
                .and_then(|m| m.as_str().parse::<usize>().ok())
                .unwrap_or(line_start);
            current.line_start = Some(current.line_start.map_or(line_start, |n| n.min(line_start)));
            current.line_end = Some(current.line_end.map_or(line_end, |n| n.max(line_end)));
            if let Some(content) = captures.get(3) {
                current.content.push(content.as_str().to_string());
            }
        } else if current.line_start.is_some() {
            current
                .content
                .push(line.trim_start_matches('.').trim_start().to_string());
        }
    }

    if let Some(excerpt) = current.finish() {
        excerpts.push(excerpt);
    }

    if excerpts.is_empty() && !response.trim().is_empty() {
        let sample = response.lines().take(8).collect::<Vec<_>>().join("\\n");
        log::debug!("failed to parse auggie codebase-retrieval response; sample: {sample}");
        return Err(AuggieMcpError::Parse(
            "no Path:/line-range excerpt blocks found".to_string(),
        ));
    }

    Ok(excerpts)
}

#[cfg(test)]
pub(crate) struct MockAuggieMcpClient {
    excerpts: Vec<AuggieExcerpt>,
}

#[cfg(test)]
impl MockAuggieMcpClient {
    pub(crate) fn new(excerpts: Vec<AuggieExcerpt>) -> Self {
        Self { excerpts }
    }
}

#[cfg(test)]
#[async_trait]
impl AuggieMcpClient for MockAuggieMcpClient {
    async fn codebase_retrieval(
        &self,
        _query: String,
        _directory: PathBuf,
    ) -> Result<Vec<AuggieExcerpt>, AuggieMcpError> {
        Ok(self.excerpts.clone())
    }
}

#[cfg(test)]
mod availability_tests {
    use super::*;

    #[test]
    fn fresh_model_reports_available() {
        let model = AuggieMcpClientModel::new_for_test(Arc::new(MockAuggieMcpClient::new(vec![])));
        assert!(!model.is_unavailable());
    }

    #[test]
    fn recorded_failure_reports_unavailable() {
        let model = AuggieMcpClientModel::new_for_test_unavailable();
        assert!(model.is_unavailable());
    }
}
