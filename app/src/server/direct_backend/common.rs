//! Shared helpers across the OpenAI / Anthropic / Gemini backends:
//! the JSON-payload parsers for `generate_commands` / `generate_metadata`,
//! the ```json fence stripper, and the BYOK unlimited request-limit stub.

use anyhow::{anyhow, Context};
use chrono::Utc;
use serde_json::Value;
use warp_graphql::scalars::time::ServerTimestamp;

use crate::ai::request_usage_model::{RequestLimitInfo, RequestLimitRefreshDuration};
use crate::ai_assistant::{AIGeneratedCommand, AIGeneratedCommandParameter};
use crate::drive::workflows::ai_assist::{GeneratedArgument, GeneratedCommandMetadata};

/// System prompt that asks the LLM to return a JSON object describing one or
/// more shell commands that satisfy the natural-language input.
pub const COMMANDS_SYSTEM_PROMPT: &str = r#"You are a shell command suggester for the Warp terminal.
Given a natural language description, return a JSON object with a single key
`commands` whose value is an array of suggestions. Each suggestion has:
  - `command` (string): the full shell command, with `{{argument}}` placeholders
    where the user should fill in values
  - `description` (string): a short, friendly explanation
  - `parameters` (array): one entry per `{{argument}}`, each shaped like
      { "id": "<argument_name>", "description": "<what to put here>" }

Return ONLY valid JSON; no Markdown, no commentary, no preamble."#;

/// System prompt that asks the LLM to return metadata for a single command.
pub const METADATA_SYSTEM_PROMPT: &str = r#"You analyse a single shell command for the Warp terminal.
Return ONLY a JSON object with these keys:
  - `command` (string): the command with `{{name}}` placeholders for each argument
  - `title`   (string): a short, human-friendly name (≤ 60 chars)
  - `description` (string): one sentence explaining what the command does
  - `arguments` (array): each entry is
      { "name": "<placeholder name>", "description": "<purpose>", "default_value": "<example or empty>" }

If the command has no arguments, return `arguments: []`. No Markdown, no commentary."#;

/// System prompt for the AI Chat side panel / # natural-language conversations.
pub const DIALOGUE_SYSTEM_PROMPT: &str =
    "You are Warp's terminal assistant. Answer the user's question concisely. \
     Use Markdown formatting for code blocks; keep responses focused on the terminal context.";

pub fn parse_commands_payload(raw: &str) -> anyhow::Result<Vec<AIGeneratedCommand>> {
    let value: Value = serde_json::from_str(strip_json_fence(raw))
        .with_context(|| format!("not valid JSON: {}", truncate_for_log(raw, 256)))?;
    let array = value
        .get("commands")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow!("missing `commands` array in response"))?;

    let mut out = Vec::with_capacity(array.len());
    for entry in array {
        let command = entry
            .get("command")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        if command.is_empty() {
            continue;
        }
        let description = entry
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        let parameters = entry
            .get("parameters")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|p| {
                        let id = p.get("id").and_then(|v| v.as_str()).unwrap_or_default();
                        if id.is_empty() {
                            return None;
                        }
                        let desc = p
                            .get("description")
                            .and_then(|v| v.as_str())
                            .unwrap_or_default();
                        Some(AIGeneratedCommandParameter::new(
                            id.to_string(),
                            desc.to_string(),
                        ))
                    })
                    .collect()
            })
            .unwrap_or_default();

        out.push(AIGeneratedCommand::new(command, description, parameters));
    }
    Ok(out)
}

pub fn parse_metadata_payload(raw: &str) -> anyhow::Result<GeneratedCommandMetadata> {
    let value: Value = serde_json::from_str(strip_json_fence(raw))
        .with_context(|| format!("not valid JSON: {}", truncate_for_log(raw, 256)))?;

    let command = value
        .get("command")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("missing `command`"))?
        .to_string();
    let title = value
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let description = value
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let arguments = value
        .get("arguments")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|a| {
                    let name = a.get("name").and_then(|v| v.as_str()).unwrap_or_default();
                    if name.is_empty() {
                        return None;
                    }
                    let desc = a
                        .get("description")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string();
                    let default_value = a
                        .get("default_value")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string();
                    Some(GeneratedArgument {
                        name: name.to_string(),
                        description: desc,
                        default_value,
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(GeneratedCommandMetadata {
        command,
        title,
        description,
        arguments,
    })
}

/// Some providers (Ollama, older Anthropic-compat gateways) wrap JSON output in
/// ```json fences even when asked not to. Strip them before parsing.
pub fn strip_json_fence(raw: &str) -> &str {
    let trimmed = raw.trim();
    if let Some(rest) = trimmed.strip_prefix("```json") {
        return rest.trim_end_matches("```").trim();
    }
    if let Some(rest) = trimmed.strip_prefix("```") {
        return rest.trim_end_matches("```").trim();
    }
    trimmed
}

pub fn truncate_for_log(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…(truncated)", &s[..max])
    }
}

/// Direct mode bypasses Warp Credits entirely; we still have to populate the
/// `RequestLimitInfo` field on `GenerateDialogueResult::Success` so downstream
/// rate-limit UI doesn't surface "you're out of credits".
pub fn byok_unlimited_request_limit() -> RequestLimitInfo {
    let next_refresh_time = ServerTimestamp::new(Utc::now() + chrono::Duration::days(365));
    RequestLimitInfo {
        limit: usize::MAX,
        num_requests_used_since_refresh: 0,
        included_usage_cents: None,
        usage_cents_used_since_last_refresh: None,
        next_refresh_time,
        is_unlimited: true,
        request_limit_refresh_duration: RequestLimitRefreshDuration::Monthly,
        is_unlimited_voice: true,
        voice_request_limit: usize::MAX,
        voice_requests_used_since_last_refresh: 0,
        is_unlimited_codebase_indices: true,
        max_codebase_indices: usize::MAX,
        max_files_per_repo: usize::MAX,
        embedding_generation_batch_size: 100,
    }
}
