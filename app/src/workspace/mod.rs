mod action;
mod active_session;
pub(crate) mod auto_handoff;
pub mod bonus_grant_notification_model;
#[cfg(target_os = "macos")]
pub(crate) mod cli_install;
mod close_session_confirmation_dialog;
pub(crate) mod cross_window_tab_drag;
pub mod delete_conversation_confirmation_dialog;
mod global_actions;
pub mod header_toolbar_editor;
pub mod header_toolbar_item;
pub mod hoa_onboarding;
mod home;
mod lightbox_view;
mod native_modal;
mod one_time_modal_model;
mod registry;
pub mod rewind_confirmation_dialog;
pub mod sync_inputs;
pub mod tab_group;
pub mod tab_settings;
mod toast_stack;
pub mod util;
pub mod view;

pub use action::{
    AutoCloudHandoffTrigger, CommandSearchOptions, InitContent, RestoreConversationLayout,
    TabContextMenuAnchor, VerticalTabsPaneContextMenuTarget, WorkspaceAction,
};
pub use active_session::ActiveSession;
pub use global_actions::{
    ForkAIConversationParams, ForkFromExchange, ForkedConversationDestination,
};
use serde::{Deserialize, Serialize};
pub use util::{PaneViewLocator, TabMovement, active_terminal_in_window};
pub use view::{
    NEW_SESSION_MENU_BUTTON_POSITION_ID, NEW_TAB_BUTTON_POSITION_ID, PANEL_HEADER_HEIGHT,
    TAB_BAR_HEIGHT, TOTAL_TAB_BAR_HEIGHT, WORKSPACE_PADDING, Workspace,
};
use warp_core::context_flag::ContextFlag;
use warp_i18n::t;
use warpui::AppContext;
use warpui::SingletonEntity;
use warpui::accessibility::AccessibilityVerbosity;
use warpui::elements::DropTargetData;
use warpui::keymap::{BindingDescription, EditableBinding, FixedBinding};

use crate::ai::blocklist::NEW_AGENT_PANE_LABEL;
use crate::ai::skills::SkillManager;
use crate::ai::AIRequestUsageModel;
use crate::channel::{Channel, ChannelState};
use crate::features::FeatureFlag;
use crate::palette::PaletteMode;
use crate::server::telemetry::{AgentModeEntrypoint, PaletteSource};
use crate::settings::AISettings;
use crate::settings_view::{self, SettingsSection, flags};
use crate::tab::{NewSessionMenuItem, uses_vertical_tabs};
use crate::util::bindings::{
    self, BindingDescriptionFluentExt, CustomAction, cmd_or_ctrl_shift, is_binding_pty_compliant,
};
use crate::{code, modal, notebooks, tab_configs};

// Helper function to access panel header corner radius from other modules
pub fn panel_header_corner_radius() -> warpui::elements::CornerRadius {
    warpui::elements::CornerRadius::with_top(warpui::elements::Radius::Pixels(8.))
}

/// Returns `true` when `WorkspaceAction::SendFeedback` will launch the guided
/// feedback skill in a new agent pane. When `false`, the action falls back to
/// opening the GitHub issue form in the browser.
///
/// Kept in sync with the availability check in `Workspace::send_feedback` so
/// the command palette label and the menu item behavior never diverge.
pub fn is_feedback_skill_available(ctx: &AppContext) -> bool {
    AISettings::as_ref(ctx).is_any_ai_enabled(ctx)
        && AIRequestUsageModel::as_ref(ctx).has_any_ai_remaining(ctx)
        && SkillManager::as_ref(ctx)
            .active_local_bundled_skill("feedback", ctx)
            .is_some()
}

pub use one_time_modal_model::OneTimeModalModel;
pub use registry::WorkspaceRegistry;
pub use toast_stack::{ToastStack, ToastStackEvent};

use crate::workspace::view::{
    LEFT_PANEL_AGENT_CONVERSATIONS_BINDING_NAME, LEFT_PANEL_GLOBAL_SEARCH_BINDING_NAME,
    LEFT_PANEL_PROJECT_EXPLORER_BINDING_NAME, LEFT_PANEL_WARP_DRIVE_BINDING_NAME,
    NEW_AGENT_TAB_BINDING_NAME, NEW_AMBIENT_AGENT_TAB_BINDING_NAME, NEW_FILE_BINDING_NAME,
    NEW_TAB_BINDING_NAME, NEW_TERMINAL_TAB_BINDING_NAME, OPEN_GLOBAL_SEARCH_BINDING_NAME,
    TOGGLE_CONVERSATION_LIST_VIEW_BINDING_NAME, TOGGLE_NOTIFICATION_MAILBOX_BINDING_NAME,
    TOGGLE_PROJECT_EXPLORER_BINDING_NAME, TOGGLE_RIGHT_PANEL_BINDING_NAME,
    TOGGLE_TAB_CONFIGS_MENU_BINDING_NAME, TOGGLE_VERTICAL_TABS_PANEL_BINDING_NAME,
    TOGGLE_WARP_DRIVE_BINDING_NAME,
};

pub fn init(app: &mut AppContext) {
    app.add_singleton_model(|_| WorkspaceRegistry::new());
    app.add_singleton_model(|_| cross_window_tab_drag::CrossWindowTabDrag::new());
    use warpui::keymap::macros::*;
    app.register_binding_validator::<Workspace>(is_binding_pty_compliant);

    modal::init(app);
    native_modal::init(app);
    lightbox_view::init(app);
    rewind_confirmation_dialog::init(app);
    delete_conversation_confirmation_dialog::init(app);
    crate::tab_configs::remove_confirmation_dialog::init(app);
    hoa_onboarding::init(app);
    tab_configs::session_config_modal::init(app);
    view::launch_modal::oz_launch::init(app);
    view::openwarp_launch_modal::init(app);
    view::orchestration_launch_modal::init(app);
    view::agent_cli_launch_modal::init(app);
    view::feature_intro_modal::init(app);
    view::auto_handoff_sleep_modal::init(app);
    view::cloud_agent_capacity_modal::init(app);
    view::codex_modal::init(app);
    view::free_ai_removal_modal::init(app);
    view::global_search::view::GlobalSearchView::init(app);
    view::right_panel::RightPanelView::init(app);
    header_toolbar_editor::init(app);
    view::conversation_list::view::register_conversation_list_view_bindings(app);

    settings_view::init_actions_from_parent_view(app, &id!("Workspace"), |settings_action| {
        WorkspaceAction::DispatchToSettingsTab(settings_action)
    });
    global_actions::init_global_actions(app);
    notebooks::init(app);
    code::init(app);
    sync_inputs::init(app);
    lsp::init(app);

    app.register_fixed_bindings([FixedBinding::empty(
        BindingDescription::fluent("binding-workspace-dump-debug-info"),
        WorkspaceAction::DumpDebugInfo,
        id!("Workspace"),
    )]);
    app.register_fixed_bindings([
        FixedBinding::new(
            "escape",
            WorkspaceAction::DismissSessionConfigTabConfigChip,
            id!("Workspace") & id!(flags::SESSION_CONFIG_TAB_CONFIG_CHIP_OPEN),
        ),
        FixedBinding::new(
            "enter",
            WorkspaceAction::DismissSessionConfigTabConfigChip,
            id!("Workspace") & id!(flags::SESSION_CONFIG_TAB_CONFIG_CHIP_OPEN),
        ),
        // Feature intro never steals focus, so Escape must be handled at the workspace
        // level while the popover is open rather than on FeatureIntroModal itself.
        FixedBinding::new(
            "escape",
            WorkspaceAction::DismissFeatureIntroModal,
            id!("Workspace") & id!(flags::FEATURE_INTRO_MODAL_OPEN),
        ),
    ]);

    if ChannelState::enable_debug_features() {
        let crash_description = if cfg!(target_os = "macos") {
            BindingDescription::fluent("binding-workspace-crash-mac")
        } else {
            BindingDescription::fluent("binding-workspace-crash-other")
        };
        app.register_editable_bindings([
            EditableBinding::new("workspace:crash", crash_description, WorkspaceAction::Crash)
                .with_context_predicate(id!("Workspace")),
            EditableBinding::new(
                "workspace:log_review_comment_send_status_for_active_tab",
                BindingDescription::fluent("binding-workspace-log-review-comment-send-status"),
                WorkspaceAction::LogReviewCommentSendStatusForActiveTab,
            )
            .with_context_predicate(id!("Workspace")),
            EditableBinding::new(
                "workspace:panic",
                BindingDescription::fluent("binding-workspace-panic"),
                WorkspaceAction::Panic,
            )
            .with_context_predicate(id!("Workspace")),
            EditableBinding::new(
                "workspace:open_view_tree_debug_view",
                BindingDescription::fluent("binding-workspace-open-view-tree-debugger"),
                WorkspaceAction::OpenViewTreeDebugWindow,
            )
            .with_context_predicate(id!("Workspace")),
        ]);
        app.register_fixed_bindings([FixedBinding::empty(
            BindingDescription::fluent("binding-workspace-debug-fte-view"),
            WorkspaceAction::AddGetStartedTab,
            id!("Workspace"),
        )]);
        #[cfg(debug_assertions)]
        {
            // Debug actions for build plan migration modal (command palette only)
            app.register_editable_bindings([
                EditableBinding::new(
                    "workspace:open_build_plan_migration_modal",
                    BindingDescription::fluent(
                        "binding-workspace-debug-build-plan-migration-modal",
                    ),
                    WorkspaceAction::OpenBuildPlanMigrationModal,
                )
                .with_context_predicate(id!("Workspace")),
                EditableBinding::new(
                    "workspace:reset_build_plan_migration_modal_state",
                    BindingDescription::fluent(
                        "binding-workspace-debug-reset-build-plan-migration",
                    ),
                    WorkspaceAction::ResetBuildPlanMigrationModalState,
                )
                .with_context_predicate(id!("Workspace")),
                EditableBinding::new(
                    "workspace:debug_reset_aws_bedrock_login_banner_dismissed",
                    BindingDescription::fluent("binding-workspace-debug-reset-aws-bedrock-banner"),
                    WorkspaceAction::DebugResetAwsBedrockLoginBannerDismissed,
                )
                .with_context_predicate(id!("Workspace")),
                EditableBinding::new(
                    "workspace:open_oz_launch_modal",
                    BindingDescription::fluent("binding-workspace-debug-open-oz-launch-modal"),
                    WorkspaceAction::OpenOzLaunchModal,
                )
                .with_context_predicate(id!("Workspace")),
                EditableBinding::new(
                    "workspace:reset_oz_launch_modal_state",
                    BindingDescription::fluent("binding-workspace-debug-reset-oz-launch-modal"),
                    WorkspaceAction::ResetOzLaunchModalState,
                )
                .with_context_predicate(id!("Workspace")),
                EditableBinding::new(
                    "workspace:open_openwarp_launch_modal",
                    BindingDescription::fluent(
                        "binding-workspace-debug-open-openwarp-launch-modal",
                    ),
                    WorkspaceAction::OpenOpenWarpLaunchModal,
                )
                .with_context_predicate(id!("Workspace")),
                EditableBinding::new(
                    "workspace:reset_openwarp_launch_modal_state",
                    BindingDescription::fluent(
                        "binding-workspace-debug-reset-openwarp-launch-modal",
                    ),
                    WorkspaceAction::ResetOpenWarpLaunchModalState,
                )
                .with_context_predicate(id!("Workspace")),
                EditableBinding::new(
                    "workspace:open_orchestration_launch_modal",
                    "[Debug] Open Orchestration Launch Modal",
                    WorkspaceAction::OpenOrchestrationLaunchModal,
                )
                .with_context_predicate(id!("Workspace")),
                EditableBinding::new(
                    "workspace:reset_orchestration_launch_modal_state",
                    "[Debug] Reset Orchestration Launch Modal State",
                    WorkspaceAction::ResetOrchestrationLaunchModalState,
                )
                .with_context_predicate(id!("Workspace")),
                EditableBinding::new(
                    "workspace:open_agent_cli_launch_modal",
                    "[Debug] Open Warp Agent CLI Launch Modal",
                    WorkspaceAction::OpenAgentCliLaunchModal,
                )
                .with_context_predicate(id!("Workspace")),
                EditableBinding::new(
                    "workspace:reset_agent_cli_launch_modal_state",
                    "[Debug] Reset Warp Agent CLI Launch Modal State",
                    WorkspaceAction::ResetAgentCliLaunchModalState,
                )
                .with_context_predicate(id!("Workspace")),
                EditableBinding::new(
                    "workspace:open_feature_intro_modal",
                    "[Debug] Open Feature Intro Modal",
                    WorkspaceAction::OpenFeatureIntroModal,
                )
                .with_context_predicate(id!("Workspace")),
                EditableBinding::new(
                    "workspace:reset_feature_intro_modal_state",
                    "[Debug] Reset Feature Intro Modal State",
                    WorkspaceAction::ResetFeatureIntroModalState,
                )
                .with_context_predicate(id!("Workspace")),
                EditableBinding::new(
                    "workspace:open_auto_handoff_sleep_modal",
                    "[Debug] Open Auto-Handoff Sleep Modal",
                    WorkspaceAction::OpenAutoHandoffSleepModal,
                )
                .with_context_predicate(id!("Workspace")),
                EditableBinding::new(
                    "workspace:reset_auto_handoff_sleep_modal_state",
                    "[Debug] Reset Auto-Handoff Sleep Modal State",
                    WorkspaceAction::ResetAutoHandoffSleepModalState,
                )
                .with_context_predicate(id!("Workspace")),
                EditableBinding::new(
                    "workspace:trigger_auto_handoff_to_cloud",
                    "[Debug] Trigger Auto-Handoff to Cloud",
                    WorkspaceAction::TriggerAutoHandoffToCloud,
                )
                .with_context_predicate(id!("Workspace")),
                EditableBinding::new(
                    "workspace:open_free_ai_removal_modal",
                    "[Debug] Open Free AI Removal Modal",
                    WorkspaceAction::OpenFreeAiRemovalModal,
                )
                .with_context_predicate(id!("Workspace")),
                EditableBinding::new(
                    "workspace:reset_free_ai_removal_modal_state",
                    "[Debug] Reset Free AI Removal Modal State",
                    WorkspaceAction::ResetFreeAiRemovalModalState,
                )
                .with_context_predicate(id!("Workspace")),
                EditableBinding::new(
                    "workspace:install_opencode_warp_plugin",
                    BindingDescription::fluent(
                        "binding-workspace-debug-install-opencode-warp-plugin",
                    ),
                    WorkspaceAction::InstallOpenCodeWarpPlugin,
                )
                .with_context_predicate(id!("Workspace")),
                EditableBinding::new(
                    "workspace:use_local_opencode_warp_plugin",
                    BindingDescription::fluent(
                        "binding-workspace-debug-use-local-opencode-warp-plugin",
                    ),
                    WorkspaceAction::UseLocalOpenCodeWarpPlugin,
                )
                .with_context_predicate(id!("Workspace")),
                EditableBinding::new(
                    "workspace:open_session_config_modal",
                    BindingDescription::fluent("binding-workspace-debug-open-session-config-modal"),
                    WorkspaceAction::ShowSessionConfigModal,
                )
                .with_context_predicate(id!("Workspace")),
                EditableBinding::new(
                    "workspace:show_hoa_onboarding_flow",
                    BindingDescription::fluent("binding-workspace-debug-show-hoa-onboarding"),
                    WorkspaceAction::ShowHoaOnboardingFlow,
                )
                .with_context_predicate(id!("Workspace")),
            ]);
        }
    }

    #[cfg(target_os = "macos")]
    app.register_editable_bindings([EditableBinding::new(
        "workspace:sample_process",
        BindingDescription::fluent("binding-workspace-sample-process"),
        WorkspaceAction::SampleProcess,
    )
    .with_context_predicate(id!("Workspace"))]);

    #[cfg(any(feature = "dhat_heap_profiling", feature = "heap_usage_tracking"))]
    {
        app.register_editable_bindings([EditableBinding::new(
            "workspace:dump_heap_profile",
            BindingDescription::fluent("binding-workspace-dump-heap-profile"),
            WorkspaceAction::DumpHeapProfile,
        )
        .with_context_predicate(id!("Workspace"))]);
    }

    app.register_fixed_bindings([
        FixedBinding::custom(
            CustomAction::CycleNextSession,
            WorkspaceAction::CycleNextSession,
            BindingDescription::fluent("binding-workspace-cycle-next-session"),
            id!("Workspace") & id!("Workspace_MultipleTabs"),
        ),
        FixedBinding::custom(
            CustomAction::CyclePrevSession,
            WorkspaceAction::CyclePrevSession,
            BindingDescription::fluent("binding-workspace-cycle-prev-session"),
            id!("Workspace") & id!("Workspace_MultipleTabs"),
        ),
        FixedBinding::custom(
            CustomAction::AddWindow,
            WorkspaceAction::AddWindow,
            BindingDescription::fluent("binding-workspace-add-window"),
            id!("Workspace"),
        )
        .with_enabled(|| ContextFlag::CreateNewSession.is_enabled()),

    ]);

    app.register_editable_bindings([EditableBinding::new(
        NEW_FILE_BINDING_NAME,
        BindingDescription::fluent("binding-workspace-new-file"),
        WorkspaceAction::NewCodeFile,
    )
    .with_custom_action(CustomAction::NewFile)
    .with_context_predicate(id!("Workspace") & !id!("Workspace_ViewOnlySharedSession"))]);

    if FeatureFlag::UIZoom.is_enabled() {
        app.register_fixed_bindings([
            FixedBinding::custom(
                CustomAction::IncreaseZoom,
                WorkspaceAction::IncreaseZoom,
                BindingDescription::fluent("binding-workspace-zoom-in"),
                id!("Workspace"),
            )
            .with_group(bindings::BindingGroup::Settings.as_str()),
            FixedBinding::custom(
                CustomAction::DecreaseZoom,
                WorkspaceAction::DecreaseZoom,
                BindingDescription::fluent("binding-workspace-zoom-out"),
                id!("Workspace"),
            )
            .with_group(bindings::BindingGroup::Settings.as_str()),
            FixedBinding::custom(
                CustomAction::ResetZoom,
                WorkspaceAction::ResetZoom,
                BindingDescription::fluent("binding-workspace-reset-zoom"),
                id!("Workspace"),
            )
            .with_group(bindings::BindingGroup::Settings.as_str()),
        ]);
    } else {
        app.register_fixed_bindings([
            FixedBinding::custom(
                CustomAction::IncreaseFontSize,
                WorkspaceAction::IncreaseFontSize,
                BindingDescription::fluent("binding-workspace-increase-font-size"),
                id!("Workspace"),
            )
            .with_group(bindings::BindingGroup::Settings.as_str()),
            FixedBinding::custom(
                CustomAction::DecreaseFontSize,
                WorkspaceAction::DecreaseFontSize,
                BindingDescription::fluent("binding-workspace-decrease-font-size"),
                id!("Workspace"),
            )
            .with_group(bindings::BindingGroup::Settings.as_str()),
        ]);
    }

    if ContextFlag::LaunchConfigurations.is_enabled() {
        app.register_fixed_bindings([FixedBinding::custom(
            CustomAction::SaveCurrentConfig,
            WorkspaceAction::OpenLaunchConfigSaveModal,
            BindingDescription::fluent("binding-workspace-save-launch-configuration"),
            id!("Workspace"),
        )]);
    }

    if ChannelState::channel() == Channel::Integration {
        // Hack: Add explicit bindings for the tests, since the tests' injected
        // keypresses won't trigger Mac menu items. Unfortunately we can't use
        // cfg[test] because we are a separate process!
        app.register_fixed_bindings([
            FixedBinding::new(
                cmd_or_ctrl_shift("t"),
                WorkspaceAction::AddDefaultTab,
                id!("Workspace"),
            ),
            FixedBinding::new(
                cmd_or_ctrl_shift("p"),
                WorkspaceAction::TogglePalette {
                    mode: PaletteMode::Command,
                    source: PaletteSource::IntegrationTest,
                },
                id!("Workspace"),
            ),
            FixedBinding::new(
                "cmdorctrl-,",
                WorkspaceAction::ShowSettings,
                id!("Workspace"),
            ),
        ]);
    }

    if FeatureFlag::UIZoom.is_enabled() {
        app.register_editable_bindings([
            EditableBinding::new(
                "workspace:increase_zoom",
                BindingDescription::fluent("binding-workspace-increase-zoom-level"),
                WorkspaceAction::IncreaseZoom,
            )
            .with_context_predicate(id!("Workspace"))
            .with_group(bindings::BindingGroup::Settings.as_str())
            .with_key_binding("cmdorctrl-="),
            EditableBinding::new(
                "workspace:decrease_zoom",
                BindingDescription::fluent("binding-workspace-decrease-zoom-level"),
                WorkspaceAction::DecreaseZoom,
            )
            .with_context_predicate(id!("Workspace"))
            .with_group(bindings::BindingGroup::Settings.as_str())
            .with_key_binding("cmdorctrl--"),
            EditableBinding::new(
                "workspace:reset_zoom",
                BindingDescription::fluent("binding-workspace-reset-zoom-level"),
                WorkspaceAction::ResetZoom,
            )
            .with_group(bindings::BindingGroup::Settings.as_str())
            .with_context_predicate(id!("Workspace")),
            EditableBinding::new(
                "workspace:increase_font_size",
                BindingDescription::fluent("binding-workspace-increase-font-size"),
                WorkspaceAction::IncreaseFontSize,
            )
            .with_context_predicate(id!("Workspace"))
            .with_group(bindings::BindingGroup::Settings.as_str())
            .with_key_binding("alt-shift->"),
            EditableBinding::new(
                "workspace:decrease_font_size",
                BindingDescription::fluent("binding-workspace-decrease-font-size"),
                WorkspaceAction::DecreaseFontSize,
            )
            .with_context_predicate(id!("Workspace"))
            .with_group(bindings::BindingGroup::Settings.as_str())
            .with_key_binding("alt-shift-<"),
            EditableBinding::new(
                "workspace:reset_font_size",
                BindingDescription::fluent("binding-workspace-reset-font-size"),
                WorkspaceAction::ResetFontSize,
            )
            .with_group(bindings::BindingGroup::Settings.as_str())
            .with_context_predicate(id!("Workspace")),
        ]);
    } else {
        app.register_editable_bindings([
            EditableBinding::new(
                "workspace:increase_font_size",
                BindingDescription::fluent("binding-workspace-increase-font-size"),
                WorkspaceAction::IncreaseFontSize,
            )
            .with_context_predicate(id!("Workspace"))
            .with_group(bindings::BindingGroup::Settings.as_str())
            .with_key_binding("cmdorctrl-="),
            EditableBinding::new(
                "workspace:decrease_font_size",
                BindingDescription::fluent("binding-workspace-decrease-font-size"),
                WorkspaceAction::DecreaseFontSize,
            )
            .with_context_predicate(id!("Workspace"))
            .with_group(bindings::BindingGroup::Settings.as_str())
            .with_key_binding("cmdorctrl--"),
            EditableBinding::new(
                "workspace:reset_font_size",
                BindingDescription::fluent("binding-workspace-reset-font-size"),
                WorkspaceAction::ResetFontSize,
            )
            .with_group(bindings::BindingGroup::Settings.as_str())
            .with_context_predicate(id!("Workspace"))
            .with_key_binding("cmdorctrl-0")
            .with_custom_action(CustomAction::ResetFontSize),
        ]);
    }

    app.register_fixed_bindings([
        // Menu dispatch for the "Open File Picker" custom action.
        FixedBinding::custom(
            CustomAction::ToggleProjectExplorer,
            WorkspaceAction::ToggleProjectExplorer,
            BindingDescription::new(t!("command-toggle-project-explorer")).with_custom_description(
                bindings::MAC_MENUS_CONTEXT,
                t!("command-project-explorer-mac"),
            ),
            id!("Workspace") & id!(flags::SHOW_PROJECT_EXPLORER),
        ),
    ]);

    app.register_editable_bindings([
        EditableBinding::new(
            "workspace:show_theme_chooser",
            BindingDescription::fluent("binding-workspace-open-theme-picker"),
            WorkspaceAction::ShowThemeChooserForActiveTheme,
        )
        .with_context_predicate(id!("Workspace"))
        .with_group(bindings::BindingGroup::Settings.as_str()),
        EditableBinding::new(
            TOGGLE_TAB_CONFIGS_MENU_BINDING_NAME,
            BindingDescription::fluent("binding-workspace-open-tab-configs-menu"),
            WorkspaceAction::ToggleTabConfigsMenu,
        )
        .with_context_predicate(id!("Workspace"))
        .with_mac_key_binding("cmd-ctrl-t")
        .with_linux_or_windows_key_binding("ctrl-alt-shift-T"),
        EditableBinding::new(
            "workspace:activate_first_tab",
            BindingDescription::fluent("binding-workspace-activate-tab-1"),
            WorkspaceAction::ActivateTabByNumber(1),
        )
        .with_context_predicate(id!("Workspace"))
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_key_binding("cmdorctrl-1"),
        EditableBinding::new(
            "workspace:activate_second_tab",
            BindingDescription::fluent("binding-workspace-activate-tab-2"),
            WorkspaceAction::ActivateTabByNumber(2),
        )
        .with_context_predicate(id!("Workspace"))
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_key_binding("cmdorctrl-2"),
        EditableBinding::new(
            "workspace:activate_third_tab",
            BindingDescription::fluent("binding-workspace-activate-tab-3"),
            WorkspaceAction::ActivateTabByNumber(3),
        )
        .with_context_predicate(id!("Workspace"))
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_key_binding("cmdorctrl-3"),
        EditableBinding::new(
            "workspace:activate_fourth_tab",
            BindingDescription::fluent("binding-workspace-activate-tab-4"),
            WorkspaceAction::ActivateTabByNumber(4),
        )
        .with_context_predicate(id!("Workspace"))
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_key_binding("cmdorctrl-4"),
        EditableBinding::new(
            "workspace:activate_fifth_tab",
            BindingDescription::fluent("binding-workspace-activate-tab-5"),
            WorkspaceAction::ActivateTabByNumber(5),
        )
        .with_context_predicate(id!("Workspace"))
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_key_binding("cmdorctrl-5"),
        EditableBinding::new(
            "workspace:activate_sixth_tab",
            BindingDescription::fluent("binding-workspace-activate-tab-6"),
            WorkspaceAction::ActivateTabByNumber(6),
        )
        .with_context_predicate(id!("Workspace"))
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_key_binding("cmdorctrl-6"),
        EditableBinding::new(
            "workspace:activate_seventh_tab",
            BindingDescription::fluent("binding-workspace-activate-tab-7"),
            WorkspaceAction::ActivateTabByNumber(7),
        )
        .with_context_predicate(id!("Workspace"))
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_key_binding("cmdorctrl-7"),
        EditableBinding::new(
            "workspace:activate_eighth_tab",
            BindingDescription::fluent("binding-workspace-activate-tab-8"),
            WorkspaceAction::ActivateTabByNumber(8),
        )
        .with_context_predicate(id!("Workspace"))
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_key_binding("cmdorctrl-8"),
        EditableBinding::new(
            "workspace:activate_last_tab",
            BindingDescription::fluent("binding-workspace-activate-tab-last"),
            WorkspaceAction::ActivateLastTab,
        )
        .with_context_predicate(id!("Workspace"))
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_key_binding("cmdorctrl-9"),
        EditableBinding::new(
            "workspace:activate_prev_tab",
            BindingDescription::fluent("binding-workspace-activate-prev-tab"),
            WorkspaceAction::ActivatePrevTab,
        )
        .with_context_predicate(
            id!("Workspace") & id!("Workspace_MultipleTabs") & !id!("Workspace_PaneDragging"),
        )
        .with_mac_key_binding("shift-cmd-{")
        .with_linux_or_windows_key_binding("ctrl-pageup"),
        EditableBinding::new(
            "workspace:activate_next_tab",
            BindingDescription::fluent("binding-workspace-activate-next-tab"),
            WorkspaceAction::ActivateNextTab,
        )
        .with_context_predicate(
            id!("Workspace") & id!("Workspace_MultipleTabs") & !id!("Workspace_PaneDragging"),
        )
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_mac_key_binding("shift-cmd-}")
        .with_linux_or_windows_key_binding("ctrl-pagedown"),
        EditableBinding::new(
            "pane_group:navigate_prev",
            BindingDescription::fluent("binding-workspace-activate-prev-pane"),
            WorkspaceAction::NavigatePrevPaneOrPanel,
        )
        .with_context_predicate(id!("Workspace"))
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_custom_action(CustomAction::ActivatePreviousPane),
        EditableBinding::new(
            "pane_group:navigate_next",
            BindingDescription::fluent("binding-workspace-activate-next-pane"),
            WorkspaceAction::NavigateNextPaneOrPanel,
        )
        .with_context_predicate(id!("Workspace"))
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_custom_action(CustomAction::ActivateNextPane),
        EditableBinding::new(
            "workspace:create_team_notebook",
            BindingDescription::new(t!("command-new-team-notebook")).with_custom_description(
                bindings::MAC_MENUS_CONTEXT,
                t!("command-new-team-notebook-mac"),
            ),
            WorkspaceAction::CreateTeamNotebook,
        )
        .with_custom_action(CustomAction::NewTeamNotebook)
        .with_context_predicate(
            id!("Workspace")
                & id!(flags::ENABLE_WARP_DRIVE)
                & id!("WarpDrive_BelongsToTeam")
                & id!("IsOnline"),
        )
        .with_group(bindings::BindingGroup::Notebooks.as_str()),
        EditableBinding::new(
            "workspace:create_personal_notebook",
            BindingDescription::new(t!("command-new-personal-notebook")).with_custom_description(
                bindings::MAC_MENUS_CONTEXT,
                t!("command-new-personal-notebook-mac"),
            ),
            WorkspaceAction::CreatePersonalNotebook,
        )
        .with_group(bindings::BindingGroup::Notebooks.as_str())
        .with_custom_action(CustomAction::NewPersonalNotebook)
        .with_context_predicate(id!("Workspace") & id!(flags::ENABLE_WARP_DRIVE)),
        EditableBinding::new(
            "workspace:create_team_workflow",
            BindingDescription::new(t!("command-new-team-workflow")).with_custom_description(
                bindings::MAC_MENUS_CONTEXT,
                t!("command-new-team-workflow-mac"),
            ),
            WorkspaceAction::CreateTeamWorkflow,
        )
        .with_custom_action(CustomAction::NewTeamWorkflow)
        .with_context_predicate(
            id!("Workspace")
                & id!(flags::ENABLE_WARP_DRIVE)
                & id!("IsOnline")
                & id!("WarpDrive_BelongsToTeam"),
        )
        .with_group(bindings::BindingGroup::Workflow.as_str()),
        EditableBinding::new(
            "workspace:create_personal_workflow",
            BindingDescription::new(t!("command-new-personal-workflow")).with_custom_description(
                bindings::MAC_MENUS_CONTEXT,
                t!("command-new-personal-workflow-mac"),
            ),
            WorkspaceAction::CreatePersonalWorkflow,
        )
        .with_group(bindings::BindingGroup::Workflow.as_str())
        .with_custom_action(CustomAction::NewPersonalWorkflow)
        .with_context_predicate(id!("Workspace") & id!(flags::ENABLE_WARP_DRIVE)),
        EditableBinding::new(
            "workspace:create_team_folder",
            BindingDescription::new(t!("command-new-team-folder")).with_custom_description(
                bindings::MAC_MENUS_CONTEXT,
                t!("command-new-team-folder-mac"),
            ),
            WorkspaceAction::CreateTeamFolder,
        )
        .with_context_predicate(
            id!("Workspace")
                & id!(flags::ENABLE_WARP_DRIVE)
                & id!("IsOnline")
                & id!("WarpDrive_BelongsToTeam"),
        )
        .with_group(bindings::BindingGroup::Folders.as_str()),
        EditableBinding::new(
            "workspace:create_personal_folder",
            BindingDescription::new(t!("command-new-personal-folder")).with_custom_description(
                bindings::MAC_MENUS_CONTEXT,
                t!("command-new-personal-folder-mac"),
            ),
            WorkspaceAction::CreatePersonalFolder,
        )
        .with_group(bindings::BindingGroup::Folders.as_str())
        .with_context_predicate(id!("Workspace") & id!(flags::ENABLE_WARP_DRIVE) & id!("IsOnline")),
        EditableBinding::new(
            NEW_TAB_BINDING_NAME,
            BindingDescription::new(t!("command-new-tab")),
            WorkspaceAction::AddDefaultTab,
        )
        .with_context_predicate(id!("Workspace") & !id!("Workspace_PaneDragging"))
        .with_custom_action(CustomAction::NewTab)
        .with_enabled(|| ContextFlag::CreateNewSession.is_enabled()),
        EditableBinding::new(
            NEW_TERMINAL_TAB_BINDING_NAME,
            BindingDescription::new(t!("command-new-terminal-tab")),
            WorkspaceAction::AddTerminalTab {
                hide_homepage: false,
            },
        )
        .with_context_predicate(id!("Workspace") & !id!("Workspace_PaneDragging"))
        .with_custom_action(CustomAction::NewTerminalTab)
        .with_enabled(|| ContextFlag::CreateNewSession.is_enabled()),
        EditableBinding::new(
            NEW_AGENT_TAB_BINDING_NAME,
            BindingDescription::new(t!("command-new-agent-tab")),
            WorkspaceAction::AddAgentTab,
        )
        .with_group(bindings::BindingGroup::WarpAi.as_str())
        .with_custom_action(CustomAction::NewAgentTab)
        .with_context_predicate(
            id!("Workspace") & id!(flags::IS_ANY_AI_ENABLED) & !id!("Workspace_PaneDragging"),
        ),
        EditableBinding::new(
            NEW_AMBIENT_AGENT_TAB_BINDING_NAME,
            BindingDescription::new(t!("command-new-cloud-agent-tab")),
            WorkspaceAction::AddAmbientAgentTab,
        )
        .with_group(bindings::BindingGroup::WarpAi.as_str())
        .with_context_predicate(
            id!("Workspace") & id!(flags::IS_ANY_AI_ENABLED) & !id!("Workspace_PaneDragging"),
        )
        .with_enabled(|| {
            FeatureFlag::AgentView.is_enabled() && FeatureFlag::CloudMode.is_enabled()
        }),
        EditableBinding::new(
            "workspace:toggle_left_panel",
            BindingDescription::new(t!("command-open-left-panel")),
            WorkspaceAction::ToggleLeftPanel,
        )
        .with_context_predicate(id!("Workspace"))
        .with_custom_action(CustomAction::ToggleWarpDrive),
        EditableBinding::new(
            TOGGLE_RIGHT_PANEL_BINDING_NAME,
            BindingDescription::new(t!("command-toggle-code-review")).with_custom_description(
                bindings::MAC_MENUS_CONTEXT,
                t!("command-toggle-code-review-mac"),
            ),
            WorkspaceAction::ToggleRightPanel,
        )
        .with_enabled(|| cfg!(feature = "local_fs"))
        .with_context_predicate(id!("Workspace"))
        .with_mac_key_binding("cmd-shift-+")
        .with_linux_or_windows_key_binding("ctrl-shift-+"),
        EditableBinding::new(
            TOGGLE_VERTICAL_TABS_PANEL_BINDING_NAME,
            BindingDescription::new(t!("command-toggle-vertical-tabs-panel"))
                .with_custom_description(
                    bindings::MAC_MENUS_CONTEXT,
                    t!("command-toggle-vertical-tabs-panel-mac"),
                ),
            WorkspaceAction::ToggleVerticalTabsPanel,
        )
        .with_context_predicate(id!("Workspace") & id!(flags::USE_VERTICAL_TABS_FLAG))
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_enabled(|| FeatureFlag::VerticalTabs.is_enabled())
        .with_key_binding(cmd_or_ctrl_shift("b")),
        EditableBinding::new(
            LEFT_PANEL_PROJECT_EXPLORER_BINDING_NAME,
            BindingDescription::new(t!("command-left-panel-project-explorer")),
            WorkspaceAction::ToggleProjectExplorer,
        )
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_context_predicate(id!("Workspace") & id!(flags::SHOW_PROJECT_EXPLORER))
        .with_custom_action(CustomAction::ToggleProjectExplorer),
        EditableBinding::new(
            LEFT_PANEL_AGENT_CONVERSATIONS_BINDING_NAME,
            BindingDescription::new(t!("command-left-panel-agent-conversations")),
            WorkspaceAction::ToggleConversationListView,
        )
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_context_predicate(id!("Workspace") & id!(flags::SHOW_CONVERSATION_HISTORY))
        .with_enabled(|| FeatureFlag::AgentViewConversationListView.is_enabled())
        .with_custom_action(CustomAction::ToggleConversationListView),
        EditableBinding::new(
            LEFT_PANEL_GLOBAL_SEARCH_BINDING_NAME,
            BindingDescription::new(t!("command-left-panel-global-search")),
            WorkspaceAction::ToggleGlobalSearch,
        )
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_context_predicate(id!("Workspace") & id!(flags::SHOW_GLOBAL_SEARCH))
        .with_enabled(|| FeatureFlag::GlobalSearch.is_enabled())
        .with_custom_action(CustomAction::ToggleGlobalSearch),
        EditableBinding::new(
            "file_tree:toggle_hidden_files",
            BindingDescription::new("Toggle hidden files in Project Explorer"),
            WorkspaceAction::ToggleHiddenFiles,
        )
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_context_predicate(id!("Workspace") & id!(flags::SHOW_PROJECT_EXPLORER))
        .with_mac_key_binding("cmd-shift->")
        .with_linux_or_windows_key_binding("ctrl-shift->"),
        EditableBinding::new(
            LEFT_PANEL_WARP_DRIVE_BINDING_NAME,
            BindingDescription::new(t!("command-left-panel-warp-drive")),
            WorkspaceAction::ToggleWarpDrive,
        )
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_context_predicate(id!("Workspace") & id!(flags::ENABLE_WARP_DRIVE))
        .with_mac_key_binding("ctrl-4")
        .with_linux_or_windows_key_binding("alt-4"),
        EditableBinding::new(
            TOGGLE_PROJECT_EXPLORER_BINDING_NAME,
            BindingDescription::new(t!("command-toggle-project-explorer")).with_custom_description(
                bindings::MAC_MENUS_CONTEXT,
                t!("command-project-explorer-mac"),
            ),
            WorkspaceAction::ToggleProjectExplorer,
        )
        .with_context_predicate(id!("Workspace") & id!(flags::SHOW_PROJECT_EXPLORER)),
        EditableBinding::new(
            OPEN_GLOBAL_SEARCH_BINDING_NAME,
            BindingDescription::new(t!("command-open-global-search")).with_custom_description(
                bindings::MAC_MENUS_CONTEXT,
                t!("command-global-search-mac"),
            ),
            WorkspaceAction::OpenGlobalSearch,
        )
        .with_context_predicate(id!("Workspace") & id!(flags::SHOW_GLOBAL_SEARCH))
        .with_mac_key_binding("cmd-shift-F")
        // we use alt because we use ctrl-shift-f for find because ctrl-f needs to be reserved for the shell
        .with_linux_or_windows_key_binding("alt-shift-F"),
        EditableBinding::new(
            TOGGLE_WARP_DRIVE_BINDING_NAME,
            BindingDescription::new(t!("command-toggle-warp-drive"))
                .with_custom_description(bindings::MAC_MENUS_CONTEXT, t!("command-warp-drive-mac")),
            WorkspaceAction::ToggleWarpDrive,
        )
        .with_context_predicate(id!("Workspace") & id!(flags::ENABLE_WARP_DRIVE)),
        EditableBinding::new(
            TOGGLE_CONVERSATION_LIST_VIEW_BINDING_NAME,
            BindingDescription::new(t!("command-toggle-agent-conversation-list"))
                .with_custom_description(
                    bindings::MAC_MENUS_CONTEXT,
                    t!("command-agent-conversation-list-mac"),
                ),
            WorkspaceAction::ToggleConversationListView,
        )
        .with_enabled(|| FeatureFlag::AgentViewConversationListView.is_enabled())
        .with_context_predicate(id!("Workspace") & id!(flags::SHOW_CONVERSATION_HISTORY))
        .with_mac_key_binding("cmd-shift-A")
        .with_linux_or_windows_key_binding("ctrl-shift-A")
        .with_group(bindings::BindingGroup::WarpAi.as_str()),
        EditableBinding::new(
            "workspace:close_panel",
            BindingDescription::new(t!("command-close-focused-panel")).with_custom_description(
                bindings::MAC_MENUS_CONTEXT,
                t!("command-close-focused-panel-mac"),
            ),
            WorkspaceAction::ClosePanel,
        )
        .with_context_predicate(id!("Workspace"))
        .with_custom_action(CustomAction::CloseCurrentSession),
        EditableBinding::new(
            "workspace:toggle_command_palette",
            BindingDescription::new(t!("command-toggle-command-palette")).with_custom_description(
                bindings::MAC_MENUS_CONTEXT,
                t!("command-command-palette-mac"),
            ),
            WorkspaceAction::TogglePalette {
                mode: PaletteMode::Command,
                source: PaletteSource::Keybinding,
            },
        )
        .with_group(bindings::BindingGroup::Settings.as_str())
        .with_context_predicate(id!("Workspace") & !id!("Workspace_CloudConversationWebViewer"))
        .with_custom_action(CustomAction::CommandPalette),
        EditableBinding::new(
            "workspace:move_tab_left",
            BindingDescription::new(t!("command-move-tab-left"))
                .with_dynamic_override(|ctx| uses_vertical_tabs(ctx).then(|| "move tab up".into())),
            WorkspaceAction::MoveActiveTabLeft,
        )
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_context_predicate(
            id!("Workspace")
                & id!("Workspace_MultipleTabs")
                & !id!("Workspace_LeftmostTabActive")
                & !id!("Workspace_PaneDragging"),
        )
        .with_custom_action(CustomAction::MoveTabLeft),
        EditableBinding::new(
            "workspace:move_tab_right",
            BindingDescription::new(t!("command-move-tab-right")).with_dynamic_override(|ctx| {
                uses_vertical_tabs(ctx).then(|| "move tab down".into())
            }),
            WorkspaceAction::MoveActiveTabRight,
        )
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_context_predicate(
            id!("Workspace")
                & id!("Workspace_MultipleTabs")
                & !id!("Workspace_RightmostTabActive")
                & !id!("Workspace_PaneDragging"),
        )
        .with_custom_action(CustomAction::MoveTabRight),
        EditableBinding::new(
            "workspace:toggle_keybindings_page",
            BindingDescription::fluent("binding-workspace-toggle-keybindings-page"),
            WorkspaceAction::ToggleKeybindingsPage,
        )
        .with_group(bindings::BindingGroup::KeyboardShortcuts.as_str())
        .with_context_predicate(id!("Workspace") & !id!("Workspace_TextOpen"))
        .with_custom_action(CustomAction::ToggleKeybindingsPage),
        EditableBinding::new(
            "workspace:show_keybinding_settings",
            BindingDescription::fluent("binding-workspace-open-keybindings-editor"),
            WorkspaceAction::ConfigureKeybindingSettings {
                keybinding_name: None,
            },
        )
        .with_group(bindings::BindingGroup::KeyboardShortcuts.as_str())
        .with_context_predicate(id!("Workspace"))
        .with_mac_key_binding("cmd-ctrl-k"),
        EditableBinding::new(
            "workspace:toggle_block_snackbar",
            BindingDescription::fluent("binding-workspace-toggle-block-snackbar"),
            WorkspaceAction::ToggleBlockSnackbar,
        )
        .with_group(bindings::BindingGroup::Settings.as_str())
        .with_context_predicate(id!("Workspace")),
    ]);

    // TODO(PLAT-113): Support a11y on non-MacOS platforms
    if cfg!(target_os = "macos") {
        app.register_editable_bindings([
            EditableBinding::new(
                "workspace:set_a11y_concise_verbosity_level",
                BindingDescription::fluent("binding-workspace-a11y-concise"),
                WorkspaceAction::SetA11yVerbosityLevel(AccessibilityVerbosity::Concise),
            )
            .with_context_predicate(id!("Workspace"))
            .with_key_binding("cmdorctrl-alt-c"),
            EditableBinding::new(
                "workspace:set_a11y_verbose_verbosity_level",
                BindingDescription::fluent("binding-workspace-a11y-verbose"),
                WorkspaceAction::SetA11yVerbosityLevel(AccessibilityVerbosity::Verbose),
            )
            .with_context_predicate(id!("Workspace"))
            .with_key_binding("cmdorctrl-alt-v"),
        ]);
    }

    app.register_editable_bindings([EditableBinding::new(
        "workspace:rename_active_tab",
        BindingDescription::fluent("binding-workspace-rename-active-tab"),
        WorkspaceAction::RenameActiveTab,
    )
    .with_group(bindings::BindingGroup::Settings.as_str())
    .with_custom_action(CustomAction::RenameTab)
    .with_context_predicate(id!("Workspace"))]);

    // Pane rename — same shape as RenameActiveTab but acts on the focused pane
    // in the active tab. Ships with no default keybinding so it surfaces in
    // Settings → Keyboard shortcuts as remappable; resolves issue #9351, where
    // the action existed only in the right-click context menu and was not
    // reachable via the binding registry.
    app.register_editable_bindings([EditableBinding::new(
        "workspace:rename_active_pane",
        "Rename the current pane",
        WorkspaceAction::RenameActivePane,
    )
    .with_group(bindings::BindingGroup::Settings.as_str())
    .with_context_predicate(id!("Workspace"))]);

    // Tab grouping bindings (keyless by default; gated on `GroupedTabs`).
    app.register_editable_bindings([
        EditableBinding::new(
            "workspace:new_tab_group",
            "Create new tab group",
            // Reuse the new-session dropdown's action, not a dedicated variant.
            WorkspaceAction::SelectNewSessionMenuItem(NewSessionMenuItem::CreateNewTabGroup),
        )
        .with_enabled(|| FeatureFlag::GroupedTabs.is_enabled())
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_context_predicate(id!("Workspace") & !id!("Workspace_PaneDragging")),
        EditableBinding::new(
            "workspace:new_tab_group_from_active_or_selected_tabs",
            "Create tab group from active or selected tab(s)",
            WorkspaceAction::NewTabGroupFromActiveOrSelectedTabs,
        )
        .with_enabled(|| FeatureFlag::GroupedTabs.is_enabled())
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_context_predicate(id!("Workspace") & !id!("Workspace_PaneDragging")),
        // Gated on `Workspace_ActiveOrSelectedTabsInGroup`: offered only when
        // there's an unambiguous group to leave — a single-group multi-selection,
        // or (with no selection) a grouped active tab. Mixed selections aren't
        // offered, matching the multi-tab right-click menu.
        EditableBinding::new(
            "workspace:remove_active_or_selected_tabs_from_group",
            "Remove active or selected tab(s) from group",
            WorkspaceAction::RemoveActiveOrSelectedTabsFromGroup,
        )
        .with_enabled(|| FeatureFlag::GroupedTabs.is_enabled())
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_context_predicate(
            id!("Workspace")
                & id!("Workspace_ActiveOrSelectedTabsInGroup")
                & !id!("Workspace_PaneDragging"),
        ),
    ]);

    // Tab/group pinning bindings (keyless by default; gated on `PinnedTabs`).
    // Pin/unpin are split into separate entries so the palette label tracks
    // the active tab/group's current state.
    app.register_editable_bindings([
        EditableBinding::new(
            "workspace:pin_active_tab",
            "Pin current tab",
            WorkspaceAction::PinActiveTab,
        )
        .with_enabled(|| FeatureFlag::PinnedTabs.is_enabled())
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_context_predicate(
            id!("Workspace") & !id!("Workspace_ActiveTabPinned") & !id!("Workspace_PaneDragging"),
        ),
        EditableBinding::new(
            "workspace:unpin_active_tab",
            "Unpin current tab",
            WorkspaceAction::UnpinActiveTab,
        )
        .with_enabled(|| FeatureFlag::PinnedTabs.is_enabled())
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_context_predicate(
            id!("Workspace") & id!("Workspace_ActiveTabPinned") & !id!("Workspace_PaneDragging"),
        ),
        EditableBinding::new(
            "workspace:pin_active_tab_group",
            "Pin current tab group",
            WorkspaceAction::PinActiveTabGroup,
        )
        .with_enabled(|| {
            FeatureFlag::PinnedTabs.is_enabled() && FeatureFlag::GroupedTabs.is_enabled()
        })
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_context_predicate(
            id!("Workspace")
                & id!("Workspace_ActiveTabInGroup")
                & !id!("Workspace_ActiveTabGroupPinned")
                & !id!("Workspace_PaneDragging"),
        ),
        EditableBinding::new(
            "workspace:unpin_active_tab_group",
            "Unpin current tab group",
            WorkspaceAction::UnpinActiveTabGroup,
        )
        .with_enabled(|| {
            FeatureFlag::PinnedTabs.is_enabled() && FeatureFlag::GroupedTabs.is_enabled()
        })
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_context_predicate(
            id!("Workspace")
                & id!("Workspace_ActiveTabInGroup")
                & id!("Workspace_ActiveTabGroupPinned")
                & !id!("Workspace_PaneDragging"),
        ),
    ]);

    app.register_editable_bindings([
        EditableBinding::new(
            "workspace:terminate_app",
            BindingDescription::fluent("binding-workspace-quit-warp"),
            WorkspaceAction::TerminateApp,
        )
        .with_context_predicate(id!("Workspace"))
        .with_group(bindings::BindingGroup::Close.as_str())
        .with_enabled(|| ContextFlag::CloseWindow.is_enabled()),
        EditableBinding::new(
            "workspace:close_window",
            BindingDescription::new(t!("command-close-window")).with_custom_description(
                bindings::MAC_MENUS_CONTEXT,
                t!("command-close-window-mac"),
            ),
            WorkspaceAction::CloseWindow,
        )
        .with_mac_key_binding("cmd-shift-W")
        .with_context_predicate(id!("Workspace"))
        .with_group(bindings::BindingGroup::Close.as_str())
        .with_custom_action(CustomAction::CloseWindow)
        .with_enabled(|| ContextFlag::CloseWindow.is_enabled()),
        EditableBinding::new(
            "workspace:close_active_tab",
            BindingDescription::fluent("binding-workspace-close-active-tab"),
            WorkspaceAction::CloseActiveTab,
        )
        .with_custom_action(CustomAction::CloseTab)
        .with_group(bindings::BindingGroup::Close.as_str())
        .with_context_predicate(
            id!("Workspace") & (id!("Workspace_CloseWindow") | id!("Workspace_MultipleTabs")),
        ),
        EditableBinding::new(
            "workspace:close_other_tabs",
            BindingDescription::fluent("binding-workspace-close-other-tabs"),
            WorkspaceAction::CloseNonActiveTabs,
        )
        .with_custom_action(CustomAction::CloseOtherTabs)
        .with_group(bindings::BindingGroup::Close.as_str())
        .with_context_predicate(id!("Workspace")),
        EditableBinding::new(
            "workspace:close_tabs_right_active_tab",
            BindingDescription::new(t!("command-close-tabs-right")).with_dynamic_override(|ctx| {
                uses_vertical_tabs(ctx).then(|| "close tabs below".into())
            }),
            WorkspaceAction::CloseTabsRightActiveTab,
        )
        .with_group(bindings::BindingGroup::Close.as_str())
        .with_custom_action(CustomAction::CloseTabsRight)
        .with_context_predicate(id!("Workspace")),
        // We have two actions depending on the current state
        // (i.e. whether notifications are already on or off).
        EditableBinding::new(
            "workspace:toggle_notifications_on",
            BindingDescription::fluent("binding-workspace-toggle-notifications-on"),
            WorkspaceAction::ToggleNotifications,
        )
        .with_group(bindings::BindingGroup::Notifications.as_str())
        .with_context_predicate(id!("Workspace") & !id!("Notifications_Enabled")),
        EditableBinding::new(
            "workspace:toggle_notifications_off",
            BindingDescription::fluent("binding-workspace-toggle-notifications-off"),
            WorkspaceAction::ToggleNotifications,
        )
        .with_group(bindings::BindingGroup::Notifications.as_str())
        .with_context_predicate(id!("Workspace") & id!("Notifications_Enabled")),
        EditableBinding::new(
            "workspace:toggle_navigation_palette",
            BindingDescription::new(t!("command-toggle-navigation-palette"))
                .with_custom_description(
                    bindings::MAC_MENUS_CONTEXT,
                    t!("command-navigation-palette-mac"),
                ),
            WorkspaceAction::TogglePalette {
                mode: PaletteMode::Navigation,
                source: PaletteSource::Keybinding,
            },
        )
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_context_predicate(id!("Workspace"))
        .with_custom_action(CustomAction::NavigationPalette),
        EditableBinding::new(
            "workspace:toggle_launch_config_palette",
            BindingDescription::fluent("binding-workspace-launch-config-palette"),
            WorkspaceAction::TogglePalette {
                mode: PaletteMode::LaunchConfig,
                source: PaletteSource::Keybinding,
            },
        )
        .with_context_predicate(id!("Workspace"))
        .with_custom_action(CustomAction::LaunchConfigPalette)
        .with_enabled(|| ContextFlag::LaunchConfigurations.is_enabled()),
        EditableBinding::new(
            "workspace:toggle_files_palette",
            BindingDescription::fluent("binding-workspace-toggle-files-palette"),
            WorkspaceAction::TogglePalette {
                mode: PaletteMode::Files,
                source: PaletteSource::Keybinding,
            },
        )
        .with_context_predicate(id!("Workspace") & !id!("Workspace_ViewOnlySharedSession"))
        .with_custom_action(CustomAction::FilesPalette),
        EditableBinding::new(
            "workspace:open_launch_config_save_modal",
            BindingDescription::fluent("binding-workspace-save-launch-configuration"),
            WorkspaceAction::OpenLaunchConfigSaveModal,
        )
        .with_context_predicate(id!("Workspace"))
        .with_custom_action(CustomAction::SaveCurrentConfig)
        .with_enabled(|| ContextFlag::LaunchConfigurations.is_enabled()),
        EditableBinding::new(
            // If you rename this name, please update the name in command_palette/action/data_source.rs
            "workspace:search_drive",
            BindingDescription::fluent("binding-workspace-search-warp-drive"),
            WorkspaceAction::OpenPalette {
                mode: PaletteMode::WarpDrive,
                source: PaletteSource::Keybinding,
                query: None,
            },
        )
        .with_context_predicate(id!("Workspace"))
        .with_custom_action(CustomAction::SearchDrive),
    ]);

    if FeatureFlag::Autoupdate.is_enabled() {
        app.register_editable_bindings([
            EditableBinding::new(
                "workspace:update_and_relaunch",
                BindingDescription::fluent("binding-workspace-update-and-relaunch"),
                // TODO(vorporeal): I wonder if we should change wording here?
                WorkspaceAction::ApplyUpdate,
            )
            .with_group(bindings::BindingGroup::AutoUpdate.as_str())
            .with_context_predicate(id!("Workspace") & id!("AutoupdateState_UpdateReady"))
            .with_enabled(|| ContextFlag::PromptForVersionUpdates.is_enabled()),
            EditableBinding::new(
                "workspace:check_for_updates",
                BindingDescription::fluent("binding-workspace-check-for-updates"),
                WorkspaceAction::CheckForUpdate,
            )
            .with_group(bindings::BindingGroup::AutoUpdate.as_str())
            .with_context_predicate(id!("Workspace") & !id!("AutoupdateState_UpdateReady"))
            .with_enabled(|| ContextFlag::PromptForVersionUpdates.is_enabled()),
        ]);
    }

    app.register_editable_bindings([EditableBinding::new(
        "workspace:log_out",
        BindingDescription::fluent("binding-workspace-log-out"),
        WorkspaceAction::LogOut,
    )
    .with_group(bindings::BindingGroup::Settings.as_str())
    .with_context_predicate(id!("Workspace") & !id!("IsAnonymousUser"))]);

    if !FeatureFlag::AvatarInTabBar.is_enabled() {
        app.register_editable_bindings([EditableBinding::new(
            "workspace:toggle_resource_center",
            BindingDescription::fluent("binding-workspace-toggle-resource-center"),
            WorkspaceAction::ToggleResourceCenter,
        )
        .with_group(bindings::BindingGroup::Navigation.as_str())
        .with_context_predicate(id!("Workspace"))
        .with_custom_action(CustomAction::ToggleResourceCenter)]);
    }

    if cfg!(not(target_family = "wasm")) {
        app.register_editable_bindings([EditableBinding::new(
            "workspace:export_all_warp_drive_objects",
            BindingDescription::fluent("binding-workspace-export-warp-drive-objects"),
            WorkspaceAction::ExportAllWarpDriveObjects,
        )
        .with_group(bindings::BindingGroup::Settings.as_str())
        .with_context_predicate(id!("Workspace") & id!(flags::ENABLE_WARP_DRIVE))]);
    }

    // Oz and Warp Control CLI install/uninstall actions (macOS only)
    #[cfg(target_os = "macos")]
    {
        app.register_editable_bindings([
            EditableBinding::new(
                "workspace:install_cli",
                BindingDescription::fluent("binding-workspace-install-cli"),
                WorkspaceAction::InstallOz,
            )
            .with_group(bindings::BindingGroup::Settings.as_str())
            .with_context_predicate(id!("Workspace")),
            EditableBinding::new(
                "workspace:uninstall_cli",
                BindingDescription::fluent("binding-workspace-uninstall-cli"),
                WorkspaceAction::UninstallOz,
            )
            .with_group(bindings::BindingGroup::Settings.as_str())
            .with_context_predicate(id!("Workspace")),
        ]);
        if FeatureFlag::WarpControlCli.is_enabled() {
            app.register_editable_bindings([
                EditableBinding::new(
                    "workspace:install_warpctrl",
                    "Install Warp Control CLI globally for use outside of Warp",
                    WorkspaceAction::InstallWarpctrl,
                )
                .with_group(bindings::BindingGroup::Settings.as_str())
                .with_context_predicate(id!("Workspace")),
                EditableBinding::new(
                    "workspace:uninstall_warpctrl",
                    "Undo global Warp Control CLI installation (warpctrl will still work within Warp)",
                    WorkspaceAction::UninstallWarpctrl,
                )
                .with_group(bindings::BindingGroup::Settings.as_str())
                .with_context_predicate(id!("Workspace")),
            ]);
        }
    }

    if FeatureFlag::Changelog.is_enabled() {
        app.register_editable_bindings([
            // Always show the BindingDescription::fluent("binding-workspace-view-changelog") action in the command palette,
            // but without a keybinding when the update toast is not visible.
            EditableBinding::new(
                "workspace:view_changelog",
                BindingDescription::fluent("binding-workspace-view-changelog"),
                WorkspaceAction::ViewLatestChangelog,
            )
            .with_context_predicate(id!("Workspace") & !id!("UpdateToastVisible"))
            .with_group(bindings::BindingGroup::Settings.as_str())
            // Note that while the changelog resides in WarpEssentials, we should gate access to
            // the changelog based on whether WarpEssentials is an available view.
            .with_enabled(|| ContextFlag::WarpEssentials.is_enabled()),
            // When the update toast is visible, register the keybinding as well.
            EditableBinding::new(
                "workspace:view_changelog",
                BindingDescription::fluent("binding-workspace-view-changelog"),
                WorkspaceAction::ViewLatestChangelog,
            )
            .with_context_predicate(id!("Workspace") & id!("UpdateToastVisible"))
            .with_group(bindings::BindingGroup::Settings.as_str())
            .with_custom_action(CustomAction::ViewChangelog)
            .with_linux_or_windows_key_binding(format!("alt-{}", cmd_or_ctrl_shift("o")))
            .with_enabled(|| ContextFlag::WarpEssentials.is_enabled()),
        ]);
    }

    // We use the same binding name for the AI Assistant and block list AI to preserve custom
    // keybindings between them.
    app.register_editable_bindings([
        EditableBinding::new(
            "workspace:toggle_ai_assistant",
            *NEW_AGENT_PANE_LABEL,
            WorkspaceAction::NewPaneInAgentMode {
                entrypoint: AgentModeEntrypoint::NewPaneBinding,
                zero_state_prompt_suggestion_type: None,
            },
        )
        .with_enabled(|| FeatureFlag::AgentMode.is_enabled())
        .with_context_predicate(id!("Workspace") & id!(flags::IS_ANY_AI_ENABLED))
        .with_group(bindings::BindingGroup::WarpAi.as_str())
        .with_custom_action(CustomAction::NewAgentModePane),
        EditableBinding::new(
            "workspace:toggle_ai_assistant",
            BindingDescription::fluent("binding-workspace-toggle-warp-ai"),
            WorkspaceAction::ToggleAIAssistant,
        )
        .with_enabled(|| !FeatureFlag::AgentMode.is_enabled())
        .with_context_predicate(id!("Workspace") & id!(flags::IS_ANY_AI_ENABLED))
        .with_group(bindings::BindingGroup::WarpAi.as_str())
        // We use the same custom action as AM so that we don't have
        // two mac menu items for AM vs Warp AI since they are mutually exclusive.
        .with_custom_action(CustomAction::NewAgentModePane),
    ]);

    app.register_editable_bindings([
        EditableBinding::new(
            "workspace:create_team_env_vars",
            BindingDescription::new(t!("command-new-team-env-vars")).with_custom_description(
                bindings::MAC_MENUS_CONTEXT,
                t!("command-new-team-env-vars-mac"),
            ),
            WorkspaceAction::CreateTeamEnvVarCollection,
        )
        .with_custom_action(CustomAction::NewTeamEnvVars)
        .with_context_predicate(
            id!("Workspace")
                & id!(flags::ENABLE_WARP_DRIVE)
                & id!("WarpDrive_BelongsToTeam")
                & id!("IsOnline"),
        )
        .with_group(bindings::BindingGroup::EnvVarCollection.as_str()),
        EditableBinding::new(
            "workspace:create_personal_env_vars",
            BindingDescription::new(t!("command-new-personal-env-vars")).with_custom_description(
                bindings::MAC_MENUS_CONTEXT,
                t!("command-new-personal-env-vars-mac"),
            ),
            WorkspaceAction::CreatePersonalEnvVarCollection,
        )
        .with_group(bindings::BindingGroup::EnvVarCollection.as_str())
        .with_custom_action(CustomAction::NewPersonalEnvVars)
        .with_context_predicate(id!("Workspace") & id!(flags::ENABLE_WARP_DRIVE)),
        EditableBinding::new(
            "workspace:create_personal_ai_prompt",
            BindingDescription::new(t!("command-new-personal-prompt")).with_custom_description(
                bindings::MAC_MENUS_CONTEXT,
                t!("command-new-personal-prompt-mac"),
            ),
            WorkspaceAction::CreatePersonalAIPrompt,
        )
        .with_group(bindings::BindingGroup::WarpAi.as_str())
        .with_custom_action(CustomAction::NewPersonalAIPrompt)
        .with_context_predicate(
            id!("Workspace") & id!(flags::ENABLE_WARP_DRIVE) & id!(flags::IS_ANY_AI_ENABLED),
        ),
        EditableBinding::new(
            "workspace:create_team_ai_prompt",
            BindingDescription::new(t!("command-new-team-prompt")).with_custom_description(
                bindings::MAC_MENUS_CONTEXT,
                t!("command-new-team-prompt-mac"),
            ),
            WorkspaceAction::CreateTeamAIPrompt,
        )
        .with_group(bindings::BindingGroup::WarpAi.as_str())
        .with_custom_action(CustomAction::NewTeamAIPrompt)
        .with_context_predicate(
            id!("Workspace")
                & id!(flags::ENABLE_WARP_DRIVE)
                & id!("WarpDrive_BelongsToTeam")
                & id!("IsOnline")
                & id!(flags::IS_ANY_AI_ENABLED),
        ),
    ]);

    app.register_editable_bindings([
        EditableBinding::new(
            "workspace:shift_focus_left",
            BindingDescription::fluent("binding-workspace-shift-focus-left"),
            WorkspaceAction::FocusLeftPanel,
        )
        .with_context_predicate(id!("Workspace"))
        .with_key_binding("cmdorctrl-shift-("),
        EditableBinding::new(
            "workspace:shift_focus_right",
            BindingDescription::fluent("binding-workspace-shift-focus-right"),
            WorkspaceAction::FocusRightPanel,
        )
        .with_context_predicate(id!("Workspace"))
        .with_key_binding("cmdorctrl-shift-)"),
    ]);

    app.register_editable_bindings([
        EditableBinding::new(
            "workspace:import_to_personal_drive",
            BindingDescription::fluent("binding-workspace-import-to-personal-drive"),
            WorkspaceAction::ImportToPersonalDrive,
        )
        .with_context_predicate(id!("Workspace") & id!(flags::ENABLE_WARP_DRIVE)),
        EditableBinding::new(
            "workspace:import_to_team_drive",
            BindingDescription::fluent("binding-workspace-import-to-team-drive"),
            WorkspaceAction::ImportToTeamDrive,
        )
        .with_context_predicate(
            id!("Workspace") & id!(flags::ENABLE_WARP_DRIVE) & id!("WarpDrive_BelongsToTeam"),
        ),
    ]);

    // Register a debug-only action for writing the user's access token to the system clipboard
    // to aid debugging and development.
    #[cfg(not(feature = "skip_login"))]
    if ChannelState::enable_debug_features() {
        app.register_editable_bindings([EditableBinding::new(
            "workspace:copy_access_token_to_clipboard",
            BindingDescription::fluent("binding-workspace-copy-access-token"),
            WorkspaceAction::CopyAccessTokenToClipboard,
        )
        .with_context_predicate(id!("Workspace"))]);
    }

    app.register_editable_bindings([
        EditableBinding::new(
            "workspace:copy_current_path",
            BindingDescription::new("Copy current path")
                .with_custom_description(bindings::MAC_MENUS_CONTEXT, "Copy Current Path"),
            WorkspaceAction::CopyCurrentPath,
        )
        .with_context_predicate(id!("Workspace")),
        EditableBinding::new(
            "workspace:open_repository",
            BindingDescription::new(t!("command-open-repository")).with_custom_description(
                bindings::MAC_MENUS_CONTEXT,
                t!("command-open-repository-mac"),
            ),
            WorkspaceAction::OpenRepository { path: None },
        )
        .with_context_predicate(id!("Workspace"))
        .with_custom_action(CustomAction::OpenRepository)
        .with_group(bindings::BindingGroup::Folders.as_str()),
        EditableBinding::new(
            "workspace:open_ai_fact_collection",
            BindingDescription::new(t!("command-open-ai-rules")).with_custom_description(
                bindings::MAC_MENUS_CONTEXT,
                t!("command-open-ai-rules-mac"),
            ),
            WorkspaceAction::OpenAIFactCollection,
        )
        .with_enabled(|| FeatureFlag::AIRules.is_enabled())
        .with_custom_action(CustomAction::OpenAIFactCollection)
        .with_context_predicate(id!("Workspace") & id!(flags::IS_ANY_AI_ENABLED))
        .with_group(bindings::BindingGroup::WarpAi.as_str()),
    ]);

    app.register_editable_bindings([EditableBinding::new(
        "workspace:open_mcp_servers",
        BindingDescription::new(t!("command-open-mcp-servers")).with_custom_description(
            bindings::MAC_MENUS_CONTEXT,
            t!("command-open-mcp-servers-mac"),
        ),
        WorkspaceAction::OpenMCPServerCollection,
    )
    .with_enabled(|| {
        FeatureFlag::McpServer.is_enabled() && ContextFlag::ShowMCPServers.is_enabled()
    })
    .with_custom_action(CustomAction::OpenMCPServerCollection)
    .with_context_predicate(id!("Workspace") & id!(flags::IS_ANY_AI_ENABLED))
    .with_group(bindings::BindingGroup::WarpAi.as_str())]);

    app.register_editable_bindings([EditableBinding::new(
        "workspace:jump_to_latest_toast",
        BindingDescription::fluent("binding-workspace-jump-to-latest-toast"),
        WorkspaceAction::JumpToLatestToast,
    )
    .with_enabled(|| FeatureFlag::AgentMode.is_enabled())
    .with_context_predicate(id!("Workspace") & id!(flags::IS_ANY_AI_ENABLED))
    .with_mac_key_binding("cmd-shift-G")
    .with_linux_or_windows_key_binding("ctrl-shift-G")
    .with_group(bindings::BindingGroup::WarpAi.as_str())]);

    app.register_editable_bindings([EditableBinding::new(
        TOGGLE_NOTIFICATION_MAILBOX_BINDING_NAME,
        BindingDescription::fluent("binding-workspace-toggle-notification-mailbox"),
        WorkspaceAction::ToggleNotificationMailbox { select_first: true },
    )
    .with_enabled(|| FeatureFlag::HOANotifications.is_enabled())
    .with_context_predicate(id!("Workspace"))
    .with_mac_key_binding("cmd-shift-U")
    .with_linux_or_windows_key_binding("ctrl-shift-U")
    .with_group(bindings::BindingGroup::WarpAi.as_str())]);

    add_open_setting_pages_as_editable_binding(app);
    add_overflow_menu_items_as_editable_binding(app);

    app.register_editable_bindings([EditableBinding::new(
        "workspace:toggle_agent_management_view",
        BindingDescription::fluent("binding-workspace-toggle-agent-management-view"),
        WorkspaceAction::ToggleAgentManagementView,
    )
    .with_enabled(|| FeatureFlag::AgentManagementView.is_enabled())
    .with_context_predicate(id!("Workspace") & id!(flags::IS_ANY_AI_ENABLED))
    .with_mac_key_binding("cmd-shift-M")
    .with_linux_or_windows_key_binding("ctrl-shift-M")
    .with_group(bindings::BindingGroup::WarpAi.as_str())]);
}

fn add_open_setting_pages_as_editable_binding(app: &mut AppContext) {
    use warpui::keymap::macros::*;

    // Add the ability to open setting modals to the command palette.
    app.register_editable_bindings([
        EditableBinding::new(
            "workspace:show_settings",
            BindingDescription::new(t!("command-open-settings"))
                .with_custom_description(bindings::MAC_MENUS_CONTEXT, t!("command-settings-mac")),
            WorkspaceAction::ShowSettings,
        )
        .with_context_predicate(id!("Workspace"))
        .with_group(bindings::BindingGroup::Settings.as_str())
        .with_custom_action(CustomAction::ShowSettings),
        EditableBinding::new(
            "workspace:show_settings_account_page",
            BindingDescription::fluent("binding-workspace-open-settings-account"),
            WorkspaceAction::ShowSettingsPage(SettingsSection::Account),
        )
        .with_context_predicate(id!("Workspace"))
        .with_group(bindings::BindingGroup::Settings.as_str())
        .with_custom_action(CustomAction::ShowAccount),
        EditableBinding::new(
            "workspace:show_settings_appearance_page",
            BindingDescription::new(t!("command-open-settings-appearance"))
                .with_custom_description(bindings::MAC_MENUS_CONTEXT, t!("command-appearance-mac")),
            WorkspaceAction::ShowSettingsPage(SettingsSection::Appearance),
        )
        .with_group(bindings::BindingGroup::Settings.as_str())
        .with_context_predicate(id!("Workspace"))
        .with_custom_action(CustomAction::ShowAppearance),
        EditableBinding::new(
            "workspace:show_settings_features_page",
            BindingDescription::fluent("binding-workspace-open-settings-features"),
            WorkspaceAction::ShowSettingsPage(SettingsSection::Features),
        )
        .with_group(bindings::BindingGroup::Settings.as_str())
        .with_context_predicate(id!("Workspace")),
        EditableBinding::new(
            "workspace:show_settings_shared_blocks_page",
            BindingDescription::new(t!("command-open-settings-shared-blocks"))
                .with_custom_description(
                    bindings::MAC_MENUS_CONTEXT,
                    t!("command-view-shared-blocks-mac"),
                ),
            WorkspaceAction::ShowSettingsPage(SettingsSection::SharedBlocks),
        )
        .with_group(bindings::BindingGroup::Settings.as_str())
        .with_context_predicate(id!("Workspace"))
        .with_custom_action(CustomAction::ViewSharedBlocks),
        EditableBinding::new(
            "workspace:show_settings_keyboard_shortcuts_page",
            BindingDescription::new(t!("command-open-settings-keyboard-shortcuts"))
                .with_custom_description(
                    bindings::MAC_MENUS_CONTEXT,
                    t!("command-configure-keyboard-shortcuts-mac"),
                ),
            WorkspaceAction::ShowSettingsPage(SettingsSection::Keybindings),
        )
        .with_group(bindings::BindingGroup::KeyboardShortcuts.as_str())
        .with_context_predicate(id!("Workspace"))
        .with_custom_action(CustomAction::ConfigureKeybindings),
        EditableBinding::new(
            "workspace:show_settings_about_page",
            BindingDescription::new(t!("command-open-settings-about"))
                .with_custom_description(bindings::MAC_MENUS_CONTEXT, t!("command-about-warp-mac")),
            WorkspaceAction::ShowSettingsPage(SettingsSection::About),
        )
        .with_group(bindings::BindingGroup::Settings.as_str())
        .with_context_predicate(id!("Workspace"))
        .with_custom_action(CustomAction::ShowAboutWarp),
        EditableBinding::new(
            "workspace:show_settings_teams_page",
            BindingDescription::new(t!("command-open-settings-teams")).with_custom_description(
                bindings::MAC_MENUS_CONTEXT,
                t!("command-open-team-settings-mac"),
            ),
            WorkspaceAction::ShowSettingsPage(SettingsSection::Teams),
        )
        .with_group(bindings::BindingGroup::Settings.as_str())
        .with_custom_action(CustomAction::OpenTeamSettings)
        .with_context_predicate(id!("Workspace")),
        EditableBinding::new(
            "workspace:show_settings_privacy_page",
            BindingDescription::new(t!("command-open-settings-privacy")),
            WorkspaceAction::ShowSettingsPage(SettingsSection::Privacy),
        )
        .with_group(bindings::BindingGroup::Settings.as_str())
        .with_context_predicate(id!("Workspace")),
        EditableBinding::new(
            "workspace:show_settings_warpify_page",
            BindingDescription::new(t!("command-open-settings-warpify")).with_custom_description(
                bindings::MAC_MENUS_CONTEXT,
                t!("command-configure-warpify-mac"),
            ),
            WorkspaceAction::ShowSettingsPage(SettingsSection::Warpify),
        )
        .with_group(bindings::BindingGroup::Settings.as_str())
        .with_context_predicate(id!("Workspace")),
        EditableBinding::new(
            "workspace:show_ai_settings_page",
            BindingDescription::new(t!("command-open-settings-ai")),
            WorkspaceAction::ShowSettingsPage(SettingsSection::WarpAgent),
        )
        .with_enabled(|| FeatureFlag::AgentMode.is_enabled())
        .with_group(bindings::BindingGroup::Settings.as_str())
        .with_context_predicate(id!("Workspace")),
        EditableBinding::new(
            "workspace:show_settings_billing_and_usage_page",
            BindingDescription::new(t!("command-open-settings-billing")),
            WorkspaceAction::ShowSettingsPage(SettingsSection::BillingAndUsage),
        )
        .with_group(bindings::BindingGroup::Settings.as_str())
        .with_context_predicate(id!("Workspace")),
        EditableBinding::new(
            "workspace:show_settings_code_page",
            BindingDescription::new(t!("command-open-settings-code")),
            WorkspaceAction::ShowSettingsPage(SettingsSection::CodeIndexing),
        )
        .with_group(bindings::BindingGroup::Settings.as_str())
        .with_context_predicate(id!("Workspace")),
        EditableBinding::new(
            "workspace:show_settings_referrals_page",
            BindingDescription::new(t!("command-open-settings-referrals")),
            WorkspaceAction::ShowSettingsPage(SettingsSection::Referrals),
        )
        .with_group(bindings::BindingGroup::Settings.as_str())
        .with_context_predicate(id!("Workspace")),
        EditableBinding::new(
            "workspace:show_settings_environments_page",
            BindingDescription::new(t!("command-open-settings-environments")),
            WorkspaceAction::ShowSettingsPage(SettingsSection::CloudEnvironments),
        )
        .with_group(bindings::BindingGroup::Settings.as_str())
        .with_context_predicate(id!("Workspace")),
        EditableBinding::new(
            "workspace:show_mcp_servers_settings_page",
            BindingDescription::new(t!("command-open-settings-mcp-servers")),
            WorkspaceAction::ShowSettingsPage(SettingsSection::MCPServers),
        )
        .with_group(bindings::BindingGroup::Settings.as_str())
        .with_context_predicate(id!("Workspace")),
        EditableBinding::new(
            "workspace:open_settings_file",
            BindingDescription::fluent("binding-workspace-open-settings-file"),
            WorkspaceAction::OpenSettingsFile,
        )
        .with_enabled(|| FeatureFlag::SettingsFile.is_enabled() && cfg!(feature = "local_fs"))
        .with_group(bindings::BindingGroup::Settings.as_str())
        .with_context_predicate(id!("Workspace")),
    ]);
}

fn add_overflow_menu_items_as_editable_binding(app: &mut AppContext) {
    use warpui::keymap::macros::*;

    // Add the ability to open all overflow menu items to the command palette.
    app.register_editable_bindings([
        EditableBinding::new(
            "workspace:show_invite_modal",
            BindingDescription::fluent("binding-workspace-invite-people"),
            WorkspaceAction::ShowReferralSettingsPage,
        )
        .with_context_predicate(id!("Workspace"))
        .with_custom_action(CustomAction::ReferAFriend),
        EditableBinding::new(
            "workspace:link_to_slack",
            BindingDescription::fluent("binding-workspace-link-to-slack"),
            WorkspaceAction::JoinSlack,
        )
        .with_context_predicate(id!("Workspace")),
        EditableBinding::new(
            "workspace:link_to_user_docs",
            BindingDescription::fluent("binding-workspace-link-to-user-docs"),
            WorkspaceAction::ViewUserDocs,
        )
        .with_context_predicate(id!("Workspace")),
        EditableBinding::new(
            "workspace:send_feedback",
            BindingDescription::new(t!("command-send-feedback")).with_dynamic_override(|ctx| {
                is_feedback_skill_available(ctx)
                    .then(|| t!("command-send-feedback-with-oz").into())
            }),
            WorkspaceAction::SendFeedback,
        )
        .with_context_predicate(id!("Workspace")),
        #[cfg(not(target_family = "wasm"))]
        EditableBinding::new(
            "workspace:view_logs",
            BindingDescription::fluent("binding-workspace-view-warp-logs"),
            WorkspaceAction::ViewLogs,
        )
        .with_context_predicate(id!("Workspace")),
        EditableBinding::new(
            "workspace:link_to_privacy_policy",
            BindingDescription::fluent("binding-workspace-link-to-privacy-policy"),
            WorkspaceAction::ViewPrivacyPolicy,
        )
        .with_context_predicate(id!("Workspace")),
    ]);
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct TabBarDropTargetData {
    pub tab_bar_location: TabBarLocation,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct VerticalTabsPaneDropTargetData {
    pub tab_bar_location: TabBarLocation,
}

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum TabBarLocation {
    TabIndex(usize),
    AfterTabIndex(usize), // Indicates any area after the tabs in the tab bar, includes the total tab count
}

impl DropTargetData for TabBarDropTargetData {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl DropTargetData for VerticalTabsPaneDropTargetData {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
