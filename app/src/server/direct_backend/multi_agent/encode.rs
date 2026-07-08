//! `DriverOutput` → `Vec<api::ResponseEvent>` projection.
//!
//! Emits one of two patterns:
//!   1. `[Init]` (immediate, before driver call) — emitted from `mod.rs::run`
//!      so the UI can scaffold a chat bubble while the LLM is still working.
//!   2. `[ClientActions{[CreateTask?, AddMessagesToTask]}, Finished{Done|InternalError}]`
//!      after the driver returns.
//!
//! When `existing_task_id` is provided we skip CreateTask and AddMessagesToTask
//! straight onto the existing root task — re-creating a server-backed task
//! breaks the client's `apply_client_action` upgrade path on turn 2+.

use uuid::Uuid;
use warp_multi_agent_api as api;

use super::{DecodedBlock, DriverOutput, DriverStreamChunk};

pub fn build_init_event(conversation_id: String) -> (api::ResponseEvent, String) {
    let request_id = Uuid::new_v4().to_string();
    let init = api::ResponseEvent {
        r#type: Some(api::response_event::Type::Init(api::response_event::StreamInit {
            conversation_id,
            request_id: request_id.clone(),
            run_id: String::new(),
        })),
    };
    (init, request_id)
}

pub fn build_success_actions_and_finished(
    out: DriverOutput,
    request_id: String,
    existing_task_id: Option<String>,
) -> Vec<api::ResponseEvent> {
    let needs_create_task = existing_task_id.is_none();
    let task_id = existing_task_id.unwrap_or_else(|| Uuid::new_v4().to_string());

    let mut messages: Vec<api::Message> = Vec::with_capacity(out.blocks.len());
    for block in out.blocks {
        let msg_id = Uuid::new_v4().to_string();
        match block {
            DecodedBlock::Text(text) => {
                messages.push(api::Message {
        fetched_memories: vec![],
                    id: msg_id,
                    task_id: task_id.clone(),
                    request_id: request_id.clone(),
                    timestamp: None,
                    server_message_data: String::new(),
                    citations: vec![],
                    message: Some(api::message::Message::AgentOutput(api::message::AgentOutput {
                        text,
                    })),
                });
            }
            DecodedBlock::ToolUse { tool_use_id, tool } => {
                messages.push(api::Message {
        fetched_memories: vec![],
                    id: msg_id,
                    task_id: task_id.clone(),
                    request_id: request_id.clone(),
                    timestamp: None,
                    server_message_data: String::new(),
                    citations: vec![],
                    message: Some(api::message::Message::ToolCall(api::message::ToolCall {
                        tool_call_id: tool_use_id,
                        tool: Some(tool),
                    })),
                });
            }
        }
    }

    let mut actions: Vec<api::ClientAction> = Vec::with_capacity(2);
    if needs_create_task && !messages.is_empty() {
        // Reserved for first-turn flow when no task exists; we only enter
        // this branch with a fresh task_id.
        actions.push(wrap_action(api::client_action::Action::CreateTask(
            api::client_action::CreateTask {
                task: Some(api::Task {
                    id: task_id.clone(),
                    description: String::new(),
                    dependencies: None,
                    messages: vec![],
                    summary: String::new(),
                    server_data: String::new(),
                }),
            },
        )));
    }
    if !messages.is_empty() {
        actions.push(wrap_action(api::client_action::Action::AddMessagesToTask(
            api::client_action::AddMessagesToTask { task_id, messages },
        )));
    }

    let mut events = Vec::with_capacity(2);
    if !actions.is_empty() {
        events.push(api::ResponseEvent {
            r#type: Some(api::response_event::Type::ClientActions(
                api::response_event::ClientActions { actions },
            )),
        });
    }
    events.push(api::ResponseEvent {
        r#type: Some(api::response_event::Type::Finished(api::response_event::StreamFinished {
            reason: Some(api::response_event::stream_finished::Reason::Done(
                api::response_event::stream_finished::Done {},
            )),
            ..Default::default()
        })),
    });
    events
}

pub fn build_finished_error(message: String) -> api::ResponseEvent {
    api::ResponseEvent {
        r#type: Some(api::response_event::Type::Finished(api::response_event::StreamFinished {
            reason: Some(api::response_event::stream_finished::Reason::InternalError(
                api::response_event::stream_finished::InternalError { message },
            )),
            ..Default::default()
        })),
    }
}

fn wrap_action(action: api::client_action::Action) -> api::ClientAction {
    api::ClientAction {
        action: Some(action),
    }
}

// ─── Streaming helpers (M4.2.5) ─────────────────────────────────────────────

/// Emit a `CreateTask` action ahead of the first delta so the UI can scaffold
/// a chat bubble with a stable task_id before the LLM produces any tokens.
pub fn build_create_task_action(task_id: String) -> api::ResponseEvent {
    api::ResponseEvent {
        r#type: Some(api::response_event::Type::ClientActions(
            api::response_event::ClientActions {
                actions: vec![wrap_action(api::client_action::Action::CreateTask(
                    api::client_action::CreateTask {
                        task: Some(api::Task {
                            id: task_id,
                            description: String::new(),
                            dependencies: None,
                            messages: vec![],
                            summary: String::new(),
                            server_data: String::new(),
                        }),
                    },
                ))],
            },
        )),
    }
}

/// First text delta on a block: `[AddMessagesToTask{empty AgentOutput}, AppendToMessageContent{first delta}]`
/// in one event. Done in one ClientActions so the consumer (`conversation.rs`)
/// processes Add before Append within the same atomic event.
pub fn build_add_then_append_text(
    task_id: &str,
    request_id: &str,
    msg_id: &str,
    delta: &str,
) -> api::ResponseEvent {
    let empty_msg = api::Message {
        fetched_memories: vec![],
        id: msg_id.to_string(),
        task_id: task_id.into(),
        request_id: request_id.into(),
        timestamp: None,
        server_message_data: String::new(),
        citations: vec![],
        message: Some(api::message::Message::AgentOutput(
            api::message::AgentOutput {
                text: String::new(),
            },
        )),
    };
    let delta_msg = api::Message {
        fetched_memories: vec![],
        id: msg_id.to_string(),
        task_id: task_id.into(),
        request_id: request_id.into(),
        timestamp: None,
        server_message_data: String::new(),
        citations: vec![],
        message: Some(api::message::Message::AgentOutput(
            api::message::AgentOutput {
                text: delta.to_string(),
            },
        )),
    };
    // FieldMask path: prost-reflect's `get_field_by_name` doesn't surface
    // oneof container names (`message` here), so the leading "message." segment
    // is silently ignored and the append is no-op'd. The first segment must be
    // the actual proto field name of the oneof variant (`agent_output`).
    let mask = prost_types::FieldMask {
        paths: vec!["agent_output.text".into()],
    };
    api::ResponseEvent {
        r#type: Some(api::response_event::Type::ClientActions(
            api::response_event::ClientActions {
                actions: vec![
                    wrap_action(api::client_action::Action::AddMessagesToTask(
                        api::client_action::AddMessagesToTask {
                            task_id: task_id.into(),
                            messages: vec![empty_msg],
                        },
                    )),
                    wrap_action(api::client_action::Action::AppendToMessageContent(
                        api::client_action::AppendToMessageContent {
                            task_id: task_id.into(),
                            message: Some(delta_msg),
                            mask: Some(mask),
                        },
                    )),
                ],
            },
        )),
    }
}

/// Reasoning analogue of [`build_add_then_append_text`]. Produces an
/// `AgentReasoning` message bubble (rendered as a foldable thinking block)
/// for chain-of-thought tokens from DeepSeek-R1 / o1-style models.
pub fn build_add_then_append_reasoning(
    task_id: &str,
    request_id: &str,
    msg_id: &str,
    delta: &str,
) -> api::ResponseEvent {
    let empty_msg = api::Message {
        fetched_memories: vec![],
        id: msg_id.to_string(),
        task_id: task_id.into(),
        request_id: request_id.into(),
        timestamp: None,
        server_message_data: String::new(),
        citations: vec![],
        message: Some(api::message::Message::AgentReasoning(
            api::message::AgentReasoning {
                reasoning: String::new(),
                finished_duration: None,
            },
        )),
    };
    let delta_msg = api::Message {
        fetched_memories: vec![],
        id: msg_id.to_string(),
        task_id: task_id.into(),
        request_id: request_id.into(),
        timestamp: None,
        server_message_data: String::new(),
        citations: vec![],
        message: Some(api::message::Message::AgentReasoning(
            api::message::AgentReasoning {
                reasoning: delta.to_string(),
                finished_duration: None,
            },
        )),
    };
    let mask = prost_types::FieldMask {
        paths: vec!["agent_reasoning.reasoning".into()],
    };
    api::ResponseEvent {
        r#type: Some(api::response_event::Type::ClientActions(
            api::response_event::ClientActions {
                actions: vec![
                    wrap_action(api::client_action::Action::AddMessagesToTask(
                        api::client_action::AddMessagesToTask {
                            task_id: task_id.into(),
                            messages: vec![empty_msg],
                        },
                    )),
                    wrap_action(api::client_action::Action::AppendToMessageContent(
                        api::client_action::AppendToMessageContent {
                            task_id: task_id.into(),
                            message: Some(delta_msg),
                            mask: Some(mask),
                        },
                    )),
                ],
            },
        )),
    }
}

pub fn build_append_to_reasoning(task_id: &str, msg_id: &str, delta: &str) -> api::ResponseEvent {
    let delta_msg = api::Message {
        fetched_memories: vec![],
        id: msg_id.to_string(),
        task_id: task_id.into(),
        request_id: String::new(),
        timestamp: None,
        server_message_data: String::new(),
        citations: vec![],
        message: Some(api::message::Message::AgentReasoning(
            api::message::AgentReasoning {
                reasoning: delta.to_string(),
                finished_duration: None,
            },
        )),
    };
    let mask = prost_types::FieldMask {
        paths: vec!["agent_reasoning.reasoning".into()],
    };
    api::ResponseEvent {
        r#type: Some(api::response_event::Type::ClientActions(
            api::response_event::ClientActions {
                actions: vec![wrap_action(
                    api::client_action::Action::AppendToMessageContent(
                        api::client_action::AppendToMessageContent {
                            task_id: task_id.into(),
                            message: Some(delta_msg),
                            mask: Some(mask),
                        },
                    ),
                )],
            },
        )),
    }
}

pub fn build_append_to_text(task_id: &str, msg_id: &str, delta: &str) -> api::ResponseEvent {
    let delta_msg = api::Message {
        fetched_memories: vec![],
        id: msg_id.to_string(),
        task_id: task_id.into(),
        request_id: String::new(),
        timestamp: None,
        server_message_data: String::new(),
        citations: vec![],
        message: Some(api::message::Message::AgentOutput(
            api::message::AgentOutput {
                text: delta.to_string(),
            },
        )),
    };
    // FieldMask path: prost-reflect's `get_field_by_name` doesn't surface
    // oneof container names (`message` here), so the leading "message." segment
    // is silently ignored and the append is no-op'd. The first segment must be
    // the actual proto field name of the oneof variant (`agent_output`).
    let mask = prost_types::FieldMask {
        paths: vec!["agent_output.text".into()],
    };
    api::ResponseEvent {
        r#type: Some(api::response_event::Type::ClientActions(
            api::response_event::ClientActions {
                actions: vec![wrap_action(
                    api::client_action::Action::AppendToMessageContent(
                        api::client_action::AppendToMessageContent {
                            task_id: task_id.into(),
                            message: Some(delta_msg),
                            mask: Some(mask),
                        },
                    ),
                )],
            },
        )),
    }
}

pub fn build_tool_call_message_action(
    task_id: &str,
    request_id: &str,
    tool_use_id: &str,
    tool: api::message::tool_call::Tool,
) -> api::ResponseEvent {
    let msg = api::Message {
        fetched_memories: vec![],
        id: Uuid::new_v4().to_string(),
        task_id: task_id.into(),
        request_id: request_id.into(),
        timestamp: None,
        server_message_data: String::new(),
        citations: vec![],
        message: Some(api::message::Message::ToolCall(api::message::ToolCall {
            tool_call_id: tool_use_id.to_string(),
            tool: Some(tool),
        })),
    };
    api::ResponseEvent {
        r#type: Some(api::response_event::Type::ClientActions(
            api::response_event::ClientActions {
                actions: vec![wrap_action(api::client_action::Action::AddMessagesToTask(
                    api::client_action::AddMessagesToTask {
                        task_id: task_id.into(),
                        messages: vec![msg],
                    },
                ))],
            },
        )),
    }
}

/// Emit a one-shot `AgentOutput` text message inline (used for soft tool errors
/// surfaced mid-stream so the model can self-correct without crashing the run).
pub fn build_inline_text_message(
    task_id: &str,
    request_id: &str,
    text: &str,
) -> api::ResponseEvent {
    let msg = api::Message {
        fetched_memories: vec![],
        id: Uuid::new_v4().to_string(),
        task_id: task_id.into(),
        request_id: request_id.into(),
        timestamp: None,
        server_message_data: String::new(),
        citations: vec![],
        message: Some(api::message::Message::AgentOutput(
            api::message::AgentOutput {
                text: text.to_string(),
            },
        )),
    };
    api::ResponseEvent {
        r#type: Some(api::response_event::Type::ClientActions(
            api::response_event::ClientActions {
                actions: vec![wrap_action(api::client_action::Action::AddMessagesToTask(
                    api::client_action::AddMessagesToTask {
                        task_id: task_id.into(),
                        messages: vec![msg],
                    },
                ))],
            },
        )),
    }
}

/// Build the terminal `Finished{Done}` event. When a `Stop` chunk carries any
/// usage data, populates `token_usage[0]`; otherwise leaves it empty so the
/// UI can distinguish "0 tokens charged" from "provider didn't report usage".
pub fn build_finished_from_stop(stop: Option<DriverStreamChunk>) -> api::ResponseEvent {
    let token_usage = match &stop {
        Some(DriverStreamChunk::Stop {
            input_tokens,
            output_tokens,
            input_cache_read,
            model_id,
            ..
        }) if input_tokens.is_some()
            || output_tokens.is_some()
            || input_cache_read.is_some() =>
        {
            vec![api::response_event::stream_finished::TokenUsage {
                model_id: model_id.clone(),
                total_input: input_tokens.unwrap_or(0),
                output: output_tokens.unwrap_or(0),
                input_cache_read: input_cache_read.unwrap_or(0),
                ..Default::default()
            }]
        }
        _ => vec![],
    };
    api::ResponseEvent {
        r#type: Some(api::response_event::Type::Finished(
            api::response_event::StreamFinished {
                reason: Some(api::response_event::stream_finished::Reason::Done(
                    api::response_event::stream_finished::Done {},
                )),
                token_usage,
                ..Default::default()
            },
        )),
    }
}

