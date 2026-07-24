pub mod settings;
mod stack;

use warpui::AppContext;
use warpui::keymap::{BindingDescription, EditableBinding};

pub use self::settings::UndoCloseSettings;
pub use self::stack::{UndoCloseStack, UndoCloseStackEvent};
use crate::util::bindings::{BindingDescriptionFluentExt, CustomAction};
use crate::workspace::WorkspaceAction;

/// Register keybindings for undo close functionality.
pub fn init(ctx: &mut AppContext) {
    use warpui::keymap::macros::*;

    ctx.register_editable_bindings([EditableBinding::new(
        "app:reopen_closed_session",
        BindingDescription::fluent("binding-undo-close-reopen"),
        // Trigger ReopenClosedSession on the active workspace when
        // the action is taken from the command palette.
        WorkspaceAction::ReopenClosedSession,
    )
    .with_custom_action(CustomAction::ReopenClosedSession)
    // Scope to the GUI `Workspace` context so this binding doesn't leak into the
    // headless TUI's keymap contexts (mirrors the sibling `workspace:*` bindings).
    .with_context_predicate(id!("Workspace"))]);
}
