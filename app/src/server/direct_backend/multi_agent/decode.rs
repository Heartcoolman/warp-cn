//! `api::Request` → provider-neutral chat history.
//!
//! Walks `task_context.tasks[*].messages` chronologically plus the current
//! `input.user_inputs`, emitting `Vec<NormalizedTurn>` where each turn groups
//! all consecutive atoms produced by the same actor (model response or
//! user/tool side). Without grouping, Anthropic rejects "roles must alternate"
//! and OpenAI rejects unbatched `tool_calls` arrays.
//!
//! Anything outside our Tier-1/2 tool surface downgrades the history to "drop
//! everything except the last user text" — providers refuse chats with
//! dangling tool_use blocks and we can't fabricate missing tool_results.

use serde_json::Value;
use warp_multi_agent_api as api;

use super::tool_schema::{message_result_to_text, proto_to_history, request_result_to_text, ToolKind};

/// One tool call inside an assistant turn.
#[derive(Debug, Clone)]
pub struct ToolUse {
    pub tool_use_id: String,
    pub tool_kind: ToolKind,
    pub input: Value,
}

/// One tool result inside a user turn.
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub tool_use_id: String,
    pub tool_kind: ToolKind,
    pub content: String,
    pub is_error: bool,
}

/// Grouped turn: one user or assistant block, possibly mixing text + tool
/// uses/results, ready to be projected to a single provider message.
#[derive(Debug, Clone)]
pub enum NormalizedTurn {
    User {
        text: Option<String>,
        tool_results: Vec<ToolResult>,
    },
    Assistant {
        text: Option<String>,
        tool_uses: Vec<ToolUse>,
        /// Chain-of-thought from reasoning models. DeepSeek-R1 / o1 require
        /// the prior turn's `reasoning_content` to be echoed back in the
        /// next request, otherwise they reject with HTTP 400. Other
        /// providers (Anthropic / Gemini) ignore this.
        reasoning: Option<String>,
    },
}

#[derive(Debug)]
pub struct DecodedRequest {
    pub conversation_id: String,
    /// Existing task ID to AddMessagesToTask onto. None means we should emit
    /// a fresh CreateTask. Reusing an existing ID is required for multi-turn
    /// conversations because the server-backed root task can't be re-upgraded.
    pub existing_task_id: Option<String>,
    pub turns: Vec<NormalizedTurn>,
    /// `false` if any unsupported tool/result was encountered. Caller dropped
    /// the history down to "last user text"; UX surfaces a system note.
    #[allow(dead_code)]
    pub history_compatible: bool,
    /// `true` if at least one user input was extracted.
    pub has_user_input: bool,
    /// MCP context (servers + their resources/tools) carried through to each
    /// driver so the system prompt can advertise valid `server_id` values
    /// for the `read_mcp_resource` / `call_mcp_tool` Tier-3 tools.
    pub mcp_context: Option<api::request::McpContext>,
    /// `Settings.ModelConfig.base` — the LLM ID the user chose in the
    /// `/MODEL` picker. When present this **overrides** the configured
    /// `DirectBackendConfig` `model_id` so the picker selection actually
    /// reaches the provider, instead of silently falling back to the
    /// per-provider default (`gpt-4o-mini` / `claude-sonnet-4-6` / …).
    pub base_model: Option<String>,
}

#[derive(Debug, Clone)]
enum Atom {
    UserText(String),
    UserToolResult(ToolResult),
    AssistantText(String),
    AssistantReasoning(String),
    AssistantToolUse(ToolUse),
}

pub fn decode(req: &api::Request) -> DecodedRequest {
    let conversation_id = req
        .metadata
        .as_ref()
        .map(|m| m.conversation_id.clone())
        .filter(|s| !s.is_empty())
        .unwrap_or_default();

    let existing_task_id = req
        .task_context
        .as_ref()
        .and_then(|tc| tc.tasks.first())
        .map(|t| t.id.clone())
        .filter(|id| !id.is_empty());

    let mut atoms = Vec::new();
    let mut compat = true;
    let mut has_user_input = false;

    if let Some(ctx) = req.task_context.as_ref() {
        for task in &ctx.tasks {
            for msg in &task.messages {
                project_history_message(msg, &mut atoms, &mut compat, &mut has_user_input);
            }
        }
    }

    if let Some(input) = req.input.as_ref() {
        if let Some(api::request::input::Type::UserInputs(inputs)) = input.r#type.as_ref() {
            for ui in &inputs.inputs {
                if let Some(inner) = ui.input.as_ref() {
                    project_user_input(inner, &mut atoms, &mut compat, &mut has_user_input);
                }
            }
        }
    }

    let turns = if compat {
        coalesce(atoms)
    } else {
        // History truncated; surface a synthetic system note so the model
        // (and the client log) understand context is missing.
        let last_user = atoms
            .into_iter()
            .rev()
            .find(|a| matches!(a, Atom::UserText(_)));
        match last_user {
            Some(Atom::UserText(t)) => vec![NormalizedTurn::User {
                text: Some(format!(
                    "[System: previous conversation history was dropped because it referenced \
                     tools this build doesn't support; only your latest message is visible.]\n\n{t}"
                )),
                tool_results: vec![],
            }],
            _ => vec![],
        }
    };

    let base_model = req
        .settings
        .as_ref()
        .and_then(|s| s.model_config.as_ref())
        .map(|mc| mc.base.trim().to_owned())
        .filter(|s| !s.is_empty());

    DecodedRequest {
        conversation_id,
        existing_task_id,
        turns,
        history_compatible: compat,
        has_user_input,
        mcp_context: req.mcp_context.clone(),
        base_model,
    }
}

fn coalesce(atoms: Vec<Atom>) -> Vec<NormalizedTurn> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < atoms.len() {
        let is_user = matches!(
            &atoms[i],
            Atom::UserText(_) | Atom::UserToolResult(_)
        );

        let mut text_parts: Vec<String> = Vec::new();
        let mut reasoning_parts: Vec<String> = Vec::new();
        let mut tool_uses: Vec<ToolUse> = Vec::new();
        let mut tool_results: Vec<ToolResult> = Vec::new();

        while i < atoms.len() {
            let still_user = matches!(
                &atoms[i],
                Atom::UserText(_) | Atom::UserToolResult(_)
            );
            if still_user != is_user {
                break;
            }
            match atoms[i].clone() {
                Atom::UserText(t) => text_parts.push(t),
                Atom::UserToolResult(r) => tool_results.push(r),
                Atom::AssistantText(t) => text_parts.push(t),
                Atom::AssistantReasoning(r) => reasoning_parts.push(r),
                Atom::AssistantToolUse(u) => tool_uses.push(u),
            }
            i += 1;
        }

        let text = if text_parts.is_empty() {
            None
        } else {
            Some(text_parts.join("\n\n"))
        };
        let reasoning = if reasoning_parts.is_empty() {
            None
        } else {
            Some(reasoning_parts.join("\n\n"))
        };

        if is_user {
            // Skip empty user turns (no text, no results) — they emit nothing
            // useful and providers reject empty messages.
            if text.is_some() || !tool_results.is_empty() {
                out.push(NormalizedTurn::User { text, tool_results });
            }
        } else if text.is_some() || !tool_uses.is_empty() || reasoning.is_some() {
            out.push(NormalizedTurn::Assistant { text, tool_uses, reasoning });
        }
    }
    out
}

fn project_history_message(
    msg: &api::Message,
    out: &mut Vec<Atom>,
    compat: &mut bool,
    has_user_input: &mut bool,
) {
    use api::message::Message as M;
    match msg.message.as_ref() {
        Some(M::UserQuery(uq)) => {
            if !uq.query.is_empty() {
                out.push(Atom::UserText(uq.query.clone()));
                *has_user_input = true;
            }
        }
        Some(M::AgentOutput(ao)) => {
            if !ao.text.is_empty() {
                out.push(Atom::AssistantText(ao.text.clone()));
            }
        }
        Some(M::AgentReasoning(ar)) => {
            if !ar.reasoning.is_empty() {
                out.push(Atom::AssistantReasoning(ar.reasoning.clone()));
            }
        }
        Some(M::ToolCall(tc)) => match tc.tool.as_ref() {
            Some(tool) => match proto_to_history(tool) {
                Some((tool_kind, input)) => out.push(Atom::AssistantToolUse(ToolUse {
                    tool_use_id: tc.tool_call_id.clone(),
                    tool_kind,
                    input,
                })),
                None => *compat = false,
            },
            None => *compat = false,
        },
        Some(M::ToolCallResult(tcr)) => match tcr.result.as_ref() {
            Some(r) => match message_result_to_text(r) {
                Some((tool_kind, content)) => {
                    let is_error = content.starts_with("ERROR:");
                    out.push(Atom::UserToolResult(ToolResult {
                        tool_use_id: tcr.tool_call_id.clone(),
                        tool_kind,
                        content,
                        is_error,
                    }))
                }
                None => *compat = false,
            },
            None => *compat = false,
        },
        _ => {}
    }
}

fn project_user_input(
    input: &api::request::input::user_inputs::user_input::Input,
    out: &mut Vec<Atom>,
    compat: &mut bool,
    has_user_input: &mut bool,
) {
    use api::request::input::user_inputs::user_input::Input as I;
    match input {
        I::UserQuery(uq) => {
            if !uq.query.is_empty() {
                out.push(Atom::UserText(uq.query.clone()));
                *has_user_input = true;
            }
        }
        I::CliAgentUserQuery(cli) => {
            if let Some(uq) = cli.user_query.as_ref() {
                if !uq.query.is_empty() {
                    out.push(Atom::UserText(uq.query.clone()));
                    *has_user_input = true;
                }
            }
        }
        I::ToolCallResult(tcr) => match tcr.result.as_ref() {
            Some(r) => match request_result_to_text(r) {
                Some((tool_kind, content)) => {
                    let is_error = content.starts_with("ERROR:");
                    out.push(Atom::UserToolResult(ToolResult {
                        tool_use_id: tcr.tool_call_id.clone(),
                        tool_kind,
                        content,
                        is_error,
                    }));
                    // Tool results are the user-side input of the next agent
                    // turn — without this, multi-turn loops bail at `has_user_input`.
                    *has_user_input = true;
                }
                None => *compat = false,
            },
            None => *compat = false,
        },
        _ => *compat = false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user_msg(text: &str) -> api::Message {
        api::Message {
            id: "m".into(),
            task_id: "t".into(),
            request_id: "r".into(),
            timestamp: None,
            server_message_data: String::new(),
            citations: vec![],
            fetched_memories: vec![],
            message: Some(api::message::Message::UserQuery(api::message::UserQuery {
                query: text.into(),
                ..Default::default()
            })),
        }
    }

    fn agent_msg(text: &str) -> api::Message {
        api::Message {
            id: "m".into(),
            task_id: "t".into(),
            request_id: "r".into(),
            timestamp: None,
            server_message_data: String::new(),
            citations: vec![],
            fetched_memories: vec![],
            message: Some(api::message::Message::AgentOutput(api::message::AgentOutput {
                text: text.into(),
            })),
        }
    }

    fn agent_tool_msg(id: &str) -> api::Message {
        let rf = api::message::tool_call::ReadFiles {
            files: vec![api::message::tool_call::read_files::File {
                name: "x".into(),
                line_ranges: vec![],
            }],
        };
        api::Message {
            id: "m".into(),
            task_id: "t".into(),
            request_id: "r".into(),
            timestamp: None,
            server_message_data: String::new(),
            citations: vec![],
            fetched_memories: vec![],
            message: Some(api::message::Message::ToolCall(api::message::ToolCall {
                tool_call_id: id.into(),
                tool: Some(api::message::tool_call::Tool::ReadFiles(rf)),
            })),
        }
    }

    #[test]
    fn empty_request_yields_no_turns() {
        let d = decode(&api::Request::default());
        assert!(d.turns.is_empty());
        assert!(!d.has_user_input);
        assert!(d.history_compatible);
        assert!(d.existing_task_id.is_none());
    }

    #[test]
    fn user_query_groups_with_assistant_text() {
        let req = api::Request {
            task_context: Some(api::request::TaskContext {
                tasks: vec![api::Task {
                    id: "task-1".into(),
                    messages: vec![user_msg("hello"), agent_msg("hi back")],
                    ..Default::default()
                }],
            }),
            ..Default::default()
        };
        let d = decode(&req);
        assert_eq!(d.existing_task_id.as_deref(), Some("task-1"));
        assert!(d.has_user_input);
        assert_eq!(d.turns.len(), 2);
        assert!(matches!(d.turns[0], NormalizedTurn::User { .. }));
        assert!(matches!(d.turns[1], NormalizedTurn::Assistant { .. }));
    }

    #[test]
    fn assistant_text_and_tool_use_coalesce() {
        let req = api::Request {
            task_context: Some(api::request::TaskContext {
                tasks: vec![api::Task {
                    id: "t".into(),
                    messages: vec![user_msg("read it"), agent_msg("here goes"), agent_tool_msg("tc1")],
                    ..Default::default()
                }],
            }),
            ..Default::default()
        };
        let d = decode(&req);
        assert_eq!(d.turns.len(), 2);
        if let NormalizedTurn::Assistant { text, tool_uses, .. } = &d.turns[1] {
            assert_eq!(text.as_deref(), Some("here goes"));
            assert_eq!(tool_uses.len(), 1);
            assert_eq!(tool_uses[0].tool_use_id, "tc1");
        } else {
            panic!("expected grouped Assistant turn");
        }
    }

    #[test]
    fn unsupported_tool_call_triggers_degrade_with_system_note() {
        let req = api::Request {
            task_context: Some(api::request::TaskContext {
                tasks: vec![api::Task {
                    id: "t".into(),
                    messages: vec![
                        user_msg("first"),
                        api::Message {
                            id: "m".into(),
                            task_id: "t".into(),
                            request_id: "r".into(),
                            timestamp: None,
                            server_message_data: String::new(),
                            citations: vec![],
                            fetched_memories: vec![],
                            message: Some(api::message::Message::ToolCall(api::message::ToolCall {
                                tool_call_id: "tc".into(),
                                tool: Some(api::message::tool_call::Tool::SearchCodebase(
                                    api::message::tool_call::SearchCodebase::default(),
                                )),
                            })),
                        },
                        user_msg("after search"),
                    ],
                    ..Default::default()
                }],
            }),
            ..Default::default()
        };
        let d = decode(&req);
        assert!(!d.history_compatible);
        assert_eq!(d.turns.len(), 1);
        if let NormalizedTurn::User { text, .. } = &d.turns[0] {
            let t = text.as_deref().unwrap();
            assert!(t.starts_with("[System:"));
            assert!(t.ends_with("after search"));
        } else {
            panic!("expected User turn");
        }
    }

    #[test]
    fn current_user_input_appended() {
        let req = api::Request {
            input: Some(api::request::Input {
                context: None,
                r#type: Some(api::request::input::Type::UserInputs(
                    api::request::input::UserInputs {
                        inputs: vec![api::request::input::user_inputs::UserInput {
                            input: Some(
                                api::request::input::user_inputs::user_input::Input::UserQuery(
                                    api::request::input::UserQuery {
                                        query: "show me lib.rs".into(),
                                        ..Default::default()
                                    },
                                ),
                            ),
                        }],
                    },
                )),
            }),
            ..Default::default()
        };
        let d = decode(&req);
        assert!(d.has_user_input);
        assert_eq!(d.turns.len(), 1);
        if let NormalizedTurn::User { text, .. } = &d.turns[0] {
            assert_eq!(text.as_deref(), Some("show me lib.rs"));
        }
    }

    #[test]
    fn multiple_tool_results_group_into_one_user_turn() {
        let req = api::Request {
            input: Some(api::request::Input {
                context: None,
                r#type: Some(api::request::input::Type::UserInputs(
                    api::request::input::UserInputs {
                        inputs: vec![
                            api::request::input::user_inputs::UserInput {
                                input: Some(
                                    api::request::input::user_inputs::user_input::Input::ToolCallResult(
                                        api::request::input::ToolCallResult {
                                            tool_call_id: "t1".into(),
                                            result: Some(api::request::input::tool_call_result::Result::ReadFiles(
                                                api::ReadFilesResult { result: None },
                                            )),
                                        },
                                    ),
                                ),
                            },
                            api::request::input::user_inputs::UserInput {
                                input: Some(
                                    api::request::input::user_inputs::user_input::Input::ToolCallResult(
                                        api::request::input::ToolCallResult {
                                            tool_call_id: "t2".into(),
                                            result: Some(api::request::input::tool_call_result::Result::ReadFiles(
                                                api::ReadFilesResult { result: None },
                                            )),
                                        },
                                    ),
                                ),
                            },
                        ],
                    },
                )),
            }),
            task_context: Some(api::request::TaskContext {
                tasks: vec![api::Task {
                    id: "t".into(),
                    messages: vec![user_msg("seed")],
                    ..Default::default()
                }],
            }),
            ..Default::default()
        };
        let d = decode(&req);
        // Last turn should be one user turn carrying TWO tool results.
        let last = d.turns.last().unwrap();
        if let NormalizedTurn::User { tool_results, .. } = last {
            assert_eq!(tool_results.len(), 2);
            assert_eq!(tool_results[0].tool_use_id, "t1");
            assert_eq!(tool_results[1].tool_use_id, "t2");
        } else {
            panic!("expected last turn to be User");
        }
    }
}
