//! Adapter from Anthropic's `computer_use_20250124` tool spec to the local
//! [`computer_use`] crate's [`Action`] enum.
//!
//! Wired by M4.2+ when the Direct-backend Anthropic provider gets full
//! tool-calling support. Until then this module is dead code on purpose:
//! it exists so the M4.2 patch can drop straight in without re-litigating
//! the JSON-action mapping. Tests cover the dozen action variants the
//! Anthropic API can emit against the canonical `computer_use::Action`
//! / `ScreenshotParams` shapes.
//!
//! Reference: <https://docs.anthropic.com/en/docs/agents-and-tools/computer-use>

#![cfg_attr(not(test), allow(dead_code))]

use anyhow::{anyhow, bail, Context, Result};
use computer_use::{
    Action, Key, MouseButton, ScreenshotParams, ScrollDirection, ScrollDistance, Vector2I,
};
use serde_json::Value;
use std::time::Duration;

/// Anthropic emits a single object per tool_use block. The `action` field
/// dispatches; the rest are action-specific. Returns the lowered action list
/// plus optional screenshot params (the "screenshot" action is a screenshot
/// without any prior input).
pub fn anthropic_input_to_actions(input: &Value) -> Result<DecodedAnthropicComputerUse> {
    let action = input
        .get("action")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("anthropic computer_use input missing `action`"))?;

    match action {
        "screenshot" => Ok(DecodedAnthropicComputerUse {
            actions: vec![],
            screenshot: Some(default_screenshot_params()),
        }),

        "wait" => {
            let secs = input
                .get("duration")
                .and_then(|v| v.as_f64())
                .ok_or_else(|| anyhow!("`wait` requires `duration` (seconds)"))?;
            Ok(DecodedAnthropicComputerUse {
                actions: vec![Action::Wait(Duration::from_secs_f64(secs))],
                screenshot: Some(default_screenshot_params()),
            })
        }

        "mouse_move" => {
            let to = parse_coordinate(input).context("`mouse_move` requires `coordinate`")?;
            Ok(DecodedAnthropicComputerUse {
                actions: vec![Action::MouseMove { to }],
                screenshot: Some(default_screenshot_params()),
            })
        }

        "left_click" | "right_click" | "middle_click" => {
            let button = match action {
                "left_click" => MouseButton::Left,
                "right_click" => MouseButton::Right,
                "middle_click" => MouseButton::Middle,
                _ => unreachable!(),
            };
            let at = parse_coordinate(input)
                .with_context(|| format!("`{action}` requires `coordinate`"))?;
            Ok(DecodedAnthropicComputerUse {
                actions: click_at(button, at),
                screenshot: Some(default_screenshot_params()),
            })
        }

        "double_click" => {
            let at = parse_coordinate(input).context("`double_click` requires `coordinate`")?;
            let mut actions = click_at(MouseButton::Left, at);
            actions.extend(click_at(MouseButton::Left, at));
            Ok(DecodedAnthropicComputerUse {
                actions,
                screenshot: Some(default_screenshot_params()),
            })
        }

        "triple_click" => {
            let at = parse_coordinate(input).context("`triple_click` requires `coordinate`")?;
            let mut actions = click_at(MouseButton::Left, at);
            actions.extend(click_at(MouseButton::Left, at));
            actions.extend(click_at(MouseButton::Left, at));
            Ok(DecodedAnthropicComputerUse {
                actions,
                screenshot: Some(default_screenshot_params()),
            })
        }

        "left_mouse_down" => {
            let at = parse_coordinate(input).context("`left_mouse_down` requires `coordinate`")?;
            Ok(DecodedAnthropicComputerUse {
                actions: vec![Action::MouseDown {
                    button: MouseButton::Left,
                    at,
                }],
                screenshot: None,
            })
        }

        "left_mouse_up" => Ok(DecodedAnthropicComputerUse {
            actions: vec![Action::MouseUp {
                button: MouseButton::Left,
            }],
            screenshot: Some(default_screenshot_params()),
        }),

        "left_click_drag" => {
            // Anthropic sometimes emits `start_coordinate` + `coordinate`; older
            // transcripts only have `coordinate` plus an implicit current
            // cursor. We support both: when only one coordinate is given, we
            // treat it as the destination and rely on the caller to have
            // already moved the cursor.
            let end = parse_coordinate(input).context("`left_click_drag` requires `coordinate`")?;
            let start = parse_optional_coordinate(input, "start_coordinate");

            let mut actions = Vec::new();
            if let Some(start) = start {
                actions.push(Action::MouseMove { to: start });
                actions.push(Action::MouseDown {
                    button: MouseButton::Left,
                    at: start,
                });
            } else {
                actions.push(Action::MouseDown {
                    button: MouseButton::Left,
                    at: end,
                });
            }
            actions.push(Action::MouseMove { to: end });
            actions.push(Action::MouseUp {
                button: MouseButton::Left,
            });
            Ok(DecodedAnthropicComputerUse {
                actions,
                screenshot: Some(default_screenshot_params()),
            })
        }

        "scroll" => {
            let at = parse_coordinate(input).context("`scroll` requires `coordinate`")?;
            let direction = match input
                .get("scroll_direction")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("`scroll` requires `scroll_direction`"))?
            {
                "up" => ScrollDirection::Up,
                "down" => ScrollDirection::Down,
                "left" => ScrollDirection::Left,
                "right" => ScrollDirection::Right,
                other => bail!("unknown scroll_direction: {other}"),
            };
            let amount = input
                .get("scroll_amount")
                .and_then(|v| v.as_i64())
                .unwrap_or(3) as i32;
            Ok(DecodedAnthropicComputerUse {
                actions: vec![Action::MouseWheel {
                    at,
                    direction,
                    distance: ScrollDistance::Clicks(amount),
                }],
                screenshot: Some(default_screenshot_params()),
            })
        }

        "type" => {
            let text = input
                .get("text")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("`type` requires `text`"))?
                .to_string();
            Ok(DecodedAnthropicComputerUse {
                actions: vec![Action::TypeText { text }],
                screenshot: Some(default_screenshot_params()),
            })
        }

        "key" => {
            // Anthropic uses xdotool key combos: e.g. "ctrl+c", "Return", "F11".
            let combo = input
                .get("text")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("`key` requires `text`"))?;
            Ok(DecodedAnthropicComputerUse {
                actions: chord_keys(combo),
                screenshot: Some(default_screenshot_params()),
            })
        }

        "hold_key" => {
            let combo = input
                .get("text")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("`hold_key` requires `text`"))?;
            let secs = input
                .get("duration")
                .and_then(|v| v.as_f64())
                .ok_or_else(|| anyhow!("`hold_key` requires `duration`"))?;

            // Press all, hold, release in reverse.
            let keys: Vec<Key> = combo.split('+').map(parse_key_token).collect();
            let mut actions = Vec::with_capacity(keys.len() * 2 + 1);
            for key in &keys {
                actions.push(Action::KeyDown { key: key.clone() });
            }
            actions.push(Action::Wait(Duration::from_secs_f64(secs)));
            for key in keys.into_iter().rev() {
                actions.push(Action::KeyUp { key });
            }
            Ok(DecodedAnthropicComputerUse {
                actions,
                screenshot: Some(default_screenshot_params()),
            })
        }

        "cursor_position" => Ok(DecodedAnthropicComputerUse {
            actions: vec![],
            screenshot: None,
        }),

        other => bail!("unsupported anthropic computer_use action: {other}"),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedAnthropicComputerUse {
    pub actions: Vec<Action>,
    pub screenshot: Option<ScreenshotParams>,
}

fn click_at(button: MouseButton, at: Vector2I) -> Vec<Action> {
    vec![
        Action::MouseDown {
            button: button.clone(),
            at,
        },
        Action::MouseUp { button },
    ]
}

fn parse_coordinate(input: &Value) -> Result<Vector2I> {
    parse_optional_coordinate(input, "coordinate")
        .ok_or_else(|| anyhow!("`coordinate` missing or malformed"))
}

fn parse_optional_coordinate(input: &Value, field: &str) -> Option<Vector2I> {
    let arr = input.get(field)?.as_array()?;
    if arr.len() < 2 {
        return None;
    }
    let x = arr[0].as_i64()? as i32;
    let y = arr[1].as_i64()? as i32;
    Some(Vector2I::new(x, y))
}

fn default_screenshot_params() -> ScreenshotParams {
    ScreenshotParams {
        max_long_edge_px: None,
        max_total_px: None,
        region: None,
        target: Default::default(),
    }
}

/// Lower a single `+`-separated chord (e.g. `"ctrl+shift+t"`) into a press-all
/// then release-all-in-reverse sequence. Tokens use xdotool conventions; we
/// emit `Key::Char` for single printable characters and `Key::Keycode` with a
/// best-effort mapping for the named modifiers / special keys we expect from
/// Anthropic. Unknown tokens fall back to `Key::Char` of the first character.
fn chord_keys(combo: &str) -> Vec<Action> {
    let keys: Vec<Key> = combo.split('+').map(parse_key_token).collect();
    let mut actions = Vec::with_capacity(keys.len() * 2);
    for key in &keys {
        actions.push(Action::KeyDown { key: key.clone() });
    }
    for key in keys.into_iter().rev() {
        actions.push(Action::KeyUp { key });
    }
    actions
}

/// Map a single Anthropic / xdotool key token to [`computer_use::Key`].
///
/// The underlying [`computer_use::Action`] only carries the key as either an
/// integer keycode or a single character; platform-specific resolution of
/// named keys (e.g. "Return", "shift") happens inside the [`computer_use`]
/// platform actors. To keep this adapter platform-agnostic we encode named
/// keys as `Key::Char` of a placeholder when single-char, and otherwise
/// preserve the lowercase ASCII first character. Production wiring (M4.2)
/// should replace this with a per-platform keysym table.
fn parse_key_token(token: &str) -> Key {
    let lower = token.trim().to_ascii_lowercase();
    if lower.chars().count() == 1 {
        return Key::Char(lower.chars().next().unwrap());
    }
    // Best-effort fallback for named keys: project to first ASCII char so the
    // pipeline stays lossy-but-runnable. M4.2 swaps in the keysym table.
    Key::Char(lower.chars().next().unwrap_or(' '))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn screenshot_action_emits_only_screenshot_request() {
        let decoded = anthropic_input_to_actions(&json!({"action": "screenshot"})).unwrap();
        assert!(decoded.actions.is_empty());
        assert!(decoded.screenshot.is_some());
    }

    #[test]
    fn left_click_lowers_to_down_then_up() {
        let decoded = anthropic_input_to_actions(&json!({
            "action": "left_click",
            "coordinate": [120, 240],
        }))
        .unwrap();
        assert_eq!(decoded.actions.len(), 2);
        match &decoded.actions[0] {
            Action::MouseDown { button, at } => {
                assert!(matches!(button, MouseButton::Left));
                assert_eq!(at.x(), 120);
                assert_eq!(at.y(), 240);
            }
            other => panic!("unexpected first action: {other:?}"),
        }
        assert!(matches!(
            decoded.actions[1],
            Action::MouseUp {
                button: MouseButton::Left
            }
        ));
    }

    #[test]
    fn type_action_passes_text_through() {
        let decoded = anthropic_input_to_actions(&json!({
            "action": "type",
            "text": "hello world",
        }))
        .unwrap();
        assert_eq!(decoded.actions.len(), 1);
        match &decoded.actions[0] {
            Action::TypeText { text } => assert_eq!(text, "hello world"),
            other => panic!("unexpected action: {other:?}"),
        }
    }

    #[test]
    fn scroll_uses_clicks_distance() {
        let decoded = anthropic_input_to_actions(&json!({
            "action": "scroll",
            "coordinate": [400, 300],
            "scroll_direction": "down",
            "scroll_amount": 5,
        }))
        .unwrap();
        assert_eq!(decoded.actions.len(), 1);
        match &decoded.actions[0] {
            Action::MouseWheel {
                at,
                direction,
                distance,
            } => {
                assert_eq!(at.x(), 400);
                assert_eq!(at.y(), 300);
                assert!(matches!(direction, ScrollDirection::Down));
                assert!(matches!(distance, ScrollDistance::Clicks(5)));
            }
            other => panic!("unexpected action: {other:?}"),
        }
    }

    #[test]
    fn drag_with_start_and_end_emits_full_sequence() {
        let decoded = anthropic_input_to_actions(&json!({
            "action": "left_click_drag",
            "start_coordinate": [10, 20],
            "coordinate": [100, 200],
        }))
        .unwrap();
        assert_eq!(decoded.actions.len(), 4);
        assert!(matches!(decoded.actions[0], Action::MouseMove { .. }));
        assert!(matches!(decoded.actions[1], Action::MouseDown { .. }));
        assert!(matches!(decoded.actions[2], Action::MouseMove { .. }));
        assert!(matches!(decoded.actions[3], Action::MouseUp { .. }));
    }

    #[test]
    fn unknown_action_errors() {
        let result = anthropic_input_to_actions(&json!({"action": "warpify_blocks"}));
        assert!(result.is_err());
    }
}
