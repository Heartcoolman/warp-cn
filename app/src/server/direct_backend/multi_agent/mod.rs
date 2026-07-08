//! Direct LLM backend — multi-agent (Agent Mode) entry point.
//!
//! M4.2 ships 9-tool support against three providers:
//! Anthropic Messages, OpenAI Chat Completions (+ compatible gateways), and
//! Gemini `:generateContent`. Decoding is provider-neutral via [`decode`] and
//! [`tool_schema`]; each driver projects the same [`decode::NormalizedTurn`]
//! sequence onto its own wire shape.

pub mod adapter;
pub mod anthropic_driver;
pub mod decode;
pub mod encode;
pub mod gemini_driver;
pub mod openai_driver;
pub mod tool_schema;

#[cfg(test)]
mod sse_integration_tests;

use std::sync::Arc;
use std::time::Duration;

#[cfg(not(target_family = "wasm"))]
use futures::stream::BoxStream;
use futures::StreamExt;
use uuid::Uuid;
use warp_multi_agent_api as api;

use super::{DirectProviderKind, ResolvedProvider};
use crate::server::server_api::AIApiError;

// Replacement for the old `server_api::AIOutputStream` alias, removed upstream
// when the multi-agent HTTP client moved to `warp_multi_agent_client`.
cfg_if::cfg_if! {
    if #[cfg(target_family = "wasm")] {
        pub type AIOutputStream<T> =
            futures::stream::LocalBoxStream<'static, Result<T, Arc<AIApiError>>>;
    } else {
        pub type AIOutputStream<T> =
            futures::stream::BoxStream<'static, Result<T, Arc<AIApiError>>>;
    }
}

/// Hard ceiling on a single provider HTTP call. Without this any of the
/// drivers' `reqwest::Client::send().await` calls could hang indefinitely on
/// a stalled proxy / TCP connection / never-ending body, leaving the UI
/// waiting on a stream that emitted `Init` but never `Finished`.
pub(super) const HTTP_TIMEOUT: Duration = Duration::from_secs(120);

#[allow(dead_code)] // legacy stub path; preserved for explicit fallback callers.
const STUB_MESSAGE: &str = "Direct LLM mode is enabled (warp-cn fork). Agent Mode tool calling \
     isn't yet wired against your provider in this build — use the AI \
     Chat side panel or `#` natural-language commands while M4 follow-up \
     phases ship real tool support.";

const NO_PROVIDER: &str = "Direct LLM Agent Mode (M4.2) needs provider credentials. \
     Set `WARP_CN_DIRECT_PROVIDER=anthropic|openai|openai-compatible|gemini` \
     and `WARP_CN_API_KEY=...` in your environment, then restart Warp.";

/// One block produced by a driver: either a plain text reply or a parsed
/// tool invocation. `encode.rs` dispatches generically off `tool`.
#[derive(Debug, Clone)]
pub enum DecodedBlock {
    Text(String),
    ToolUse {
        tool_use_id: String,
        tool: api::message::tool_call::Tool,
    },
}

#[derive(Debug, Clone)]
pub struct DriverOutput {
    pub blocks: Vec<DecodedBlock>,
    pub stop_reason: Option<String>,
    pub input_tokens: Option<u32>,
    pub output_tokens: Option<u32>,
    #[allow(dead_code)]
    pub input_cache_read: Option<u32>,
    pub model_id: String,
}

/// Provider-neutral streaming chunk emitted by drivers' `call_streaming`.
/// `block_idx` is the producer-assigned content-block index used to
/// distinguish concurrent text streams (Anthropic can interleave multiple
/// text blocks; OpenAI never does; Gemini emits text per part).
#[derive(Debug, Clone)]
pub enum DriverStreamChunk {
    /// Token(s) for an assistant text block. The adapter will start a new
    /// AgentOutput message on first `block_idx` and append on subsequent ones.
    TextDelta {
        block_idx: u32,
        text: String,
    },
    /// Chain-of-thought tokens emitted by reasoning models (DeepSeek-R1 /
    /// o1-style). Rendered as a separate `AgentReasoning` message so it
    /// shows up as a foldable thinking block instead of being concatenated
    /// onto the visible reply.
    ReasoningDelta {
        block_idx: u32,
        text: String,
    },
    /// A tool_use block has begun. Adapter ignores; here for tracing.
    #[allow(dead_code)]
    ToolUseStart {
        block_idx: u32,
        tool_use_id: String,
        name: String,
    },
    /// A tool_use finished and the buffered JSON parsed cleanly.
    ToolUseComplete {
        #[allow(dead_code)]
        block_idx: u32,
        tool_use_id: String,
        name: String,
        parsed_input: serde_json::Value,
    },
    /// Soft tool error (parse failure / unknown name). Adapter surfaces it as
    /// inline `[tool error]` text so the model can self-correct next turn.
    ToolUseSoftError {
        message: String,
    },
    /// Terminal chunk. After this, no more chunks; the stream MUST be closed.
    Stop {
        stop_reason: Option<String>,
        input_tokens: Option<u32>,
        output_tokens: Option<u32>,
        input_cache_read: Option<u32>,
        model_id: String,
    },
}

#[cfg(target_family = "wasm")]
pub type DriverChunkStream =
    futures::stream::LocalBoxStream<'static, anyhow::Result<DriverStreamChunk>>;
#[cfg(not(target_family = "wasm"))]
pub type DriverChunkStream = BoxStream<'static, anyhow::Result<DriverStreamChunk>>;

/// Drain a chunk stream into the legacy `DriverOutput` shape so existing
/// `pub async fn call(...)` tests keep passing without rewriting them.
/// Text chunks for the same `block_idx` are concatenated and emitted in
/// first-seen order; ToolUseComplete chunks parse + register; the final
/// `Stop` provides usage + stop_reason.
pub(super) async fn aggregate_stream_to_output(
    mut s: DriverChunkStream,
) -> anyhow::Result<DriverOutput> {
    use std::collections::HashMap;
    let mut text_by_idx: HashMap<u32, usize> = HashMap::new(); // idx -> position in `blocks`
    let mut blocks: Vec<DecodedBlock> = Vec::new();
    let mut stop_reason: Option<String> = None;
    let mut input_tokens: Option<u32> = None;
    let mut output_tokens: Option<u32> = None;
    let mut input_cache_read: Option<u32> = None;
    let mut model_id: String = String::new();

    while let Some(item) = s.next().await {
        match item? {
            DriverStreamChunk::TextDelta { block_idx, text } => {
                if text.is_empty() {
                    continue;
                }
                if let Some(&pos) = text_by_idx.get(&block_idx) {
                    if let DecodedBlock::Text(ref mut existing) = blocks[pos] {
                        existing.push_str(&text);
                    }
                } else {
                    let pos = blocks.len();
                    blocks.push(DecodedBlock::Text(text));
                    text_by_idx.insert(block_idx, pos);
                }
            }
            DriverStreamChunk::ReasoningDelta { .. } => {
                // The legacy aggregator path is only used by the 36 baseline
                // unit tests, which assert on visible text. Reasoning tokens
                // are intentionally dropped here so test assertions stay
                // stable; the streaming path renders them as a separate
                // `AgentReasoning` bubble in the UI.
            }
            DriverStreamChunk::ToolUseStart { .. } => {}
            DriverStreamChunk::ToolUseComplete {
                tool_use_id,
                name,
                parsed_input,
                ..
            } => {
                let kind = match tool_schema::from_name(&name) {
                    Some(k) => k,
                    None => {
                        blocks.push(DecodedBlock::Text(format!(
                            "[tool error] unknown tool `{name}`"
                        )));
                        continue;
                    }
                };
                match tool_schema::parse_input(kind, parsed_input) {
                    Ok(tool) => blocks.push(DecodedBlock::ToolUse {
                        tool_use_id,
                        tool,
                    }),
                    Err(e) => blocks.push(DecodedBlock::Text(format!(
                        "[tool error] `{name}` malformed input: {e}"
                    ))),
                }
            }
            DriverStreamChunk::ToolUseSoftError { message } => {
                blocks.push(DecodedBlock::Text(message));
            }
            DriverStreamChunk::Stop {
                stop_reason: sr,
                input_tokens: it,
                output_tokens: ot,
                input_cache_read: icr,
                model_id: mid,
            } => {
                stop_reason = sr;
                input_tokens = it;
                output_tokens = ot;
                input_cache_read = icr;
                if !mid.is_empty() {
                    model_id = mid;
                }
            }
        }
    }

    Ok(DriverOutput {
        blocks,
        stop_reason,
        input_tokens,
        output_tokens,
        input_cache_read,
        model_id,
    })
}

/// Compose the final `system` prompt sent to a provider. Appends a render of
/// the request's `MCPContext.servers` so the model knows which `server_id` /
/// resource URIs / tool names are valid for `read_mcp_resource` and
/// `call_mcp_tool`. Returns the raw `base` unchanged when no servers are
/// configured.
///
/// User-controlled MCP metadata (server names, descriptions, resource URIs,
/// tool descriptions) is JSON-encoded so embedded newlines, quotes, or
/// instruction-like text from a malicious MCP server cannot escape the
/// listing format and become prompt injection.
pub(super) fn compose_system_prompt(
    base: &str,
    mcp: Option<&api::request::McpContext>,
) -> String {
    let Some(ctx) = mcp else { return base.to_string() };
    if ctx.servers.is_empty() {
        return base.to_string();
    }
    let servers_json: Vec<serde_json::Value> = ctx
        .servers
        .iter()
        .map(|srv| {
            let resources: Vec<serde_json::Value> = srv
                .resources
                .iter()
                .map(|r| {
                    serde_json::json!({
                        "uri": r.uri,
                        "name": r.name,
                        "mime_type": r.mime_type,
                    })
                })
                .collect();
            let tools: Vec<serde_json::Value> = srv
                .tools
                .iter()
                .map(|t| {
                    serde_json::json!({
                        "name": t.name,
                        "description": t.description,
                        "input_schema": t.input_schema.as_ref().map(prost_struct_to_serde_json),
                    })
                })
                .collect();
            serde_json::json!({
                "name": srv.name,
                "id": srv.id,
                "description": srv.description,
                "resources": resources,
                "tools": tools,
            })
        })
        .collect();
    let pretty = serde_json::to_string_pretty(&servers_json).unwrap_or_else(|_| "[]".into());

    let mut out = String::with_capacity(base.len() + pretty.len() + 256);
    out.push_str(base);
    out.push_str("\n\n--- Available MCP servers (JSON catalog; use exact `id` values) ---\n");
    out.push_str(&pretty);
    out.push_str(
        "\n--- end MCP catalog ---\n\n\
         To interact with the catalog above you MUST use the `call_mcp_tool` or \
         `read_mcp_resource` action — never invoke an MCP server's tool directly. \
         Pass the server's `id` as `server_id` and the tool's `name` exactly as listed. \
         Do NOT invent server_ids, resource URIs, or tool names that are not in the catalog.\n",
    );
    out
}

/// Convert a `prost_types::Struct` (proto JSON object) into `serde_json::Value`.
/// Used both by `compose_system_prompt` and the MCP tool registry.
pub(super) fn prost_struct_to_serde_json(s: &prost_types::Struct) -> serde_json::Value {
    let mut map = serde_json::Map::with_capacity(s.fields.len());
    for (k, v) in &s.fields {
        map.insert(k.clone(), prost_value_to_serde_json(v));
    }
    serde_json::Value::Object(map)
}

pub(super) fn prost_value_to_serde_json(v: &prost_types::Value) -> serde_json::Value {
    use prost_types::value::Kind;
    match v.kind.as_ref() {
        None | Some(Kind::NullValue(_)) => serde_json::Value::Null,
        Some(Kind::BoolValue(b)) => serde_json::Value::Bool(*b),
        Some(Kind::NumberValue(n)) => serde_json::Number::from_f64(*n)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        Some(Kind::StringValue(s)) => serde_json::Value::String(s.clone()),
        Some(Kind::StructValue(s)) => prost_struct_to_serde_json(s),
        Some(Kind::ListValue(l)) => serde_json::Value::Array(
            l.values.iter().map(prost_value_to_serde_json).collect(),
        ),
    }
}

/// Convert a `serde_json::Value` (must be an object) into a `prost_types::Struct`.
/// Returns an error if `value` is not a JSON object at the root.
pub(super) fn serde_json_to_prost_struct(
    value: &serde_json::Value,
) -> anyhow::Result<prost_types::Struct> {
    let obj = value
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("expected JSON object at root"))?;
    let mut fields = std::collections::BTreeMap::new();
    for (k, v) in obj {
        fields.insert(k.clone(), serde_json_to_prost_value(v));
    }
    Ok(prost_types::Struct { fields })
}

pub(super) fn serde_json_to_prost_value(v: &serde_json::Value) -> prost_types::Value {
    use prost_types::value::Kind;
    let kind = match v {
        serde_json::Value::Null => Kind::NullValue(0),
        serde_json::Value::Bool(b) => Kind::BoolValue(*b),
        serde_json::Value::Number(n) => Kind::NumberValue(n.as_f64().unwrap_or(0.0)),
        serde_json::Value::String(s) => Kind::StringValue(s.clone()),
        serde_json::Value::Array(a) => Kind::ListValue(prost_types::ListValue {
            values: a.iter().map(serde_json_to_prost_value).collect(),
        }),
        serde_json::Value::Object(_) => {
            // serde_json_to_prost_struct enforces root-object; for nested we
            // still need a Struct, never returning an error since we've matched.
            Kind::StructValue(serde_json_to_prost_struct(v).unwrap_or_default())
        }
    };
    prost_types::Value { kind: Some(kind) }
}

/// Public entry. Emits `StreamInit` synchronously (so the UI can scaffold
/// its bubble while we wait on the LLM), pre-emits `CreateTask` for first-turn
/// flows, then opens the provider's SSE stream and adapts each chunk into
/// `ResponseEvent`s — text deltas become `AppendToMessageContent`, tool_use
/// completions become `AddMessagesToTask{ToolCall}`, and the terminal
/// `Stop` becomes `Finished{Done, token_usage}`.
pub fn run(request: &api::Request) -> AIOutputStream<api::ResponseEvent> {
    let conversation_id = request
        .metadata
        .as_ref()
        .map(|m| m.conversation_id.clone())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let mut provider = match resolve_provider() {
        Some(p) => p,
        None => return error_stream(conversation_id, NO_PROVIDER.into()),
    };

    let decoded = decode::decode(request);
    if !decoded.has_user_input {
        return error_stream(
            conversation_id,
            "Direct LLM Agent Mode received a request with no user input.".into(),
        );
    }

    // Picker selection overrides the per-provider default. Without this the
    // dynamic catalog ID (e.g. `deepseek-v4-flash`) shown in `/MODEL` would
    // be ignored and DeepSeek would reject the request because we shipped
    // the OpenAI default `gpt-4o-mini`.
    if let Some(base) = decoded.base_model.as_deref() {
        provider.model_id = base.to_owned();
    }

    log::info!(
        "DirectBackend run: provider={:?} model={} turns={} mcp_servers={} existing_task={}",
        provider.kind,
        provider.model_id,
        decoded.turns.len(),
        decoded.mcp_context.as_ref().map(|m| m.servers.len()).unwrap_or(0),
        decoded.existing_task_id.is_some(),
    );

    let conv_id = if decoded.conversation_id.is_empty() {
        conversation_id
    } else {
        decoded.conversation_id
    };

    // Emit StreamInit BEFORE awaiting the driver so the UI doesn't appear hung.
    let (init, request_id) = encode::build_init_event(conv_id);
    let existing_task_id = decoded.existing_task_id.clone();
    let task_id = decoded
        .existing_task_id
        .clone()
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    let turns = decoded.turns;
    let mcp_context = decoded.mcp_context;

    let mut prelude: Vec<api::ResponseEvent> = Vec::with_capacity(2);
    prelude.push(init);
    if existing_task_id.is_none() {
        // Pre-emit CreateTask so the UI has a stable task_id to attach
        // subsequent AppendToMessageContent events against.
        prelude.push(encode::build_create_task_action(task_id.clone()));
    }
    let prelude_stream = futures::stream::iter(
        prelude
            .into_iter()
            .map(Ok::<_, Arc<AIApiError>>),
    );

    let body_stream = futures::stream::once(async move {
        let chunks = match provider.kind {
            DirectProviderKind::Anthropic => {
                anthropic_driver::call_streaming(&provider, &turns, mcp_context.as_ref()).await
            }
            DirectProviderKind::OpenAi | DirectProviderKind::OpenAiCompatible => {
                openai_driver::call_streaming(&provider, &turns, mcp_context.as_ref()).await
            }
            DirectProviderKind::Gemini => {
                gemini_driver::call_streaming(&provider, &turns, mcp_context.as_ref()).await
            }
        };
        match chunks {
            Ok(stream) => {
                cfg_if::cfg_if! {
                    if #[cfg(target_family = "wasm")] {
                        adapter::adapt(stream, task_id, request_id).boxed_local()
                    } else {
                        adapter::adapt(stream, task_id, request_id).boxed()
                    }
                }
            }
            Err(e) => {
                log::error!("DirectBackend agent mode init error: {e:#}");
                let single = encode::build_finished_error(sanitize_error(&e));
                let s = futures::stream::iter(vec![Ok::<_, Arc<AIApiError>>(single)]);
                cfg_if::cfg_if! {
                    if #[cfg(target_family = "wasm")] {
                        s.boxed_local()
                    } else {
                        s.boxed()
                    }
                }
            }
        }
    })
    .flatten();

    let stream = prelude_stream.chain(body_stream);

    cfg_if::cfg_if! {
        if #[cfg(target_family = "wasm")] {
            stream.boxed_local()
        } else {
            stream.boxed()
        }
    }
}

/// Legacy stub kept for callers that haven't migrated to [`run`] yet.
#[allow(dead_code)]
pub fn stub_multi_agent_stream(request: &api::Request) -> AIOutputStream<api::ResponseEvent> {
    let conversation_id = request
        .metadata
        .as_ref()
        .map(|m| m.conversation_id.clone())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    error_stream(conversation_id, STUB_MESSAGE.into())
}

fn error_stream(conversation_id: String, message: String) -> AIOutputStream<api::ResponseEvent> {
    let (init, _request_id) = encode::build_init_event(conversation_id);
    let finished = encode::build_finished_error(message);
    let stream = futures::stream::iter(
        vec![init, finished]
            .into_iter()
            .map(Ok::<_, Arc<AIApiError>>),
    );
    cfg_if::cfg_if! {
        if #[cfg(target_family = "wasm")] {
            stream.boxed_local()
        } else {
            stream.boxed()
        }
    }
}

/// Telemetry sink for the legacy aggregator path (still used by `call(...)`
/// and tests). The streaming path emits `token_usage` directly via
/// `encode::build_finished_from_stop`; no log sink needed there.
#[allow(dead_code)]
fn log_driver_telemetry(out: &DriverOutput) {
    log::info!(
        "DirectBackend agent: stop_reason={:?} input_tokens={:?} output_tokens={:?} blocks={}",
        out.stop_reason,
        out.input_tokens,
        out.output_tokens,
        out.blocks.len()
    );
}

/// Convert an `anyhow::Error` into a user-safe message. The full chain (which
/// can include up to 1 KB of provider response body, prompt fragments, file
/// names…) goes to the log; the user sees only the high-level failure mode.
pub(super) fn sanitize_error(e: &anyhow::Error) -> String {
    let s = format!("{e}"); // top-level message only, no `{:#}` chain

    // Match against well-known patterns to give the user actionable text.
    let lower = s.to_ascii_lowercase();
    if lower.contains("transport error") || lower.contains("dns") || lower.contains("connection") {
        return "Direct LLM Agent Mode: network error reaching the provider. Check connectivity \
                and `WARP_CN_BASE_URL`."
            .into();
    }
    if lower.contains("http 401") || lower.contains("http 403") {
        return "Direct LLM Agent Mode: authentication rejected by the provider. Check your \
                `WARP_CN_API_KEY`."
            .into();
    }
    if lower.contains("http 429") {
        return "Direct LLM Agent Mode: rate-limited by the provider. Wait a moment or switch \
                provider."
            .into();
    }
    if lower.contains("http 5") {
        return "Direct LLM Agent Mode: provider server error. Try again shortly.".into();
    }
    if lower.contains("timed out") || lower.contains("timeout") {
        return "Direct LLM Agent Mode: provider call timed out (>120s). Retry or pick a faster \
                model."
            .into();
    }
    if lower.contains("parse error") || lower.contains("not the expected shape") {
        return "Direct LLM Agent Mode: model returned a malformed tool call. Retry, or rephrase \
                the request."
            .into();
    }
    if lower.contains("unsupported tool") {
        return "Direct LLM Agent Mode: model attempted a tool that's not yet wired in this \
                build."
            .into();
    }
    // Catch-all: surface the short top-level message but never the deeper chain.
    format!("Direct LLM Agent Mode failed: {s}")
}

fn resolve_provider() -> Option<ResolvedProvider> {
    // Priority 1: in-process snapshot of `DirectBackendConfig` (UI-driven).
    // The snapshot is published whenever the model persists, so changes
    // through the settings page take effect for the next agent turn without
    // requiring a restart or env-var hack.
    if let Some(p) = resolve_from_snapshot() {
        return Some(p);
    }
    // Priority 2: env-vars. Original M2-era escape hatch for headless
    // dev/QA flows that pre-date the settings UI.
    resolve_from_env()
}

fn resolve_from_snapshot() -> Option<ResolvedProvider> {
    let snap = ::ai::direct_backend::current_snapshot();
    // Auto-derive active provider: whichever has a non-empty `api_key` in
    // the override wins (same priority order as the single-call path).
    // The snapshot's `enabled` and `active` fields are intentionally ignored
    // — there's no UI toggle anymore; the presence of a key implies "use it".
    for kind in [
        DirectProviderKind::Anthropic,
        DirectProviderKind::OpenAi,
        DirectProviderKind::Gemini,
    ] {
        let overrides = match kind {
            DirectProviderKind::Anthropic => &snap.anthropic,
            DirectProviderKind::OpenAi => &snap.openai,
            DirectProviderKind::Gemini => &snap.gemini,
            DirectProviderKind::OpenAiCompatible => continue,
        };
        let api_key = overrides.api_key.trim();
        if api_key.is_empty() {
            continue;
        }
        let base_url = if overrides.base_url.trim().is_empty() {
            default_base_url(kind).to_string()
        } else {
            overrides.base_url.trim().trim_end_matches('/').to_string()
        };
        let model_id = if overrides.model_id.trim().is_empty() {
            default_model_id(kind).to_string()
        } else {
            overrides.model_id.trim().to_string()
        };
        return Some(ResolvedProvider {
            kind,
            api_key: api_key.to_string(),
            base_url,
            model_id,
        });
    }
    None
}

fn resolve_from_env() -> Option<ResolvedProvider> {
    let kind = match std::env::var("WARP_CN_DIRECT_PROVIDER")
        .ok()?
        .to_ascii_lowercase()
        .as_str()
    {
        "anthropic" => DirectProviderKind::Anthropic,
        "openai" => DirectProviderKind::OpenAi,
        "openai-compatible" | "compat" => DirectProviderKind::OpenAiCompatible,
        "gemini" => DirectProviderKind::Gemini,
        _ => return None,
    };
    let api_key = std::env::var("WARP_CN_API_KEY")
        .ok()
        .filter(|s| !s.is_empty())?;
    let base_url = std::env::var("WARP_CN_BASE_URL")
        .ok()
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_end_matches('/').to_string())
        .unwrap_or_else(|| default_base_url(kind).to_string());
    let model_id = std::env::var("WARP_CN_MODEL")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| default_model_id(kind).to_string());
    Some(ResolvedProvider {
        kind,
        api_key,
        base_url,
        model_id,
    })
}

fn default_base_url(kind: DirectProviderKind) -> &'static str {
    match kind {
        DirectProviderKind::Anthropic => "https://api.anthropic.com",
        DirectProviderKind::OpenAi | DirectProviderKind::OpenAiCompatible => {
            "https://api.openai.com"
        }
        DirectProviderKind::Gemini => "https://generativelanguage.googleapis.com",
    }
}

fn default_model_id(kind: DirectProviderKind) -> &'static str {
    match kind {
        DirectProviderKind::Anthropic => "claude-sonnet-4-6",
        DirectProviderKind::OpenAi | DirectProviderKind::OpenAiCompatible => "gpt-4o-mini",
        DirectProviderKind::Gemini => "gemini-2.5-flash",
    }
}
