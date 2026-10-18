//! Tier-1 tool registry for Direct LLM Agent Mode.
//!
//! Each [`ToolKind`] entry knows its wire name, JSON schema, model-prompt
//! description, and round-trip mapping between the model's tool_use input/
//! output JSON and the protobuf `Tool::*` / `*Result` variants the Warp client
//! understands.
//!
//! Adding a new tool requires:
//! 1. A `ToolKind` variant + `name`/`description`/`schema` const block.
//! 2. A `parse_*` helper that takes `serde_json::Value` and returns the proto
//!    `Tool` variant.
//! 3. A `*_to_json` helper for history projection (proto → JSON).
//! 4. A `*_result_to_text` helper for tool-result chat-history rendering.
//! 5. Wire all of the above into the four dispatch functions at the bottom.

use anyhow::{anyhow, Context};
use serde::Deserialize;
use serde_json::{json, Value};
use warp_multi_agent_api as api;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolKind {
    ReadFiles,
    RunShellCommand,
    Grep,
    FileGlobV2,
    ApplyFileDiffs,
    AskUserQuestion,
    WriteToLongRunningShellCommand,
    ReadShellCommandOutput,
    TransferShellCommandControlToUser,
    ReadMcpResource,
    CallMcpTool,
}

pub fn all_tools() -> &'static [ToolKind] {
    &[
        ToolKind::ReadFiles,
        ToolKind::RunShellCommand,
        ToolKind::Grep,
        ToolKind::FileGlobV2,
        ToolKind::ApplyFileDiffs,
        ToolKind::AskUserQuestion,
        ToolKind::WriteToLongRunningShellCommand,
        ToolKind::ReadShellCommandOutput,
        ToolKind::TransferShellCommandControlToUser,
        ToolKind::ReadMcpResource,
        ToolKind::CallMcpTool,
    ]
}

/// Tools advertised to the LLM for this request. Identical to [`all_tools`]
/// except the MCP tools (`read_mcp_resource`, `call_mcp_tool`) are stripped
/// when no MCP servers are configured — there's nothing valid to call, and
/// always-on advertisement encourages models to fabricate `server_id` values.
pub fn advertised_tools(
    mcp: Option<&warp_multi_agent_api::request::McpContext>,
) -> Vec<ToolKind> {
    let has_mcp = mcp.is_some_and(|ctx| !ctx.servers.is_empty());
    all_tools()
        .iter()
        .copied()
        .filter(|kind| {
            has_mcp || !matches!(kind, ToolKind::ReadMcpResource | ToolKind::CallMcpTool)
        })
        .collect()
}

pub fn from_name(name: &str) -> Option<ToolKind> {
    match name {
        "read_files" => Some(ToolKind::ReadFiles),
        "run_shell_command" => Some(ToolKind::RunShellCommand),
        "grep" => Some(ToolKind::Grep),
        "file_glob" => Some(ToolKind::FileGlobV2),
        "apply_file_diffs" => Some(ToolKind::ApplyFileDiffs),
        "ask_user_question" => Some(ToolKind::AskUserQuestion),
        "write_to_long_running_shell_command" => Some(ToolKind::WriteToLongRunningShellCommand),
        "read_shell_command_output" => Some(ToolKind::ReadShellCommandOutput),
        "transfer_shell_command_control_to_user" => {
            Some(ToolKind::TransferShellCommandControlToUser)
        }
        "read_mcp_resource" => Some(ToolKind::ReadMcpResource),
        "call_mcp_tool" => Some(ToolKind::CallMcpTool),
        _ => None,
    }
}

pub fn name(kind: ToolKind) -> &'static str {
    match kind {
        ToolKind::ReadFiles => "read_files",
        ToolKind::RunShellCommand => "run_shell_command",
        ToolKind::Grep => "grep",
        ToolKind::FileGlobV2 => "file_glob",
        ToolKind::ApplyFileDiffs => "apply_file_diffs",
        ToolKind::AskUserQuestion => "ask_user_question",
        ToolKind::WriteToLongRunningShellCommand => "write_to_long_running_shell_command",
        ToolKind::ReadShellCommandOutput => "read_shell_command_output",
        ToolKind::TransferShellCommandControlToUser => "transfer_shell_command_control_to_user",
        ToolKind::ReadMcpResource => "read_mcp_resource",
        ToolKind::CallMcpTool => "call_mcp_tool",
    }
}

pub fn description(kind: ToolKind) -> &'static str {
    match kind {
        ToolKind::ReadFiles => READ_FILES_DESC,
        ToolKind::RunShellCommand => RUN_SHELL_DESC,
        ToolKind::Grep => GREP_DESC,
        ToolKind::FileGlobV2 => FILE_GLOB_DESC,
        ToolKind::ApplyFileDiffs => APPLY_FILE_DIFFS_DESC,
        ToolKind::AskUserQuestion => ASK_USER_DESC,
        ToolKind::WriteToLongRunningShellCommand => WRITE_LRC_DESC,
        ToolKind::ReadShellCommandOutput => READ_SHELL_OUTPUT_DESC,
        ToolKind::TransferShellCommandControlToUser => TRANSFER_CONTROL_DESC,
        ToolKind::ReadMcpResource => READ_MCP_RESOURCE_DESC,
        ToolKind::CallMcpTool => CALL_MCP_TOOL_DESC,
    }
}

pub fn schema(kind: ToolKind) -> &'static str {
    match kind {
        ToolKind::ReadFiles => READ_FILES_SCHEMA,
        ToolKind::RunShellCommand => RUN_SHELL_SCHEMA,
        ToolKind::Grep => GREP_SCHEMA,
        ToolKind::FileGlobV2 => FILE_GLOB_SCHEMA,
        ToolKind::ApplyFileDiffs => APPLY_FILE_DIFFS_SCHEMA,
        ToolKind::AskUserQuestion => ASK_USER_SCHEMA,
        ToolKind::WriteToLongRunningShellCommand => WRITE_LRC_SCHEMA,
        ToolKind::ReadShellCommandOutput => READ_SHELL_OUTPUT_SCHEMA,
        ToolKind::TransferShellCommandControlToUser => TRANSFER_CONTROL_SCHEMA,
        ToolKind::ReadMcpResource => READ_MCP_RESOURCE_SCHEMA,
        ToolKind::CallMcpTool => CALL_MCP_TOOL_SCHEMA,
    }
}

/// Parse the model's `tool_use.input` JSON into the proto `Tool` variant.
pub fn parse_input(kind: ToolKind, value: Value) -> anyhow::Result<api::message::tool_call::Tool> {
    Ok(match kind {
        ToolKind::ReadFiles => api::message::tool_call::Tool::ReadFiles(parse_read_files(value)?),
        ToolKind::RunShellCommand => {
            api::message::tool_call::Tool::RunShellCommand(parse_run_shell_command(value)?)
        }
        ToolKind::Grep => api::message::tool_call::Tool::Grep(parse_grep(value)?),
        ToolKind::FileGlobV2 => api::message::tool_call::Tool::FileGlobV2(parse_file_glob_v2(value)?),
        ToolKind::ApplyFileDiffs => {
            api::message::tool_call::Tool::ApplyFileDiffs(parse_apply_file_diffs(value)?)
        }
        ToolKind::AskUserQuestion => {
            api::message::tool_call::Tool::AskUserQuestion(parse_ask_user_question(value)?)
        }
        ToolKind::WriteToLongRunningShellCommand => {
            api::message::tool_call::Tool::WriteToLongRunningShellCommand(parse_write_to_lrc(
                value,
            )?)
        }
        ToolKind::ReadShellCommandOutput => api::message::tool_call::Tool::ReadShellCommandOutput(
            parse_read_shell_command_output(value)?,
        ),
        ToolKind::TransferShellCommandControlToUser => {
            api::message::tool_call::Tool::TransferShellCommandControlToUser(parse_transfer_control(
                value,
            )?)
        }
        ToolKind::ReadMcpResource => {
            api::message::tool_call::Tool::ReadMcpResource(parse_read_mcp_resource(value)?)
        }
        ToolKind::CallMcpTool => api::message::tool_call::Tool::CallMcpTool(parse_call_mcp_tool(value)?),
    })
}

/// Project a proto `Tool` back to (kind, JSON) for chat history.
pub fn proto_to_history(
    tool: &api::message::tool_call::Tool,
) -> Option<(ToolKind, Value)> {
    use api::message::tool_call::Tool as T;
    match tool {
        T::ReadFiles(rf) => Some((ToolKind::ReadFiles, read_files_to_json(rf))),
        T::RunShellCommand(r) => Some((ToolKind::RunShellCommand, run_shell_to_json(r))),
        T::Grep(g) => Some((ToolKind::Grep, grep_to_json(g))),
        T::FileGlobV2(f) => Some((ToolKind::FileGlobV2, file_glob_v2_to_json(f))),
        T::ApplyFileDiffs(a) => Some((ToolKind::ApplyFileDiffs, apply_file_diffs_to_json(a))),
        T::AskUserQuestion(q) => Some((ToolKind::AskUserQuestion, ask_user_question_to_json(q))),
        T::WriteToLongRunningShellCommand(w) => Some((
            ToolKind::WriteToLongRunningShellCommand,
            write_to_lrc_to_json(w),
        )),
        T::ReadShellCommandOutput(r) => Some((
            ToolKind::ReadShellCommandOutput,
            read_shell_command_output_to_json(r),
        )),
        T::TransferShellCommandControlToUser(t) => Some((
            ToolKind::TransferShellCommandControlToUser,
            transfer_control_to_json(t),
        )),
        T::ReadMcpResource(r) => Some((ToolKind::ReadMcpResource, read_mcp_resource_to_json(r))),
        T::CallMcpTool(c) => Some((ToolKind::CallMcpTool, call_mcp_tool_to_json(c))),
        _ => None,
    }
}

/// Render a server-side ToolCallResult into chat-history text.
pub fn message_result_to_text(
    result: &api::message::tool_call_result::Result,
) -> Option<(ToolKind, String)> {
    use api::message::tool_call_result::Result as R;
    match result {
        R::ReadFiles(r) => Some((ToolKind::ReadFiles, read_files_result_to_text(r))),
        R::RunShellCommand(r) => Some((ToolKind::RunShellCommand, run_shell_result_to_text(r))),
        R::Grep(r) => Some((ToolKind::Grep, grep_result_to_text(r))),
        R::FileGlobV2(r) => Some((ToolKind::FileGlobV2, file_glob_v2_result_to_text(r))),
        R::ApplyFileDiffs(r) => Some((ToolKind::ApplyFileDiffs, apply_file_diffs_result_to_text(r))),
        R::AskUserQuestion(r) => Some((ToolKind::AskUserQuestion, ask_user_result_to_text(r))),
        R::WriteToLongRunningShellCommand(r) => Some((
            ToolKind::WriteToLongRunningShellCommand,
            write_to_lrc_result_to_text(r),
        )),
        R::ReadShellCommandOutput(r) => Some((
            ToolKind::ReadShellCommandOutput,
            read_shell_command_output_result_to_text(r),
        )),
        R::TransferShellCommandControlToUser(r) => Some((
            ToolKind::TransferShellCommandControlToUser,
            transfer_control_result_to_text(r),
        )),
        R::ReadMcpResource(r) => Some((
            ToolKind::ReadMcpResource,
            read_mcp_resource_result_to_text(r),
        )),
        R::CallMcpTool(r) => Some((ToolKind::CallMcpTool, call_mcp_tool_result_to_text(r))),
        _ => None,
    }
}

/// Same as [`message_result_to_text`] but for the client → server direction
/// (`request::input::tool_call_result::Result`); shares all inner result types.
pub fn request_result_to_text(
    result: &api::request::input::tool_call_result::Result,
) -> Option<(ToolKind, String)> {
    use api::request::input::tool_call_result::Result as R;
    match result {
        R::ReadFiles(r) => Some((ToolKind::ReadFiles, read_files_result_to_text(r))),
        R::RunShellCommand(r) => Some((ToolKind::RunShellCommand, run_shell_result_to_text(r))),
        R::Grep(r) => Some((ToolKind::Grep, grep_result_to_text(r))),
        R::FileGlobV2(r) => Some((ToolKind::FileGlobV2, file_glob_v2_result_to_text(r))),
        R::ApplyFileDiffs(r) => Some((ToolKind::ApplyFileDiffs, apply_file_diffs_result_to_text(r))),
        R::AskUserQuestion(r) => Some((ToolKind::AskUserQuestion, ask_user_result_to_text(r))),
        R::WriteToLongRunningShellCommand(r) => Some((
            ToolKind::WriteToLongRunningShellCommand,
            write_to_lrc_result_to_text(r),
        )),
        R::ReadShellCommandOutput(r) => Some((
            ToolKind::ReadShellCommandOutput,
            read_shell_command_output_result_to_text(r),
        )),
        R::TransferShellCommandControlToUser(r) => Some((
            ToolKind::TransferShellCommandControlToUser,
            transfer_control_result_to_text(r),
        )),
        R::ReadMcpResource(r) => Some((
            ToolKind::ReadMcpResource,
            read_mcp_resource_result_to_text(r),
        )),
        R::CallMcpTool(r) => Some((ToolKind::CallMcpTool, call_mcp_tool_result_to_text(r))),
        _ => None,
    }
}

// ── ReadFiles ────────────────────────────────────────────────────────────

const READ_FILES_DESC: &str =
    "Read text from one or more files in the user's workspace. \
     Use this when you need to inspect file contents to answer the user. \
     Provide either an absolute path or a path relative to the workspace root. \
     Optionally restrict to specific 1-indexed line ranges; omit `lines` to read \
     the whole file.";

const READ_FILES_SCHEMA: &str = r#"{
  "type": "object",
  "properties": {
    "files": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "object",
        "properties": {
          "path": {"type": "string"},
          "lines": {
            "type": "array",
            "items": {
              "type": "object",
              "properties": {
                "start": {"type": "integer", "minimum": 1},
                "end":   {"type": "integer", "minimum": 1}
              },
              "required": ["start", "end"],
              "additionalProperties": false
            }
          }
        },
        "required": ["path"],
        "additionalProperties": false
      }
    }
  },
  "required": ["files"],
  "additionalProperties": false
}"#;

#[derive(Deserialize)]
struct ReadFilesInput {
    files: Vec<ReadFilesFile>,
}
#[derive(Deserialize)]
struct ReadFilesFile {
    path: String,
    #[serde(default)]
    lines: Vec<LineRangeInput>,
}
#[derive(Deserialize)]
struct LineRangeInput {
    start: u32,
    end: u32,
}

fn parse_read_files(value: Value) -> anyhow::Result<api::message::tool_call::ReadFiles> {
    let parsed: ReadFilesInput =
        serde_json::from_value(value).context("read_files input is not the expected shape")?;
    if parsed.files.is_empty() {
        return Err(anyhow!("read_files requires at least one file entry"));
    }
    let files = parsed
        .files
        .into_iter()
        .map(|f| {
            let path = f.path.trim().to_string();
            if path.is_empty() {
                return Err(anyhow!("read_files file entry had empty path"));
            }
            let line_ranges = f
                .lines
                .into_iter()
                .map(|r| {
                    if r.start == 0 || r.end == 0 || r.start > r.end {
                        Err(anyhow!(
                            "read_files line range invalid (start={}, end={})",
                            r.start,
                            r.end
                        ))
                    } else {
                        Ok(api::FileContentLineRange {
                            start: r.start,
                            end: r.end,
                        })
                    }
                })
                .collect::<anyhow::Result<Vec<_>>>()?;
            Ok(api::message::tool_call::read_files::File {
                name: path,
                line_ranges,
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(api::message::tool_call::ReadFiles { files })
}

fn read_files_to_json(rf: &api::message::tool_call::ReadFiles) -> Value {
    let files: Vec<Value> = rf
        .files
        .iter()
        .map(|f| {
            let lines: Vec<Value> = f
                .line_ranges
                .iter()
                .map(|r| json!({"start": r.start, "end": r.end}))
                .collect();
            if lines.is_empty() {
                json!({"path": f.name})
            } else {
                json!({"path": f.name, "lines": lines})
            }
        })
        .collect();
    json!({"files": files})
}

fn read_files_result_to_text(rf: &api::ReadFilesResult) -> String {
    use api::read_files_result::Result as R;
    match rf.result.as_ref() {
        Some(R::TextFilesSuccess(s)) => format_file_contents(s.files.iter()),
        Some(R::AnyFilesSuccess(s)) => {
            let mut parts = Vec::with_capacity(s.files.len());
            for any in &s.files {
                match any.content.as_ref() {
                    Some(api::any_file_content::Content::TextContent(t)) => {
                        parts.push(format_one_file(&t.file_path, &t.content, t.line_range.as_ref()));
                    }
                    Some(api::any_file_content::Content::BinaryContent(b)) => {
                        parts.push(format!("<binary file: {}>", b.file_path));
                    }
                    None => {}
                }
            }
            parts.join("\n\n")
        }
        Some(R::Error(e)) => format!("ERROR: {}", e.message),
        None => String::from("ERROR: empty read_files result"),
    }
}

fn format_file_contents<'a>(files: impl Iterator<Item = &'a api::FileContent>) -> String {
    files
        .map(|f| format_one_file(&f.file_path, &f.content, f.line_range.as_ref()))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn format_one_file(path: &str, content: &str, line_range: Option<&api::FileContentLineRange>) -> String {
    match line_range {
        Some(r) => format!("===== {} (lines {}..{}) =====\n{}", path, r.start, r.end, content),
        None => format!("===== {} =====\n{}", path, content),
    }
}

// ── RunShellCommand ──────────────────────────────────────────────────────

const RUN_SHELL_DESC: &str =
    "Run a shell command on the user's machine and return its output. \
     Use this when the user asks to inspect system state (ls, grep, cargo …), \
     run tests, build, or otherwise execute commands. \
     \
     YOU MUST set `risk_category` for EVERY call. The client auto-runs \
     `read_only` commands without prompting and asks the user for everything \
     else — omitting or under-classifying turns every safe inspection into \
     a permission popup, which the user will reject. \
     \
     Use `read_only` for ANY command that only reads (pwd, ls, cat, head, \
     tail, less, file, stat, wc, find, grep, awk, sed-without-`-i`, sort, \
     uniq, tree, cargo metadata, cargo tree, rustup show, git status, git \
     log, git diff, git show, git blame, git remote -v, gh pr view, curl \
     GET without -d/-X POST, and any pipeline composed solely of these). \
     Default to `read_only` whenever in doubt about an inspection command. \
     \
     Use `trivial_local_change` for safe local mutations (mkdir, touch, mv \
     inside cwd, chmod). \
     Use `nontrivial_local_change` for build/install (cargo build, npm \
     install, pip install, editing source files). \
     Use `external_change` for anything affecting other systems (git push, \
     curl POST/PUT/DELETE, ssh remote, gh pr create, deploy scripts). \
     Use `risky` for irreversible / privileged actions (rm -rf, sudo, dd, \
     mkfs, kill -9 critical pids).";

const RUN_SHELL_SCHEMA: &str = r#"{
  "type": "object",
  "properties": {
    "command": {"type": "string", "description": "The full command line to execute."},
    "risk_category": {
      "type": "string",
      "enum": ["read_only", "trivial_local_change", "nontrivial_local_change", "external_change", "risky"],
      "description": "Pick the most accurate category so the client can decide whether to auto-run or ask."
    }
  },
  "required": ["command", "risk_category"],
  "additionalProperties": false
}"#;

#[derive(Deserialize)]
struct RunShellInput {
    command: String,
    /// New canonical field. Required by the schema, but accept absence
    /// gracefully so older models that learned the previous schema don't
    /// hard-fail; default to `read_only` to match the historical default
    /// behavior of `is_read_only: false → ask user`.
    #[serde(default)]
    risk_category: Option<String>,
    /// Legacy field kept for back-compat. Ignored when `risk_category` is set.
    #[serde(default)]
    is_read_only: bool,
}

fn parse_run_shell_command(value: Value) -> anyhow::Result<api::message::tool_call::RunShellCommand> {
    let parsed: RunShellInput =
        serde_json::from_value(value).context("run_shell_command input shape mismatch")?;
    let cmd = parsed.command.trim().to_string();
    if cmd.is_empty() {
        return Err(anyhow!("run_shell_command requires a non-empty command"));
    }
    let risk = match parsed.risk_category.as_deref() {
        Some("read_only") => api::RiskCategory::ReadOnly,
        Some("trivial_local_change") => api::RiskCategory::TrivialLocalChange,
        Some("nontrivial_local_change") => api::RiskCategory::NontrivialLocalChange,
        Some("external_change") => api::RiskCategory::ExternalChange,
        Some("risky") => api::RiskCategory::Risky,
        // Fall back to the legacy bool only when the new field is missing.
        None if parsed.is_read_only => api::RiskCategory::ReadOnly,
        None => api::RiskCategory::Unspecified,
        Some(other) => {
            return Err(anyhow!("unknown risk_category `{other}`"));
        }
    };
    Ok(api::message::tool_call::RunShellCommand {
        command: cmd,
        #[allow(deprecated)]
        is_read_only: matches!(risk, api::RiskCategory::ReadOnly),
        risk_category: risk as i32,
        ..Default::default()
    })
}

fn run_shell_to_json(r: &api::message::tool_call::RunShellCommand) -> Value {
    let risk = match api::RiskCategory::try_from(r.risk_category)
        .unwrap_or(api::RiskCategory::Unspecified)
    {
        api::RiskCategory::ReadOnly => "read_only",
        api::RiskCategory::TrivialLocalChange => "trivial_local_change",
        api::RiskCategory::NontrivialLocalChange => "nontrivial_local_change",
        api::RiskCategory::ExternalChange => "external_change",
        api::RiskCategory::Risky => "risky",
        api::RiskCategory::Unspecified => "read_only",
    };
    json!({"command": r.command, "risk_category": risk})
}

fn run_shell_result_to_text(r: &api::RunShellCommandResult) -> String {
    use api::run_shell_command_result::Result as R;
    let actual_cmd = if r.command.is_empty() { None } else { Some(r.command.as_str()) };
    let preface = match actual_cmd {
        Some(c) => format!("Ran: `{c}`"),
        None => String::from("Ran: (command unavailable)"),
    };
    match r.result.as_ref() {
        Some(R::CommandFinished(f)) => format!(
            "{preface}\nExit code: {}\n----- output -----\n{}",
            f.exit_code, f.output
        ),
        Some(R::LongRunningCommandSnapshot(s)) => format!(
            "{preface}\n[long-running, snapshot only]\n----- output -----\n{}",
            s.output
        ),
        Some(R::PermissionDenied(_)) => format!("{preface}\n[user denied permission]"),
        Some(R::TerminalBusy(b)) => format!(
            "{preface}\n[terminal busy with running command `{}`]",
            b.running_command_id
        ),
        None => format!("{preface}\n[no result body]"),
    }
}

// ── Grep ─────────────────────────────────────────────────────────────────

const GREP_DESC: &str =
    "Search file contents for one or more patterns. Each pattern is treated as \
     a regular expression. The result is a list of (file_path, line_number) \
     pairs the user can navigate. Use this for codebase searches before reading \
     individual files.";

const GREP_SCHEMA: &str = r#"{
  "type": "object",
  "properties": {
    "queries": {
      "type": "array",
      "minItems": 1,
      "items": {"type": "string"},
      "description": "Regex patterns to search for."
    },
    "path": {"type": "string", "description": "Workspace-relative directory or file to search in."}
  },
  "required": ["queries", "path"],
  "additionalProperties": false
}"#;

#[derive(Deserialize)]
struct GrepInput {
    queries: Vec<String>,
    path: String,
}

fn parse_grep(value: Value) -> anyhow::Result<api::message::tool_call::Grep> {
    let parsed: GrepInput =
        serde_json::from_value(value).context("grep input shape mismatch")?;
    if parsed.queries.is_empty() {
        return Err(anyhow!("grep requires at least one query"));
    }
    let queries: Vec<String> = parsed
        .queries
        .into_iter()
        .map(|q| q.trim().to_string())
        .filter(|q| !q.is_empty())
        .collect();
    if queries.is_empty() {
        return Err(anyhow!(
            "grep requires at least one non-empty query (empty regex matches every line)"
        ));
    }
    let path = parsed.path.trim().to_string();
    if path.is_empty() {
        return Err(anyhow!("grep requires a non-empty path"));
    }
    Ok(api::message::tool_call::Grep { queries, path })
}

fn grep_to_json(g: &api::message::tool_call::Grep) -> Value {
    json!({"queries": g.queries, "path": g.path})
}

fn grep_result_to_text(r: &api::GrepResult) -> String {
    use api::grep_result::Result as R;
    match r.result.as_ref() {
        Some(R::Success(s)) => {
            if s.matched_files.is_empty() {
                return String::from("(no matches)");
            }
            let mut out = String::new();
            for f in &s.matched_files {
                let lines: Vec<String> = f
                    .matched_lines
                    .iter()
                    .map(|m| m.line_number.to_string())
                    .collect();
                out.push_str(&format!("{}: lines {}\n", f.file_path, lines.join(",")));
            }
            out
        }
        Some(R::Error(e)) => format!("ERROR: {}", e.message),
        None => String::from("ERROR: empty grep result"),
    }
}

// ── FileGlobV2 ───────────────────────────────────────────────────────────

const FILE_GLOB_DESC: &str =
    "Find files in the workspace whose names match one or more glob patterns \
     (`*`, `?`, `[..]`). Use this to locate files when you don't know exact \
     paths. Returns a list of file paths. The client enforces sensible \
     defaults; explicit caps you might supply are advisory.";

// `max_matches` / `max_depth` are intentionally NOT advertised to the model
// because the client-side executor (`crates/ai/src/agent/action/convert.rs`)
// drops them when constructing `AIAgentActionType::FileGlobV2`. Telling the
// model it can cap an expensive search would be misleading.
const FILE_GLOB_SCHEMA: &str = r#"{
  "type": "object",
  "properties": {
    "patterns": {
      "type": "array",
      "minItems": 1,
      "items": {"type": "string"},
      "description": "Glob patterns (e.g. `**/*.rs`)."
    },
    "search_dir": {"type": "string", "description": "Workspace-relative directory to search in."}
  },
  "required": ["patterns"],
  "additionalProperties": false
}"#;

#[derive(Deserialize)]
struct FileGlobInput {
    patterns: Vec<String>,
    #[serde(default)]
    search_dir: String,
    #[serde(default)]
    max_matches: i32,
    #[serde(default)]
    max_depth: i32,
}

fn parse_file_glob_v2(value: Value) -> anyhow::Result<api::message::tool_call::FileGlobV2> {
    let parsed: FileGlobInput =
        serde_json::from_value(value).context("file_glob input shape mismatch")?;
    if parsed.patterns.is_empty() {
        return Err(anyhow!("file_glob requires at least one pattern"));
    }
    Ok(api::message::tool_call::FileGlobV2 {
        patterns: parsed.patterns,
        search_dir: parsed.search_dir,
        max_matches: parsed.max_matches,
        max_depth: parsed.max_depth,
        ..Default::default()
    })
}

fn file_glob_v2_to_json(f: &api::message::tool_call::FileGlobV2) -> Value {
    // Mirror schema: only the fields we advertise. max_matches / max_depth
    // are dropped here too so history projections stay in sync.
    json!({
        "patterns": f.patterns,
        "search_dir": f.search_dir,
    })
}

fn file_glob_v2_result_to_text(r: &api::FileGlobV2Result) -> String {
    use api::file_glob_v2_result::Result as R;
    match r.result.as_ref() {
        Some(R::Success(s)) => {
            if s.matched_files.is_empty() {
                return String::from("(no matches)");
            }
            let mut paths = String::new();
            for m in &s.matched_files {
                paths.push_str(&m.file_path);
                paths.push('\n');
            }
            if !s.warnings.is_empty() {
                paths.push_str("\n[warnings]\n");
                paths.push_str(&s.warnings);
            }
            paths
        }
        Some(R::Error(e)) => format!("ERROR: {}", e.message),
        None => String::from("ERROR: empty file_glob result"),
    }
}

// ── ApplyFileDiffs ───────────────────────────────────────────────────────

const APPLY_FILE_DIFFS_DESC: &str =
    "Apply targeted edits to one or more files in the workspace. Edits are \
     either search-and-replace string substitutions, new files (full content), \
     or deletions. Use this to implement code changes the user requested. \
     Always include a short `summary` and ensure `search` matches existing \
     content exactly when editing.";

const APPLY_FILE_DIFFS_SCHEMA: &str = r#"{
  "type": "object",
  "properties": {
    "summary": {"type": "string", "description": "One-line description of the change."},
    "diffs": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "file_path": {"type": "string"},
          "search":    {"type": "string", "description": "Exact existing text to replace."},
          "replace":   {"type": "string", "description": "Replacement text."}
        },
        "required": ["file_path", "search", "replace"],
        "additionalProperties": false
      }
    },
    "new_files": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "file_path": {"type": "string"},
          "content":   {"type": "string"}
        },
        "required": ["file_path", "content"],
        "additionalProperties": false
      }
    },
    "deleted_files": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "file_path": {"type": "string"}
        },
        "required": ["file_path"],
        "additionalProperties": false
      }
    }
  },
  "required": ["summary"],
  "additionalProperties": false
}"#;

#[derive(Deserialize)]
struct ApplyFileDiffsInput {
    summary: String,
    #[serde(default)]
    diffs: Vec<DiffInput>,
    #[serde(default)]
    new_files: Vec<NewFileInput>,
    #[serde(default)]
    deleted_files: Vec<DeleteFileInput>,
}
#[derive(Deserialize)]
struct DiffInput {
    file_path: String,
    search: String,
    replace: String,
}
#[derive(Deserialize)]
struct NewFileInput {
    file_path: String,
    content: String,
}
#[derive(Deserialize)]
struct DeleteFileInput {
    file_path: String,
}

fn parse_apply_file_diffs(
    value: Value,
) -> anyhow::Result<api::message::tool_call::ApplyFileDiffs> {
    let parsed: ApplyFileDiffsInput = serde_json::from_value(value)
        .context("apply_file_diffs input shape mismatch")?;
    if parsed.diffs.is_empty() && parsed.new_files.is_empty() && parsed.deleted_files.is_empty() {
        return Err(anyhow!(
            "apply_file_diffs needs at least one of diffs / new_files / deleted_files"
        ));
    }
    let summary = parsed.summary.trim().to_string();
    if summary.is_empty() {
        return Err(anyhow!("apply_file_diffs summary may not be empty"));
    }
    let diffs = parsed
        .diffs
        .into_iter()
        .map(|d| {
            let fp = d.file_path.trim().to_string();
            if fp.is_empty() {
                return Err(anyhow!("apply_file_diffs.diffs[].file_path may not be empty"));
            }
            if d.search.is_empty() {
                return Err(anyhow!(
                    "apply_file_diffs.diffs[].search may not be empty (would match nothing or everything depending on engine)"
                ));
            }
            Ok(api::message::tool_call::apply_file_diffs::FileDiff {
                file_path: fp,
                search: d.search,
                replace: d.replace,
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    let new_files = parsed
        .new_files
        .into_iter()
        .map(|n| {
            let fp = n.file_path.trim().to_string();
            if fp.is_empty() {
                return Err(anyhow!(
                    "apply_file_diffs.new_files[].file_path may not be empty"
                ));
            }
            Ok(api::message::tool_call::apply_file_diffs::NewFile {
                file_path: fp,
                content: n.content,
                allow_overwrite: false,
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    let deleted_files = parsed
        .deleted_files
        .into_iter()
        .map(|d| {
            let fp = d.file_path.trim().to_string();
            if fp.is_empty() {
                return Err(anyhow!(
                    "apply_file_diffs.deleted_files[].file_path may not be empty"
                ));
            }
            Ok(api::message::tool_call::apply_file_diffs::DeleteFile { file_path: fp })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(api::message::tool_call::ApplyFileDiffs {
        summary,
        diffs,
        new_files,
        deleted_files,
        v4a_updates: vec![],
    })
}

fn apply_file_diffs_to_json(a: &api::message::tool_call::ApplyFileDiffs) -> Value {
    let diffs: Vec<Value> = a
        .diffs
        .iter()
        .map(|d| {
            json!({
                "file_path": d.file_path,
                "search": d.search,
                "replace": d.replace,
            })
        })
        .collect();
    let new_files: Vec<Value> = a
        .new_files
        .iter()
        .map(|n| json!({"file_path": n.file_path, "content": n.content}))
        .collect();
    let deleted_files: Vec<Value> = a
        .deleted_files
        .iter()
        .map(|d| json!({"file_path": d.file_path}))
        .collect();
    json!({
        "summary": a.summary,
        "diffs": diffs,
        "new_files": new_files,
        "deleted_files": deleted_files,
    })
}

fn apply_file_diffs_result_to_text(r: &api::ApplyFileDiffsResult) -> String {
    use api::apply_file_diffs_result::Result as R;
    match r.result.as_ref() {
        Some(R::Success(s)) => {
            let mut out = String::new();
            #[allow(deprecated)]
            let v2 = &s.updated_files_v2;
            for u in v2 {
                if let Some(file) = u.file.as_ref() {
                    let edited_marker = if u.was_edited_by_user { " (edited by user)" } else { "" };
                    out.push_str(&format!(
                        "Updated {}{}: {} bytes\n",
                        file.file_path,
                        edited_marker,
                        file.content.len()
                    ));
                }
            }
            for d in &s.deleted_files {
                out.push_str(&format!("Deleted {}\n", d.file_path));
            }
            if out.is_empty() {
                String::from("(no files affected)")
            } else {
                out
            }
        }
        Some(R::Error(e)) => format!("ERROR: {}", e.message),
        None => String::from("ERROR: empty apply_file_diffs result"),
    }
}

// ── AskUserQuestion ──────────────────────────────────────────────────────

const ASK_USER_DESC: &str =
    "Pose one or more multiple-choice questions to the user when the next step \
     genuinely requires their input (e.g. choosing between two implementation \
     paths, confirming a destructive action, picking a name). Each question \
     must include `question_id`, `question` text, and `options` (1-4 labels). \
     Avoid this tool for routine confirmations the user can answer in chat.";

const ASK_USER_SCHEMA: &str = r#"{
  "type": "object",
  "properties": {
    "questions": {
      "type": "array",
      "minItems": 1,
      "maxItems": 4,
      "items": {
        "type": "object",
        "properties": {
          "question_id": {"type": "string", "description": "Stable id used to correlate the answer."},
          "question":    {"type": "string"},
          "options": {
            "type": "array",
            "minItems": 1,
            "maxItems": 4,
            "items": {"type": "string"}
          },
          "recommended_option_index": {"type": "integer", "minimum": 0, "maximum": 3},
          "is_multiselect": {"type": "boolean"},
          "supports_other": {"type": "boolean"}
        },
        "required": ["question_id", "question", "options"],
        "additionalProperties": false
      }
    }
  },
  "required": ["questions"],
  "additionalProperties": false
}"#;

#[derive(Deserialize)]
struct AskUserInput {
    questions: Vec<AskUserQuestionItem>,
}
#[derive(Deserialize)]
struct AskUserQuestionItem {
    question_id: String,
    question: String,
    options: Vec<String>,
    #[serde(default)]
    recommended_option_index: i32,
    #[serde(default)]
    is_multiselect: bool,
    #[serde(default)]
    supports_other: bool,
}

fn parse_ask_user_question(value: Value) -> anyhow::Result<api::AskUserQuestion> {
    let parsed: AskUserInput =
        serde_json::from_value(value).context("ask_user_question input shape mismatch")?;
    if parsed.questions.is_empty() {
        return Err(anyhow!("ask_user_question requires at least one question"));
    }
    if parsed.questions.len() > 4 {
        return Err(anyhow!(
            "ask_user_question allows at most 4 questions (got {})",
            parsed.questions.len()
        ));
    }
    let questions = parsed
        .questions
        .into_iter()
        .map(|q| {
            let qid = q.question_id.trim().to_string();
            let qtext = q.question.trim().to_string();
            if qid.is_empty() {
                return Err(anyhow!("ask_user_question question_id may not be empty"));
            }
            if qtext.is_empty() {
                return Err(anyhow!("ask_user_question question text may not be empty"));
            }
            if q.options.is_empty() {
                return Err(anyhow!(
                    "ask_user_question requires at least one option per question"
                ));
            }
            if q.options.len() > 4 {
                return Err(anyhow!(
                    "ask_user_question allows at most 4 options (got {})",
                    q.options.len()
                ));
            }
            let options: Vec<_> = q
                .options
                .into_iter()
                .map(|label| api::ask_user_question::Option {
                    label: label.trim().to_string(),
                })
                .collect();
            if options.iter().any(|o| o.label.is_empty()) {
                return Err(anyhow!("ask_user_question option labels may not be empty"));
            }
            let recommended = q.recommended_option_index;
            if recommended < 0 || (recommended as usize) >= options.len() {
                return Err(anyhow!(
                    "ask_user_question recommended_option_index {} out of range [0,{})",
                    recommended,
                    options.len()
                ));
            }
            Ok(api::ask_user_question::Question {
                question_id: qid,
                question: qtext,
                question_type: Some(
                    api::ask_user_question::question::QuestionType::MultipleChoice(
                        api::ask_user_question::MultipleChoice {
                            options,
                            recommended_option_index: recommended,
                            is_multiselect: q.is_multiselect,
                            supports_other: q.supports_other,
                        },
                    ),
                ),
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(api::AskUserQuestion { questions })
}

fn ask_user_question_to_json(q: &api::AskUserQuestion) -> Value {
    let questions: Vec<Value> = q
        .questions
        .iter()
        .map(|q| {
            let mc = q.question_type.as_ref().and_then(|t| match t {
                api::ask_user_question::question::QuestionType::MultipleChoice(mc) => Some(mc),
            });
            match mc {
                Some(mc) => {
                    let options: Vec<&str> =
                        mc.options.iter().map(|o| o.label.as_str()).collect();
                    json!({
                        "question_id": q.question_id,
                        "question": q.question,
                        "options": options,
                        "recommended_option_index": mc.recommended_option_index,
                        "is_multiselect": mc.is_multiselect,
                        "supports_other": mc.supports_other,
                    })
                }
                None => json!({
                    "question_id": q.question_id,
                    "question": q.question,
                    "options": Vec::<&str>::new(),
                }),
            }
        })
        .collect();
    json!({"questions": questions})
}

fn ask_user_result_to_text(r: &api::AskUserQuestionResult) -> String {
    use api::ask_user_question_result::Result as R;
    match r.result.as_ref() {
        Some(R::Success(s)) => {
            let mut out = String::new();
            for ans in &s.answers {
                use api::ask_user_question_result::answer_item::Answer as A;
                match ans.answer.as_ref() {
                    Some(A::MultipleChoice(mc)) => {
                        let mut parts = Vec::new();
                        if !mc.selected_options.is_empty() {
                            parts.push(format!("selected: [{}]", mc.selected_options.join(", ")));
                        }
                        if !mc.other_text.is_empty() {
                            parts.push(format!("other: {}", mc.other_text));
                        }
                        out.push_str(&format!(
                            "{}: {}\n",
                            ans.question_id,
                            parts.join("; ")
                        ));
                    }
                    Some(_) => out.push_str(&format!("{}: (skipped)\n", ans.question_id)),
                    None => out.push_str(&format!("{}: (no answer)\n", ans.question_id)),
                }
            }
            if out.is_empty() {
                String::from("(no answers)")
            } else {
                out
            }
        }
        Some(R::Error(e)) => format!("ERROR: {}", e.message),
        None => String::from("ERROR: empty ask_user_question result"),
    }
}

// ── WriteToLongRunningShellCommand ───────────────────────────────────────

const WRITE_LRC_DESC: &str =
    "Send keystrokes (or a line / block of text) to a long-running shell \
     command identified by `command_id` (returned in a previous \
     `run_shell_command` snapshot). Use `mode: \"line\"` for REPL prompts, \
     `mode: \"block\"` for multi-line bracketed paste, `mode: \"raw\"` \
     (default) for raw bytes including control characters.";

const WRITE_LRC_SCHEMA: &str = r#"{
  "type": "object",
  "properties": {
    "command_id": {"type": "string"},
    "input":      {"type": "string", "description": "Text to write (UTF-8 encoded)."},
    "mode":       {"type": "string", "enum": ["raw", "line", "block"], "default": "raw", "description": "Write mode. Defaults to raw bytes."}
  },
  "required": ["command_id", "input"],
  "additionalProperties": false
}"#;

#[derive(Deserialize)]
struct WriteLrcInput {
    command_id: String,
    input: String,
    #[serde(default)]
    mode: Option<String>,
}

fn parse_write_to_lrc(
    value: Value,
) -> anyhow::Result<api::message::tool_call::WriteToLongRunningShellCommand> {
    let parsed: WriteLrcInput =
        serde_json::from_value(value).context("write_to_long_running_shell_command shape mismatch")?;
    if parsed.command_id.is_empty() {
        return Err(anyhow!("write_to_lrc requires command_id"));
    }
    let mode_oneof = match parsed.mode.as_deref().unwrap_or("raw") {
        "raw" => api::message::tool_call::write_to_long_running_shell_command::mode::Mode::Raw(()),
        "line" => api::message::tool_call::write_to_long_running_shell_command::mode::Mode::Line(()),
        "block" => api::message::tool_call::write_to_long_running_shell_command::mode::Mode::Block(()),
        other => return Err(anyhow!("write_to_lrc unknown mode `{other}`")),
    };
    Ok(api::message::tool_call::WriteToLongRunningShellCommand {
        input: parsed.input.into_bytes(),
        mode: Some(api::message::tool_call::write_to_long_running_shell_command::Mode {
            mode: Some(mode_oneof),
        }),
        command_id: parsed.command_id,
    })
}

fn write_to_lrc_to_json(w: &api::message::tool_call::WriteToLongRunningShellCommand) -> Value {
    let mode_str = match w.mode.as_ref().and_then(|m| m.mode.as_ref()) {
        Some(api::message::tool_call::write_to_long_running_shell_command::mode::Mode::Raw(_)) => {
            "raw"
        }
        Some(api::message::tool_call::write_to_long_running_shell_command::mode::Mode::Line(_)) => {
            "line"
        }
        Some(api::message::tool_call::write_to_long_running_shell_command::mode::Mode::Block(_)) => {
            "block"
        }
        None => "raw",
    };
    let input_str = String::from_utf8_lossy(&w.input).to_string();
    json!({
        "command_id": w.command_id,
        "input": input_str,
        "mode": mode_str,
    })
}

fn write_to_lrc_result_to_text(r: &api::WriteToLongRunningShellCommandResult) -> String {
    use api::write_to_long_running_shell_command_result::Result as R;
    match r.result.as_ref() {
        Some(R::CommandFinished(f)) => format!(
            "Command finished. Exit code: {}\n----- output -----\n{}",
            f.exit_code, f.output
        ),
        Some(R::LongRunningCommandSnapshot(s)) => format!(
            "[long-running, snapshot]\n----- output -----\n{}",
            s.output
        ),
        Some(R::Error(_)) => String::from("ERROR: shell command error"),
        None => String::from("ERROR: empty write_to_lrc result"),
    }
}

// ── ReadShellCommandOutput ───────────────────────────────────────────────

const READ_SHELL_OUTPUT_DESC: &str =
    "Re-read the latest output of a long-running shell command identified by \
     `command_id`. Optionally wait `delay_seconds` seconds before reading, or \
     wait for completion via `wait_until_complete: true`. Use to poll \
     progress of long jobs (build, test, server log) without re-issuing them.";

const READ_SHELL_OUTPUT_SCHEMA: &str = r#"{
  "type": "object",
  "properties": {
    "command_id": {"type": "string"},
    "delay_seconds": {"type": "number", "description": "Optional. Wait this long before snapshotting."},
    "wait_until_complete": {"type": "boolean", "description": "Optional. If true, block until the command exits."}
  },
  "required": ["command_id"],
  "additionalProperties": false
}"#;

#[derive(Deserialize)]
struct ReadShellOutputInput {
    command_id: String,
    #[serde(default)]
    delay_seconds: Option<f64>,
    #[serde(default)]
    wait_until_complete: bool,
}

fn parse_read_shell_command_output(
    value: Value,
) -> anyhow::Result<api::message::tool_call::ReadShellCommandOutput> {
    let parsed: ReadShellOutputInput = serde_json::from_value(value)
        .context("read_shell_command_output shape mismatch")?;
    if parsed.command_id.is_empty() {
        return Err(anyhow!("read_shell_command_output requires command_id"));
    }
    let delay = if parsed.wait_until_complete {
        Some(api::message::tool_call::read_shell_command_output::Delay::OnCompletion(()))
    } else if let Some(secs) = parsed.delay_seconds {
        if secs < 0.0 {
            return Err(anyhow!("delay_seconds must be non-negative"));
        }
        let s = secs.trunc() as i64;
        let ns = ((secs - s as f64) * 1_000_000_000.0).round() as i32;
        Some(api::message::tool_call::read_shell_command_output::Delay::Duration(
            ::prost_types::Duration {
                seconds: s,
                nanos: ns,
            },
        ))
    } else {
        None
    };
    Ok(api::message::tool_call::ReadShellCommandOutput {
        command_id: parsed.command_id,
        delay,
    })
}

fn read_shell_command_output_to_json(
    r: &api::message::tool_call::ReadShellCommandOutput,
) -> Value {
    let mut obj = serde_json::Map::new();
    obj.insert("command_id".into(), json!(r.command_id));
    if let Some(d) = r.delay.as_ref() {
        match d {
            api::message::tool_call::read_shell_command_output::Delay::Duration(dur) => {
                let secs = dur.seconds as f64 + dur.nanos as f64 / 1_000_000_000.0;
                obj.insert("delay_seconds".into(), json!(secs));
            }
            api::message::tool_call::read_shell_command_output::Delay::OnCompletion(_) => {
                obj.insert("wait_until_complete".into(), json!(true));
            }
        }
    }
    Value::Object(obj)
}

fn read_shell_command_output_result_to_text(r: &api::ReadShellCommandOutputResult) -> String {
    use api::read_shell_command_output_result::Result as R;
    let preface = if r.command.is_empty() {
        String::from("Read shell output:")
    } else {
        format!("Read output of `{}`:", r.command)
    };
    match r.result.as_ref() {
        Some(R::CommandFinished(f)) => format!(
            "{preface}\nExit code: {}\n----- output -----\n{}",
            f.exit_code, f.output
        ),
        Some(R::LongRunningCommandSnapshot(s)) => format!(
            "{preface}\n[long-running, snapshot]\n----- output -----\n{}",
            s.output
        ),
        Some(R::Error(_)) => format!("{preface}\nERROR: shell command error"),
        None => format!("{preface}\nERROR: empty result"),
    }
}

// ── TransferShellCommandControlToUser ────────────────────────────────────

const TRANSFER_CONTROL_DESC: &str =
    "Hand control of a long-running interactive shell command back to the \
     user (e.g. they should now interact with `vim` / `top` / a REPL \
     directly). Provide a short `reason` explaining what to do next.";

const TRANSFER_CONTROL_SCHEMA: &str = r#"{
  "type": "object",
  "properties": {
    "reason": {"type": "string", "description": "Short message shown to the user explaining the handoff."}
  },
  "required": ["reason"],
  "additionalProperties": false
}"#;

#[derive(Deserialize)]
struct TransferControlInput {
    reason: String,
}

fn parse_transfer_control(
    value: Value,
) -> anyhow::Result<api::message::tool_call::TransferShellCommandControlToUser> {
    let parsed: TransferControlInput = serde_json::from_value(value)
        .context("transfer_shell_command_control_to_user shape mismatch")?;
    if parsed.reason.trim().is_empty() {
        return Err(anyhow!("transfer_shell_command_control_to_user requires reason"));
    }
    Ok(api::message::tool_call::TransferShellCommandControlToUser {
        reason: parsed.reason,
    })
}

fn transfer_control_to_json(
    t: &api::message::tool_call::TransferShellCommandControlToUser,
) -> Value {
    json!({"reason": t.reason})
}

fn transfer_control_result_to_text(r: &api::TransferShellCommandControlToUserResult) -> String {
    use api::transfer_shell_command_control_to_user_result::Result as R;
    match r.result.as_ref() {
        Some(R::CommandFinished(f)) => format!(
            "Control returned. Exit code: {}\n----- output -----\n{}",
            f.exit_code, f.output
        ),
        Some(R::LongRunningCommandSnapshot(s)) => format!(
            "[control transferred, snapshot]\n----- output -----\n{}",
            s.output
        ),
        Some(R::Error(_)) => String::from("ERROR: shell command error"),
        None => String::from("ERROR: empty transfer-control result"),
    }
}

// ── ReadMcpResource (Tier 3) ─────────────────────────────────────────────

const READ_MCP_RESOURCE_DESC: &str =
    "Read the contents of an MCP (Model Context Protocol) resource. \
     MCP resources are user-mounted data sources (databases, knowledge bases, \
     ticketing systems, etc.). The user's available servers are listed in your \
     system prompt under \"Available MCP servers\". Provide the resource URI \
     exactly as listed under that server. Use this only when the resource the \
     user is asking about lives behind one of those servers.";

const READ_MCP_RESOURCE_SCHEMA: &str = r#"{
  "type": "object",
  "properties": {
    "uri": {
      "type": "string",
      "description": "URI of the MCP resource (e.g. 'file:///foo' or a server-specific URI)."
    },
    "server_id": {
      "type": "string",
      "description": "ID of the MCP server (uuid string from the system prompt). Optional if only one server provides this URI."
    }
  },
  "required": ["uri"]
}"#;

#[derive(Deserialize)]
struct ReadMcpResourceInput {
    uri: String,
    #[serde(default)]
    server_id: String,
}

fn parse_read_mcp_resource(value: Value) -> anyhow::Result<api::message::tool_call::ReadMcpResource> {
    let p: ReadMcpResourceInput =
        serde_json::from_value(value).context("read_mcp_resource shape mismatch")?;
    if p.uri.trim().is_empty() {
        return Err(anyhow!("read_mcp_resource requires a non-empty `uri`"));
    }
    Ok(api::message::tool_call::ReadMcpResource {
        uri: p.uri,
        server_id: p.server_id,
        server_identity: None,
    })
}

fn read_mcp_resource_to_json(r: &api::message::tool_call::ReadMcpResource) -> Value {
    json!({"uri": r.uri, "server_id": r.server_id})
}

fn read_mcp_resource_result_to_text(r: &api::ReadMcpResourceResult) -> String {
    use api::read_mcp_resource_result::Result as R;
    match r.result.as_ref() {
        Some(R::Success(s)) => {
            if s.contents.is_empty() {
                return "(no contents)".into();
            }
            let mut out = String::new();
            for c in &s.contents {
                use api::mcp_resource_content::ContentType as CT;
                match c.content_type.as_ref() {
                    Some(CT::Text(t)) => {
                        out.push_str(&format!(
                            "--- {} ({}) ---\n{}\n",
                            c.uri, t.mime_type, t.content
                        ));
                    }
                    Some(CT::Binary(b)) => {
                        out.push_str(&format!(
                            "--- {} ({}, {} bytes binary, base64-elided) ---\n",
                            c.uri,
                            b.mime_type,
                            b.data.len()
                        ));
                    }
                    None => {}
                }
            }
            out
        }
        Some(R::Error(e)) => format!("ERROR: {}", e.message),
        None => "ERROR: empty read_mcp_resource result".into(),
    }
}

// ── CallMcpTool (Tier 3) ─────────────────────────────────────────────────

const CALL_MCP_TOOL_DESC: &str =
    "Invoke an MCP (Model Context Protocol) tool exposed by one of the user's \
     mounted MCP servers. The available servers and their tools are listed in \
     your system prompt under \"Available MCP servers\". `name` MUST exactly \
     match a tool name advertised by `server_id`. `arguments` must satisfy the \
     tool's input schema (also shown in the system prompt).";

const CALL_MCP_TOOL_SCHEMA: &str = r#"{
  "type": "object",
  "properties": {
    "name": {
      "type": "string",
      "description": "Tool name as advertised by the MCP server."
    },
    "server_id": {
      "type": "string",
      "description": "ID of the MCP server (uuid from the system prompt)."
    },
    "arguments": {
      "type": "object",
      "description": "Named JSON arguments matching the tool's schema. Must be an object (not a string or array)."
    }
  },
  "required": ["name", "arguments"]
}"#;

#[derive(Deserialize)]
struct CallMcpToolInput {
    name: String,
    #[serde(default)]
    server_id: String,
    #[serde(default)]
    arguments: Value,
}

fn parse_call_mcp_tool(value: Value) -> anyhow::Result<api::message::tool_call::CallMcpTool> {
    let p: CallMcpToolInput =
        serde_json::from_value(value).context("call_mcp_tool shape mismatch")?;
    if p.name.trim().is_empty() {
        return Err(anyhow!("call_mcp_tool requires a non-empty `name`"));
    }
    let args_struct = match &p.arguments {
        Value::Object(_) => super::serde_json_to_prost_struct(&p.arguments)
            .map_err(|e| anyhow!("`arguments` must be a JSON object: {e}"))?,
        Value::Null => prost_types::Struct::default(),
        other => {
            return Err(anyhow!(
                "`arguments` must be a JSON object, got {}",
                short_json_type(other)
            ))
        }
    };
    Ok(api::message::tool_call::CallMcpTool {
        name: p.name,
        args: Some(args_struct),
        server_id: p.server_id,
        server_identity: None,
    })
}

fn call_mcp_tool_to_json(c: &api::message::tool_call::CallMcpTool) -> Value {
    let args = c
        .args
        .as_ref()
        .map(super::prost_struct_to_serde_json)
        .unwrap_or_else(|| Value::Object(Default::default()));
    json!({"name": c.name, "server_id": c.server_id, "arguments": args})
}

fn call_mcp_tool_result_to_text(r: &api::CallMcpToolResult) -> String {
    use api::call_mcp_tool_result::Result as R;
    match r.result.as_ref() {
        Some(R::Success(s)) => {
            if s.results.is_empty() {
                return "(empty success)".into();
            }
            let mut out = String::new();
            for entry in &s.results {
                use api::call_mcp_tool_result::success::result::Result as ER;
                match entry.result.as_ref() {
                    Some(ER::Text(t)) => out.push_str(&t.text),
                    Some(ER::Image(im)) => {
                        out.push_str(&format!(
                            "[image {} ({} bytes)]",
                            im.mime_type,
                            im.data.len()
                        ));
                    }
                    Some(ER::Resource(rc)) => {
                        let single = api::ReadMcpResourceResult {
                            result: Some(api::read_mcp_resource_result::Result::Success(
                                api::read_mcp_resource_result::Success {
                                    contents: vec![rc.clone()],
                                },
                            )),
                        };
                        out.push_str(&read_mcp_resource_result_to_text(&single));
                    }
                    None => {}
                }
                out.push('\n');
            }
            out
        }
        Some(R::Error(e)) => format!("ERROR: {}", e.message),
        None => "ERROR: empty call_mcp_tool result".into(),
    }
}

fn short_json_type(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_schemas_are_valid_json() {
        for kind in all_tools() {
            let s = schema(*kind);
            let _: Value = serde_json::from_str(s)
                .unwrap_or_else(|_| panic!("schema for {:?} must be valid JSON", kind));
        }
    }

    #[test]
    fn names_round_trip() {
        for kind in all_tools() {
            assert_eq!(from_name(name(*kind)), Some(*kind));
        }
    }

    #[test]
    fn read_files_parse_roundtrip() {
        let v = json!({"files": [{"path": "src/lib.rs"}]});
        let proto = parse_read_files(v).unwrap();
        assert_eq!(proto.files[0].name, "src/lib.rs");
    }

    #[test]
    fn read_files_rejects_empty() {
        assert!(parse_read_files(json!({"files": []})).is_err());
    }

    #[test]
    fn run_shell_command_parses() {
        let v = json!({"command": "ls -la", "is_read_only": true});
        let proto = parse_run_shell_command(v).unwrap();
        assert_eq!(proto.command, "ls -la");
        #[allow(deprecated)]
        let ro = proto.is_read_only;
        assert!(ro);
    }

    #[test]
    fn run_shell_command_rejects_empty() {
        assert!(parse_run_shell_command(json!({"command": ""})).is_err());
    }

    #[test]
    fn grep_parses() {
        let v = json!({"queries": ["TODO"], "path": "src"});
        let proto = parse_grep(v).unwrap();
        assert_eq!(proto.queries, vec!["TODO".to_string()]);
        assert_eq!(proto.path, "src");
    }

    #[test]
    fn file_glob_parses_minimal() {
        let v = json!({"patterns": ["**/*.rs"]});
        let proto = parse_file_glob_v2(v).unwrap();
        assert_eq!(proto.patterns, vec!["**/*.rs".to_string()]);
        assert_eq!(proto.search_dir, "");
        assert_eq!(proto.max_matches, 0);
    }

    #[test]
    fn apply_file_diffs_parses_search_replace() {
        let v = json!({
            "summary": "rename foo to bar",
            "diffs": [{"file_path": "a.rs", "search": "foo", "replace": "bar"}],
        });
        let proto = parse_apply_file_diffs(v).unwrap();
        assert_eq!(proto.summary, "rename foo to bar");
        assert_eq!(proto.diffs.len(), 1);
        assert_eq!(proto.diffs[0].file_path, "a.rs");
    }

    #[test]
    fn apply_file_diffs_rejects_empty() {
        let v = json!({"summary": "noop"});
        assert!(parse_apply_file_diffs(v).is_err());
    }

    #[test]
    fn apply_file_diffs_accepts_new_files_only() {
        let v = json!({
            "summary": "create README",
            "new_files": [{"file_path": "README.md", "content": "hi"}],
        });
        let proto = parse_apply_file_diffs(v).unwrap();
        assert_eq!(proto.new_files.len(), 1);
        assert!(proto.diffs.is_empty());
    }

    #[test]
    fn parse_input_dispatch_works() {
        let v = json!({"queries": ["x"], "path": "."});
        let tool = parse_input(ToolKind::Grep, v).unwrap();
        assert!(matches!(tool, api::message::tool_call::Tool::Grep(_)));
    }

    #[test]
    fn ask_user_question_parses_minimal() {
        let v = json!({
            "questions": [{
                "question_id": "q1",
                "question": "Pick one",
                "options": ["A", "B"]
            }]
        });
        let proto = parse_ask_user_question(v).unwrap();
        assert_eq!(proto.questions.len(), 1);
        assert_eq!(proto.questions[0].question_id, "q1");
        assert!(matches!(
            proto.questions[0].question_type,
            Some(api::ask_user_question::question::QuestionType::MultipleChoice(_))
        ));
    }

    #[test]
    fn ask_user_question_rejects_empty_options() {
        let v = json!({
            "questions": [{
                "question_id": "q1",
                "question": "Pick",
                "options": []
            }]
        });
        assert!(parse_ask_user_question(v).is_err());
    }

    #[test]
    fn write_to_lrc_parses_modes() {
        for (s, want_kind) in [("raw", "raw"), ("line", "line"), ("block", "block")] {
            let v = json!({"command_id": "c", "input": "ls\n", "mode": s});
            let proto = parse_write_to_lrc(v).unwrap();
            assert_eq!(proto.command_id, "c");
            // Reverse round-trip via to_json should preserve mode.
            let back = write_to_lrc_to_json(&proto);
            assert_eq!(back["mode"].as_str().unwrap(), want_kind);
        }
    }

    #[test]
    fn write_to_lrc_rejects_unknown_mode() {
        let v = json!({"command_id": "c", "input": "x", "mode": "weird"});
        assert!(parse_write_to_lrc(v).is_err());
    }

    #[test]
    fn read_shell_command_output_with_wait() {
        let v = json!({"command_id": "c", "wait_until_complete": true});
        let proto = parse_read_shell_command_output(v).unwrap();
        assert!(matches!(
            proto.delay,
            Some(api::message::tool_call::read_shell_command_output::Delay::OnCompletion(_))
        ));
    }

    #[test]
    fn read_shell_command_output_with_duration() {
        let v = json!({"command_id": "c", "delay_seconds": 2.5});
        let proto = parse_read_shell_command_output(v).unwrap();
        if let Some(api::message::tool_call::read_shell_command_output::Delay::Duration(d)) =
            &proto.delay
        {
            assert_eq!(d.seconds, 2);
            assert!(d.nanos > 400_000_000 && d.nanos < 600_000_000);
        } else {
            panic!("expected Duration delay");
        }
    }

    #[test]
    fn transfer_control_parses() {
        let v = json!({"reason": "you handle it"});
        let proto = parse_transfer_control(v).unwrap();
        assert_eq!(proto.reason, "you handle it");
    }

    #[test]
    fn transfer_control_rejects_empty() {
        let v = json!({"reason": ""});
        assert!(parse_transfer_control(v).is_err());
    }

    #[test]
    fn all_eleven_tools_registered() {
        // 9 Tier-1/2 tools + 2 Tier-3 MCP tools (read_mcp_resource, call_mcp_tool).
        assert_eq!(all_tools().len(), 11);
        for k in all_tools() {
            assert!(from_name(name(*k)).is_some());
        }
    }

    #[test]
    fn grep_rejects_empty_path() {
        let v = json!({"queries": ["x"], "path": ""});
        assert!(parse_grep(v).is_err());
    }

    #[test]
    fn grep_rejects_empty_query_string() {
        let v = json!({"queries": [""], "path": "src"});
        assert!(parse_grep(v).is_err());
    }

    #[test]
    fn grep_trims_whitespace_queries() {
        let v = json!({"queries": ["  TODO  ", ""], "path": "src"});
        let proto = parse_grep(v).unwrap();
        assert_eq!(proto.queries, vec!["TODO".to_string()]);
    }

    #[test]
    fn ask_user_rejects_too_many_options() {
        let v = json!({"questions": [{
            "question_id": "q",
            "question": "?",
            "options": ["a", "b", "c", "d", "e"],
        }]});
        assert!(parse_ask_user_question(v).is_err());
    }

    #[test]
    fn ask_user_rejects_recommended_out_of_range() {
        let v = json!({"questions": [{
            "question_id": "q",
            "question": "?",
            "options": ["a", "b"],
            "recommended_option_index": 5,
        }]});
        assert!(parse_ask_user_question(v).is_err());
    }

    #[test]
    fn apply_file_diffs_rejects_empty_search() {
        let v = json!({
            "summary": "edit",
            "diffs": [{"file_path": "a.rs", "search": "", "replace": "x"}],
        });
        assert!(parse_apply_file_diffs(v).is_err());
    }

    #[test]
    fn apply_file_diffs_rejects_empty_summary() {
        let v = json!({
            "summary": "",
            "new_files": [{"file_path": "a.rs", "content": "x"}],
        });
        assert!(parse_apply_file_diffs(v).is_err());
    }

    #[test]
    fn file_glob_schema_does_not_advertise_max_fields() {
        let s: Value = serde_json::from_str(FILE_GLOB_SCHEMA).unwrap();
        let props = s["properties"].as_object().unwrap();
        assert!(props.contains_key("patterns"));
        assert!(props.contains_key("search_dir"));
        // These are dropped because the client ignores them.
        assert!(!props.contains_key("max_matches"));
        assert!(!props.contains_key("max_depth"));
    }

    #[test]
    fn read_mcp_resource_parses_minimal() {
        let v = json!({"uri": "memory://default/note-1"});
        let proto = parse_read_mcp_resource(v).unwrap();
        assert_eq!(proto.uri, "memory://default/note-1");
        assert_eq!(proto.server_id, "");
    }

    #[test]
    fn read_mcp_resource_rejects_empty_uri() {
        let v = json!({"uri": "   "});
        assert!(parse_read_mcp_resource(v).is_err());
    }

    #[test]
    fn call_mcp_tool_parses_with_args() {
        let v = json!({
            "name": "search",
            "server_id": "srv-1",
            "arguments": {"q": "todo", "limit": 10}
        });
        let proto = parse_call_mcp_tool(v).unwrap();
        assert_eq!(proto.name, "search");
        assert_eq!(proto.server_id, "srv-1");
        let args = proto.args.expect("args struct");
        assert!(args.fields.contains_key("q"));
        assert!(args.fields.contains_key("limit"));
    }

    #[test]
    fn call_mcp_tool_rejects_array_arguments() {
        let v = json!({"name": "x", "arguments": ["nope"]});
        assert!(parse_call_mcp_tool(v).is_err());
    }

    #[test]
    fn call_mcp_tool_rejects_empty_name() {
        let v = json!({"name": "", "arguments": {}});
        assert!(parse_call_mcp_tool(v).is_err());
    }

    #[test]
    fn call_mcp_tool_accepts_null_arguments() {
        // Null arguments → empty Struct (some models emit `null` for empty objects).
        let v = json!({"name": "ping", "arguments": null});
        let proto = parse_call_mcp_tool(v).unwrap();
        assert_eq!(proto.args.unwrap().fields.len(), 0);
    }
}

