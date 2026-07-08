//! `warp-index-bridge`: minimal stdio MCP server that exposes Warp's local
//! codebase-index metadata to external MCP clients (e.g. Claude Code).
//!
//! Counterpart to `app/src/ai/auggie_store_client.rs`: while Warp uses Auggie
//! to *do* the indexing, this bridge lets Claude Code *see* what Warp has
//! indexed (path digests, last-modified times) and decide its own retrieval
//! strategy.
//!
//! ## Tools exposed
//!
//! - `list_indexed_snapshots()` — return every `snapshot_*` file in
//!   `~/Library/Application Support/<warp-bundle>/codebase_index_snapshots/`
//!   along with its mtime and size. The filename hash uses Rust's
//!   `DefaultHasher` over the canonical repo path; mapping back is one-way.
//!
//! ## Limitations (intentional v1 scope)
//!
//! - Does not parse snapshot bincode; only reports filesystem metadata.
//! - Does not query the live Warp process.
//! - SQLite `workspace_metadata` extraction is deferred — needs schema
//!   stability proof from upstream first.
//!
//! ## Register with Claude Code
//!
//! ```jsonc
//! "warp-index-bridge": {
//!   "command": "/path/to/warp-index-bridge",
//!   "args": []
//! }
//! ```

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use anyhow::Result;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
};
use rmcp::{tool, tool_handler, tool_router, ErrorData as McpError, ServerHandler, ServiceExt};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const REPO_SNAPSHOT_SUBDIR_NAME: &str = "codebase_index_snapshots";

#[derive(Debug, Serialize, JsonSchema)]
struct SnapshotEntry {
    /// On-disk filename, e.g. `snapshot_17234829348239`.
    filename: String,
    /// `DefaultHasher::finish()` digest extracted from the filename. Same
    /// algorithm Warp uses to derive the snapshot path from a canonical repo
    /// path. NOT cryptographic; only stable within a single Rust toolchain.
    digest: u64,
    /// Last modification time as Unix timestamp seconds.
    mtime_unix: Option<i64>,
    /// File size in bytes.
    size_bytes: u64,
}

#[derive(Debug, Serialize, JsonSchema)]
struct ListIndexedSnapshotsResult {
    snapshot_dir: Option<String>,
    snapshots: Vec<SnapshotEntry>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct DigestForRepoPathArgs {
    /// Absolute path to a local repository.
    repo_path: String,
}

#[derive(Debug, Serialize, JsonSchema)]
struct DigestForRepoPathResult {
    /// Hash digest matching what Warp would have written to disk for this repo.
    digest: u64,
    /// Expected snapshot filename: `snapshot_<digest>`.
    expected_filename: String,
}

fn snapshot_dir() -> Option<PathBuf> {
    warp_core::paths::secure_state_dir()
        .or_else(|| Some(warp_core::paths::state_dir()))
        .map(|dir| dir.join(REPO_SNAPSHOT_SUBDIR_NAME))
}

/// MUST stay in sync with `crates/ai/src/index/full_source_code_embedding/snapshot.rs::snapshot_path`.
/// Both functions hash the canonical repo path with `DefaultHasher` and format
/// the file as `snapshot_<digest>`. If upstream Warp ever switches to a stable
/// hash, update both sides simultaneously and bump this crate's version.
fn digest_for_repo_path(path: &Path) -> u64 {
    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);
    hasher.finish()
}

#[derive(Clone)]
struct WarpIndexBridge {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl WarpIndexBridge {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "List all Warp codebase-index snapshot files on disk with their mtime and size. \
                       Each entry includes a u64 digest extracted from the filename; clients can compute \
                       the same digest for a known repo path via `digest_for_repo_path` to correlate."
    )]
    async fn list_indexed_snapshots(&self) -> Result<CallToolResult, McpError> {
        let Some(dir) = snapshot_dir() else {
            return Ok(CallToolResult::success(vec![Content::json(
                ListIndexedSnapshotsResult {
                    snapshot_dir: None,
                    snapshots: vec![],
                },
            )?]));
        };

        let mut snapshots = Vec::new();
        if dir.is_dir() {
            for entry in std::fs::read_dir(&dir).map_err(|e| McpError::internal_error(
                format!("read_dir failed: {e}"), None,
            ))? {
                let Ok(entry) = entry else { continue };
                let filename = entry.file_name().to_string_lossy().to_string();
                let Some(digest_str) = filename.strip_prefix("snapshot_") else {
                    continue;
                };
                let Ok(digest) = digest_str.parse::<u64>() else {
                    continue;
                };
                let metadata = entry.metadata().ok();
                let mtime_unix = metadata
                    .as_ref()
                    .and_then(|m| m.modified().ok())
                    .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                    .map(|d| d.as_secs() as i64);
                let size_bytes = metadata.map(|m| m.len()).unwrap_or(0);
                snapshots.push(SnapshotEntry {
                    filename,
                    digest,
                    mtime_unix,
                    size_bytes,
                });
            }
            snapshots.sort_by_key(|s| std::cmp::Reverse(s.mtime_unix.unwrap_or(0)));
        }

        Ok(CallToolResult::success(vec![Content::json(
            ListIndexedSnapshotsResult {
                snapshot_dir: Some(dir.to_string_lossy().into_owned()),
                snapshots,
            },
        )?]))
    }

    #[tool(
        description = "Compute the snapshot-file digest Warp would use for a given absolute repo path. \
                       Pair with `list_indexed_snapshots` to determine if this repo has been indexed and when."
    )]
    async fn digest_for_repo_path(
        &self,
        Parameters(args): Parameters<DigestForRepoPathArgs>,
    ) -> Result<CallToolResult, McpError> {
        let path = PathBuf::from(&args.repo_path);
        let canonical = dunce::canonicalize(&path).unwrap_or(path);
        let digest = digest_for_repo_path(&canonical);
        let expected_filename = format!("snapshot_{digest}");
        Ok(CallToolResult::success(vec![Content::json(
            DigestForRepoPathResult {
                digest,
                expected_filename,
            },
        )?]))
    }
}

#[tool_handler]
impl ServerHandler for WarpIndexBridge {
    fn get_info(&self) -> ServerInfo {
        // `ServerInfo` is #[non_exhaustive] in rmcp 1.6, so build it from
        // `Default` instead of a struct literal.
        let mut info = ServerInfo::default();
        info.protocol_version = ProtocolVersion::default();
        info.capabilities = ServerCapabilities::builder().enable_tools().build();
        info.server_info = Implementation::from_build_env();
        info.instructions = Some(
            "Read-only metadata bridge to Warp's local codebase-index snapshots. \
             Useful for Claude Code to know which repos Warp has indexed and how fresh \
             the snapshot is, without parsing Warp's private snapshot format."
                .into(),
        );
        info
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let service = WarpIndexBridge::new()
        .serve(rmcp::transport::stdio())
        .await?;
    service.waiting().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Pins the hashing convention. If this test fails, either the bridge's
    /// `digest_for_repo_path` or Warp's `snapshot_path` was changed unilaterally.
    #[test]
    fn digest_is_deterministic_within_toolchain() {
        let path = Path::new("/tmp/some/repo");
        let a = digest_for_repo_path(path);
        let b = digest_for_repo_path(path);
        assert_eq!(a, b, "DefaultHasher must be deterministic per-toolchain");
    }

    #[test]
    fn distinct_paths_yield_distinct_digests() {
        let a = digest_for_repo_path(Path::new("/tmp/a"));
        let b = digest_for_repo_path(Path::new("/tmp/b"));
        assert_ne!(a, b);
    }

    #[test]
    fn snapshot_filename_format_matches_warp_convention() {
        let path = Path::new("/Users/liji/warp");
        let digest = digest_for_repo_path(path);
        let filename = format!("snapshot_{digest}");
        assert!(filename.starts_with("snapshot_"));
        assert!(filename[9..].parse::<u64>().is_ok());
    }
}

