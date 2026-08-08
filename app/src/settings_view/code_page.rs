use std::borrow::Cow;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use ai::index::full_source_code_embedding::SyncProgress;
use ai::index::full_source_code_embedding::manager::{
    CodebaseIndexFinishedStatus, CodebaseIndexManager, CodebaseIndexManagerEvent,
    CodebaseIndexStatus, CodebaseIndexingError,
};
use ai::project_context::model::{ProjectContextModel, ProjectContextModelEvent};
use ai::workspace::WorkspaceMetadata;
use chrono::{DateTime, Utc};
use lsp::supported_servers::LSPServerType;
use lsp::{LspManagerModel, LspManagerModelEvent, LspServerModel, LspState};
use pathfinder_color::ColorU;
#[cfg(not(target_family = "wasm"))]
use remote_server::codebase_index_proto::{RemoteCodebaseIndexState, RemoteCodebaseIndexStatus};
use warp_core::features::FeatureFlag;
use warp_core::settings::ToggleableSetting as _;
use warp_core::ui::theme::{AnsiColorIdentifier, Fill as ThemeFill};
use warp_errors::report_if_error;
use warp_util::path::user_friendly_path;
#[cfg(not(target_family = "wasm"))]
use warp_util::remote_path::RemotePath;
use warpui::elements::{
    ChildView, ConstrainedBox, Container, CornerRadius, CrossAxisAlignment, Element, Empty,
    Expanded, Fill, Flex, MainAxisAlignment, MainAxisSize, MouseStateHandle, ParentElement, Radius,
    Shrinkable,
};
use warpui::fonts::Weight;
use warpui::keymap::ContextPredicate;
use warpui::platform::{Cursor, FilePickerConfiguration};
use warpui::ui_components::button::ButtonVariant;
use warpui::ui_components::components::{Coords, UiComponent, UiComponentStyles};
use warpui::ui_components::switch::{SwitchStateHandle, TooltipConfig};
use warpui::{
    Action, AppContext, Entity, ModelHandle, SingletonEntity, TypedActionView, View, ViewContext,
    ViewHandle, id,
};

#[cfg(feature = "local_fs")]
use super::features::external_editor::ExternalEditorView;
use super::settings_page::{
    Category, MatchData, PageType, SettingsPageMeta, SettingsPageViewHandle, SettingsWidget,
    TOGGLE_BUTTON_RIGHT_PADDING, render_body_item, render_separator,
};
use super::{
    LocalOnlyIconState, SettingsAction, SettingsSection, ToggleSettingActionPair, ToggleState,
    flags,
};
use crate::ai::persisted_workspace::{
    EnablementState, LspRepoStatus, PersistedWorkspace, PersistedWorkspaceEvent,
};
use crate::appearance::Appearance;
use crate::code::buffer_location::LocalOrRemotePath;
use crate::code::lsp_telemetry::{LspControlActionType, LspEnablementSource, LspTelemetryEvent};
#[cfg(not(target_family = "wasm"))]
use crate::remote_server::codebase_index_model::{
    RemoteCodebaseIndexModel, RemoteCodebaseIndexModelEvent, RemoteCodebaseIndexSettingsEntry,
};
use crate::settings::{AISettings, CodeSettings};
use crate::terminal::general_settings::GeneralSettings;
use crate::ui_components::avatar::{Avatar, AvatarContent, StatusElementTypes};
use crate::ui_components::buttons::icon_button;
use crate::ui_components::icons::Icon;
use crate::view_components::DismissibleToast;
use crate::view_components::action_button::{ActionButton, SecondaryTheme};
use crate::workspace::ToastStack;
use crate::workspace::tab_settings::TabSettings;
use crate::workspaces::update_manager::TeamUpdateManager;
use crate::workspaces::user_workspaces::UserWorkspaces;
use crate::workspaces::workspace::AdminEnablementSetting;
use crate::{TelemetryEvent, send_telemetry_from_ctx};

const MAIN_SECTION_MARGIN: f32 = 12.;
const SUB_SECTION_MARGIN: f32 = 8.;

const STATUS_ICON_SIZE: f32 = 16.;
const LSP_STATUS_INDICATOR_SIZE: f32 = 8.;
// User-facing labels live in the warp_i18n bundles (settings-code-*); only the
// internal server-failure match string stays a bare literal.
#[cfg(not(target_family = "wasm"))]
const REMOTE_CODEBASE_INDEX_LIMIT_REACHED_FAILURE: &str =
    "maximum number of codebase indexes has been reached";

/// Identifies which subpage of the Code settings the user is viewing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodeSubpage {
    /// Codebase indexing and initialization settings.
    Indexing,
    /// External editor, code review panel, and project explorer settings.
    EditorAndCodeReview,
}

impl CodeSubpage {
    pub fn from_section(section: SettingsSection) -> Option<Self> {
        match section {
            SettingsSection::CodeIndexing => Some(Self::Indexing),
            SettingsSection::EditorAndCodeReview => Some(Self::EditorAndCodeReview),
            _ => None,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::Indexing => warp_i18n::t_static!("settings-code-subpage-indexing"),
            Self::EditorAndCodeReview => {
                warp_i18n::t_static!("settings-code-subpage-editor-review")
            }
        }
    }
}

#[cfg(not(target_family = "wasm"))]
fn remote_codebase_index_limit_reached(status: &RemoteCodebaseIndexStatus) -> bool {
    status
        .failure_message
        .as_deref()
        .is_some_and(|message| message.contains(REMOTE_CODEBASE_INDEX_LIMIT_REACHED_FAILURE))
}

#[cfg(all(test, not(target_family = "wasm")))]
#[path = "code_page_tests.rs"]
mod tests;

#[derive(Clone, Default)]
struct LspServerRowMouseStates {
    restart: MouseStateHandle,
    #[cfg_attr(target_family = "wasm", allow(dead_code))]
    view_logs: MouseStateHandle,
    toggle: SwitchStateHandle,
    install: MouseStateHandle,
}

#[derive(Clone)]
struct InitializedFoldersMouseStates {
    codebase_manual_resync: Vec<MouseStateHandle>,
    codebase_delete: Vec<MouseStateHandle>,
    #[cfg(not(target_family = "wasm"))]
    remote_codebase_manual_resync: Vec<MouseStateHandle>,
    #[cfg(not(target_family = "wasm"))]
    remote_codebase_delete: Vec<MouseStateHandle>,
    lsp_rows: Vec<LspServerRowMouseStates>,
    open_project_rules: Vec<MouseStateHandle>,
}

#[derive(Clone)]
struct IndexingStatusPresentation {
    text: Cow<'static, str>,
    color: ColorU,
    icon: Option<Icon>,
    refresh_action: Option<IndexingRefreshAction>,
    show_delete: bool,
}

#[derive(Clone)]
enum IndexingRefreshAction {
    /// Remote rows use the same refresh icon for both "create an index for this remote path" and
    /// "refresh an existing index". Missing or disabled remote indexes need a request/create call
    /// because resync only applies once the daemon already has index state for that path.
    #[cfg_attr(target_family = "wasm", allow(dead_code))]
    RequestRemote,
    Resync,
}
pub struct CodeSettingsPageView {
    page: PageType<Self>,
    active_subpage: Option<CodeSubpage>,
    codebase_manual_resync_mouse_states: Vec<MouseStateHandle>,
    codebase_delete_mouse_states: Vec<MouseStateHandle>,
    #[cfg(not(target_family = "wasm"))]
    remote_codebase_manual_resync_mouse_states: Vec<MouseStateHandle>,
    #[cfg(not(target_family = "wasm"))]
    remote_codebase_delete_mouse_states: Vec<MouseStateHandle>,
    /// Mouse states for LSP server row buttons.
    /// This is kept separate from the codebase mouse states because each workspace/folder
    /// can have 0 to multiple LSP servers, so the count doesn't match 1:1 with workspaces.
    /// The states are flattened into a single Vec, indexed by iterating through workspaces
    /// and their enabled servers in order.
    lsp_row_mouse_states: Vec<LspServerRowMouseStates>,
    open_project_rules_mouse_states: Vec<MouseStateHandle>,
    /// Tracks installation status for suggested LSP servers so the UI can decide
    /// whether to show "Available for download" vs "Installed" and whether the
    /// "+" button should trigger install or just enable.
    suggested_server_statuses: HashMap<(PathBuf, LSPServerType), LspRepoStatus>,
    #[cfg(feature = "local_fs")]
    external_editor_view: Option<ViewHandle<ExternalEditorView>>,
}

impl CodeSettingsPageView {
    pub fn new(ctx: &mut ViewContext<CodeSettingsPageView>) -> Self {
        let index_manager = CodebaseIndexManager::handle(ctx);
        let codebase_count = index_manager
            .as_ref(ctx)
            .get_codebase_index_statuses(ctx)
            .count();

        ctx.subscribe_to_model(&index_manager, |me, index, event, ctx| {
            if matches!(
                event,
                CodebaseIndexManagerEvent::SyncStateUpdated { .. }
                    | CodebaseIndexManagerEvent::NewIndexCreated { .. }
            ) {
                let codebase_count = index.as_ref(ctx).get_codebase_index_statuses(ctx).count();

                // Only update mouse states if the number of codebases changed
                if me.codebase_manual_resync_mouse_states.len() != codebase_count {
                    // Resize the vector to match the new codebase count, but preserve the existing mouse states
                    me.codebase_manual_resync_mouse_states
                        .resize_with(codebase_count, Default::default);
                    me.codebase_delete_mouse_states
                        .resize_with(codebase_count, Default::default);
                }

                me.resize_workspace_mouse_states(ctx);

                ctx.notify();
            }
        });

        #[cfg(not(target_family = "wasm"))]
        let remote_codebase_count = {
            let remote_index_model = RemoteCodebaseIndexModel::handle(ctx);
            let remote_codebase_count = remote_index_model.as_ref(ctx).entries_for_settings().len();
            ctx.subscribe_to_model(&remote_index_model, |me, model, event, ctx| match event {
                RemoteCodebaseIndexModelEvent::SettingsEntriesChanged => {
                    let remote_codebase_count = model.as_ref(ctx).entries_for_settings().len();
                    if me.remote_codebase_manual_resync_mouse_states.len() != remote_codebase_count
                    {
                        me.remote_codebase_manual_resync_mouse_states
                            .resize_with(remote_codebase_count, Default::default);
                        me.remote_codebase_delete_mouse_states
                            .resize_with(remote_codebase_count, Default::default);
                    }
                    ctx.notify();
                }
            });
            remote_codebase_count
        };

        // Calculate total LSP server count across all workspaces (enabled + disabled + suggested)
        let lsp_server_count = PersistedWorkspace::as_ref(ctx).total_lsp_server_count(true);

        // Subscribe to LSP manager events for real-time status updates
        ctx.subscribe_to_model(
            &LspManagerModel::handle(ctx),
            |me, _, event, ctx| match event {
                LspManagerModelEvent::ServerStarted(_)
                | LspManagerModelEvent::ServerStopped(_)
                | LspManagerModelEvent::ServerRemoved { .. } => {
                    // Recalculate LSP server count and resize mouse states if needed
                    let new_count = PersistedWorkspace::as_ref(ctx).total_lsp_server_count(true);
                    if me.lsp_row_mouse_states.len() != new_count {
                        me.lsp_row_mouse_states
                            .resize_with(new_count, Default::default);
                    }

                    me.resize_workspace_mouse_states(ctx);

                    ctx.notify();
                }
            },
        );

        // Subscribe to PersistedWorkspaceEvent to handle suggested server detection
        // and installation status updates. We don't scan for suggested servers
        // here — PersistedWorkspace::new() already kicks off detection at startup
        // and emits AvailableServersDetected for each workspace, which the
        // subscription below handles.
        let persisted = PersistedWorkspace::handle(ctx);

        ctx.subscribe_to_model(&persisted, move |me, _model, event, ctx| match event {
            PersistedWorkspaceEvent::AvailableServersDetected {
                workspace_path,
                servers,
            } => {
                // New suggested servers detected — kick off install detection
                // and resize mouse states.
                for &server_type in servers {
                    #[cfg(feature = "local_fs")]
                    let status = PersistedWorkspace::handle(ctx).update(ctx, |model, ctx| {
                        model.detect_lsp_workspace_status(workspace_path.clone(), server_type, ctx)
                    });
                    #[cfg(not(feature = "local_fs"))]
                    let status = LspRepoStatus::CheckingForInstallation;
                    me.suggested_server_statuses
                        .insert((workspace_path.clone(), server_type), status);
                }
                let new_count = PersistedWorkspace::as_ref(ctx).total_lsp_server_count(true);
                if me.lsp_row_mouse_states.len() != new_count {
                    me.lsp_row_mouse_states
                        .resize_with(new_count, Default::default);
                }
                me.resize_workspace_mouse_states(ctx);
                ctx.notify();
            }
            PersistedWorkspaceEvent::InstallStatusUpdate {
                server_type,
                status,
            } => {
                let new_status = LspRepoStatus::from_installation_status(status, *server_type);
                for ((_, st), repo_status) in &mut me.suggested_server_statuses {
                    if *st == *server_type {
                        *repo_status = new_status.clone();
                    }
                }
                ctx.notify();
            }
            PersistedWorkspaceEvent::InstallationSucceeded
            | PersistedWorkspaceEvent::InstallationFailed
            | PersistedWorkspaceEvent::WorkspaceAdded { .. } => {
                ctx.notify();
            }
        });

        // Re-render when project rules are added or removed so the
        // "Open project rules" button visibility stays up to date.
        ctx.subscribe_to_model(&ProjectContextModel::handle(ctx), |_me, _, event, ctx| {
            if matches!(event, ProjectContextModelEvent::KnownRulesChanged(_)) {
                ctx.notify();
            }
        });

        let manual_add_directory_button = ctx.add_typed_action_view(|_| {
            ActionButton::new(
                warp_i18n::t!("settings-code-index-new-folder"),
                SecondaryTheme,
            )
            .with_icon(Icon::FindAll)
            .on_click(|ctx| {
                ctx.dispatch_typed_action(CodeSettingsPageAction::ManualAddDirectory);
            })
        });

        let code_page_widget = CodePageWidget {
            switch_state: Default::default(),
            auto_index_switch_state: Default::default(),
            manual_add_directory_button,
        };

        let workspace_count = PersistedWorkspace::as_ref(ctx).workspaces().count();

        #[cfg(feature = "local_fs")]
        let external_editor_view;
        let page = if FeatureFlag::OpenWarpNewSettingsModes.is_enabled() {
            #[cfg(feature = "local_fs")]
            {
                external_editor_view = Some(ctx.add_typed_action_view(ExternalEditorView::new));
            }

            let codebase_indexing_widgets: Vec<Box<dyn SettingsWidget<View = Self>>> =
                vec![Box::new(CodebaseIndexingCategorizedWidget {
                    inner: code_page_widget,
                })];
            #[cfg(feature = "local_fs")]
            let mut code_editor_review_widgets: Vec<
                Box<dyn SettingsWidget<View = Self>>,
            > = vec![Box::new(ExternalEditorCodeWidget)];
            #[cfg(not(feature = "local_fs"))]
            let mut code_editor_review_widgets: Vec<
                Box<dyn SettingsWidget<View = Self>>,
            > = vec![];
            code_editor_review_widgets.extend([
                Box::new(AutoOpenCodeReviewPaneCodeWidget::default())
                    as Box<dyn SettingsWidget<View = Self>>,
                Box::new(CodeReviewPanelToggleWidget::default()),
                Box::new(CodeReviewDiffStatsToggleWidget::default()),
                Box::new(ProjectExplorerToggleWidget::default()),
                Box::new(GlobalSearchToggleWidget::default()),
                Box::new(ShowHiddenFilesToggleWidget::default()),
                Box::new(FormatOnSaveToggleWidget::default()),
                Box::new(AutoSaveToggleWidget::default()),
            ]);
            let categories = vec![
                Category::new(
                    warp_i18n::t_static!("settings-code-category-indexing"),
                    codebase_indexing_widgets,
                ),
                Category::new(
                    warp_i18n::t_static!("settings-code-category-editor-review"),
                    code_editor_review_widgets,
                ),
            ];
            PageType::new_categorized(categories, None)
        } else {
            #[cfg(feature = "local_fs")]
            {
                external_editor_view = None;
            }
            let widgets: Vec<Box<dyn SettingsWidget<View = Self>>> =
                vec![Box::new(code_page_widget)];
            PageType::new_uncategorized(widgets, None)
        };

        Self {
            page,
            active_subpage: None,
            codebase_manual_resync_mouse_states: (0..codebase_count)
                .map(|_| Default::default())
                .collect(),
            codebase_delete_mouse_states: (0..codebase_count).map(|_| Default::default()).collect(),
            #[cfg(not(target_family = "wasm"))]
            remote_codebase_manual_resync_mouse_states: (0..remote_codebase_count)
                .map(|_| Default::default())
                .collect(),
            #[cfg(not(target_family = "wasm"))]
            remote_codebase_delete_mouse_states: (0..remote_codebase_count)
                .map(|_| Default::default())
                .collect(),
            lsp_row_mouse_states: (0..lsp_server_count).map(|_| Default::default()).collect(),
            open_project_rules_mouse_states: (0..workspace_count)
                .map(|_| Default::default())
                .collect(),
            suggested_server_statuses: HashMap::new(),
            #[cfg(feature = "local_fs")]
            external_editor_view,
        }
    }

    /// Set the active subpage and rebuild the page to show only the relevant widgets.
    pub fn set_active_subpage(
        &mut self,
        subpage: Option<CodeSubpage>,
        ctx: &mut ViewContext<Self>,
    ) {
        if self.active_subpage != subpage {
            self.active_subpage = subpage;
            // Rebuild the page with the relevant widgets for the selected subpage,
            // or the full categorized page when subpage is None.
            if let Some(subpage) = subpage {
                let manual_add_directory_button = ctx.add_typed_action_view(|_| {
                    ActionButton::new(
                        warp_i18n::t!("settings-code-index-new-folder"),
                        SecondaryTheme,
                    )
                    .with_icon(Icon::FindAll)
                    .on_click(|ctx| {
                        ctx.dispatch_typed_action(CodeSettingsPageAction::ManualAddDirectory);
                    })
                });
                let mut widgets: Vec<Box<dyn SettingsWidget<View = Self>>> = Vec::new();
                match subpage {
                    CodeSubpage::Indexing => {
                        widgets.push(Box::new(CodebaseIndexingCategorizedWidget {
                            inner: CodePageWidget {
                                switch_state: Default::default(),
                                auto_index_switch_state: Default::default(),
                                manual_add_directory_button,
                            },
                        }));
                    }
                    CodeSubpage::EditorAndCodeReview => {
                        #[cfg(feature = "local_fs")]
                        widgets.push(Box::new(ExternalEditorCodeWidget));
                        widgets.extend([
                            Box::new(AutoOpenCodeReviewPaneCodeWidget::default())
                                as Box<dyn SettingsWidget<View = Self>>,
                            Box::new(CodeReviewPanelToggleWidget::default()),
                            Box::new(CodeReviewDiffStatsToggleWidget::default()),
                            Box::new(ProjectExplorerToggleWidget::default()),
                            Box::new(GlobalSearchToggleWidget::default()),
                            Box::new(ShowHiddenFilesToggleWidget::default()),
                            Box::new(FormatOnSaveToggleWidget::default()),
                            Box::new(AutoSaveToggleWidget::default()),
                        ]);
                    }
                }
                self.page = PageType::new_uncategorized(widgets, Some(subpage.title()));
            } else {
                // None: rebuild the full categorized page (all widgets).
                self.page = Self::build_full_page(ctx);
            }
            ctx.notify();
        }
    }

    /// Builds the full categorized page with all Code widgets.
    /// Used for the default/legacy view and when resetting to all-widgets mode for search.
    fn build_full_page(ctx: &mut ViewContext<Self>) -> PageType<Self> {
        if FeatureFlag::OpenWarpNewSettingsModes.is_enabled() {
            let manual_add_directory_button = ctx.add_typed_action_view(|_| {
                ActionButton::new(
                    warp_i18n::t!("settings-code-index-new-folder"),
                    SecondaryTheme,
                )
                .with_icon(Icon::FindAll)
                .on_click(|ctx| {
                    ctx.dispatch_typed_action(CodeSettingsPageAction::ManualAddDirectory);
                })
            });
            let code_page_widget = CodePageWidget {
                switch_state: Default::default(),
                auto_index_switch_state: Default::default(),
                manual_add_directory_button,
            };
            let codebase_indexing_widgets: Vec<Box<dyn SettingsWidget<View = Self>>> =
                vec![Box::new(CodebaseIndexingCategorizedWidget {
                    inner: code_page_widget,
                })];
            #[cfg(feature = "local_fs")]
            let mut code_editor_review_widgets: Vec<
                Box<dyn SettingsWidget<View = Self>>,
            > = vec![Box::new(ExternalEditorCodeWidget)];
            #[cfg(not(feature = "local_fs"))]
            let mut code_editor_review_widgets: Vec<
                Box<dyn SettingsWidget<View = Self>>,
            > = vec![];
            code_editor_review_widgets.extend([
                Box::new(AutoOpenCodeReviewPaneCodeWidget::default())
                    as Box<dyn SettingsWidget<View = Self>>,
                Box::new(CodeReviewPanelToggleWidget::default()),
                Box::new(CodeReviewDiffStatsToggleWidget::default()),
                Box::new(ProjectExplorerToggleWidget::default()),
                Box::new(GlobalSearchToggleWidget::default()),
                Box::new(ShowHiddenFilesToggleWidget::default()),
                Box::new(FormatOnSaveToggleWidget::default()),
                Box::new(AutoSaveToggleWidget::default()),
            ]);
            let categories = vec![
                Category::new(
                    warp_i18n::t_static!("settings-code-category-indexing"),
                    codebase_indexing_widgets,
                ),
                Category::new(
                    warp_i18n::t_static!("settings-code-category-editor-review"),
                    code_editor_review_widgets,
                ),
            ];
            PageType::new_categorized(categories, None)
        } else {
            let manual_add_directory_button = ctx.add_typed_action_view(|_| {
                ActionButton::new(
                    warp_i18n::t!("settings-code-index-new-folder"),
                    SecondaryTheme,
                )
                .with_icon(Icon::FindAll)
                .on_click(|ctx| {
                    ctx.dispatch_typed_action(CodeSettingsPageAction::ManualAddDirectory);
                })
            });
            let widgets: Vec<Box<dyn SettingsWidget<View = Self>>> =
                vec![Box::new(CodePageWidget {
                    switch_state: Default::default(),
                    auto_index_switch_state: Default::default(),
                    manual_add_directory_button,
                })];
            PageType::new_uncategorized(widgets, None)
        }
    }

    /// Resize `open_project_rules_mouse_states` to match the current workspace count.
    fn resize_workspace_mouse_states(&mut self, ctx: &AppContext) {
        let workspace_count = PersistedWorkspace::as_ref(ctx).workspaces().count();
        if self.open_project_rules_mouse_states.len() != workspace_count {
            self.open_project_rules_mouse_states
                .resize_with(workspace_count, Default::default);
        }
    }

    fn open_directory_picker(&mut self, ctx: &mut ViewContext<Self>) {
        let file_picker_config = FilePickerConfiguration::new().folders_only();
        let window_id = ctx.window_id();

        ctx.open_file_picker(
            move |result, ctx| match result {
                Ok(paths) => {
                    if let Some(directory_path) = paths.first() {
                        let path = PathBuf::from(directory_path);

                        CodebaseIndexManager::handle(ctx).update(ctx, |manager, ctx| {
                            manager.index_directory(path, ctx);
                        });
                    }
                }
                Err(err) => {
                    ToastStack::handle(ctx).update(ctx, |toast_stack, ctx| {
                        toast_stack.add_ephemeral_toast(
                            DismissibleToast::error(format!("{err}")),
                            window_id,
                            ctx,
                        );
                    });
                }
            },
            file_picker_config,
        );
    }
}

impl Entity for CodeSettingsPageView {
    type Event = CodeSettingsPageEvent;
}

impl View for CodeSettingsPageView {
    fn ui_name() -> &'static str {
        "CodePage"
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        self.page.render(self, app)
    }
}

#[derive(Debug, Clone)]
pub enum CodeSettingsPageEvent {
    SignupAnonymousUser,
    OpenLspLogs { log_path: PathBuf },
    OpenProjectRules { rule_paths: Vec<PathBuf> },
}

// Define the code page actions.
#[derive(Debug, Clone)]
pub enum CodeSettingsPageAction {
    ToggleCodebaseContext,
    ToggleAutoIndexing,
    ManualResync(PathBuf),
    DeleteIndex(PathBuf),
    #[cfg(not(target_family = "wasm"))]
    RequestRemoteIndex(RemotePath),
    #[cfg(not(target_family = "wasm"))]
    ManualResyncRemote(RemotePath),
    #[cfg(not(target_family = "wasm"))]
    DeleteRemoteIndex(RemotePath),
    ManualAddDirectory,
    SignupAnonymousUser,
    /// Toggle an LSP server on/off for a workspace.
    ToggleLspServer {
        workspace_path: PathBuf,
        server_type: LSPServerType,
        currently_enabled: bool,
    },
    RestartLspServer {
        server: ModelHandle<LspServerModel>,
    },
    OpenLspLogs {
        log_path: PathBuf,
    },
    OpenProjectRules {
        rule_paths: Vec<PathBuf>,
    },
    ToggleCodeReviewPanel,
    ToggleShowCodeReviewDiffStats,
    ToggleAutoOpenCodeReviewPane,
    ToggleProjectExplorer,
    ToggleGlobalSearch,
    ToggleShowHiddenFiles,
    ToggleFormatOnSave,
    ToggleAutoSave,
    /// Install (if needed) and enable a suggested LSP server.
    InstallAndEnableLspServer {
        workspace_path: PathBuf,
        server_type: LSPServerType,
    },
    /// Enable a suggested LSP server that is already installed.
    EnableSuggestedLspServer {
        workspace_path: PathBuf,
        server_type: LSPServerType,
    },
}

impl TypedActionView for CodeSettingsPageView {
    type Action = CodeSettingsPageAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            CodeSettingsPageAction::ToggleCodebaseContext => {
                // If the organization has an explicit setting (on or off), ignore user toggles.
                // Auggie-backed indexing runs entirely locally and does not honour the cloud
                // admin policy, so toggling is always allowed for it.
                let is_auggie_backend =
                    crate::ai::codebase_index_backend::is_local_codebase_index_backend(ctx);
                let setting = UserWorkspaces::as_ref(ctx).team_allows_codebase_context();
                match setting {
                    AdminEnablementSetting::Enable | AdminEnablementSetting::Disable
                        if !is_auggie_backend =>
                    {
                        return;
                    }
                    _ => {
                        // Allow user to toggle
                    }
                }

                CodeSettings::handle(ctx).update(ctx, |settings, ctx| {
                    match settings.codebase_context_enabled.toggle_and_save_value(ctx) {
                        Ok(new_value) => {
                            send_telemetry_from_ctx!(
                                TelemetryEvent::ToggleCodebaseContext {
                                    is_codebase_context_enabled: new_value
                                },
                                ctx
                            );
                        }
                        Err(e) => {
                            log::warn!("Failed to set value for Codebase Context: {e:?}");
                        }
                    }
                });

                ctx.notify();
            }
            CodeSettingsPageAction::ToggleAutoIndexing => {
                CodeSettings::handle(ctx).update(ctx, |settings, ctx| {
                    match settings.auto_indexing_enabled.toggle_and_save_value(ctx) {
                        Ok(new_value) => {
                            send_telemetry_from_ctx!(
                                TelemetryEvent::ToggleAutoIndexing {
                                    is_autoindexing_enabled: new_value
                                },
                                ctx
                            );
                        }
                        Err(e) => {
                            log::warn!("Failed to set value for auto indexing: {e:?}");
                        }
                    }
                });

                ctx.notify();
            }
            CodeSettingsPageAction::ManualResync(repo_path) => {
                CodebaseIndexManager::handle(ctx).update(ctx, |manager, ctx| {
                    manager.try_manual_resync_codebase(repo_path, ctx);
                });
            }
            CodeSettingsPageAction::DeleteIndex(repo_path) => {
                CodebaseIndexManager::handle(ctx).update(ctx, |manager, ctx| {
                    manager.drop_index(repo_path.clone(), ctx);
                });
            }
            #[cfg(not(target_family = "wasm"))]
            CodeSettingsPageAction::RequestRemoteIndex(remote_path) => {
                RemoteCodebaseIndexModel::handle(ctx).update(ctx, |model, ctx| {
                    model.request_index(remote_path.clone(), ctx);
                });
            }
            #[cfg(not(target_family = "wasm"))]
            CodeSettingsPageAction::ManualResyncRemote(remote_path) => {
                RemoteCodebaseIndexModel::handle(ctx).update(ctx, |model, ctx| {
                    model.resync_index(remote_path.clone(), ctx);
                });
            }
            #[cfg(not(target_family = "wasm"))]
            CodeSettingsPageAction::DeleteRemoteIndex(remote_path) => {
                RemoteCodebaseIndexModel::handle(ctx).update(ctx, |model, ctx| {
                    model.drop_index(remote_path.clone(), ctx);
                });
            }
            CodeSettingsPageAction::ManualAddDirectory => {
                self.open_directory_picker(ctx);
            }
            CodeSettingsPageAction::SignupAnonymousUser => {
                ctx.emit(CodeSettingsPageEvent::SignupAnonymousUser);
            }
            CodeSettingsPageAction::ToggleLspServer {
                workspace_path,
                server_type,
                currently_enabled,
            } => {
                if *currently_enabled {
                    // Toggling OFF: stop and disable
                    send_telemetry_from_ctx!(
                        LspTelemetryEvent::ServerRemoved {
                            server_type: server_type.binary_name().to_string(),
                            source: LspEnablementSource::Settings,
                        },
                        ctx
                    );
                    LspManagerModel::handle(ctx).update(ctx, |manager, ctx| {
                        manager.remove_server(workspace_path, *server_type, ctx);
                    });
                    PersistedWorkspace::handle(ctx).update(ctx, |workspace, _| {
                        workspace.disable_lsp_server_for_path(workspace_path, *server_type);
                    });
                } else {
                    // Toggling ON: enable and spawn
                    send_telemetry_from_ctx!(
                        LspTelemetryEvent::ServerEnabled {
                            server_type: server_type.binary_name().to_string(),
                            source: LspEnablementSource::Settings,
                            needed_install: false,
                        },
                        ctx
                    );
                    let workspace_path = workspace_path.clone();
                    PersistedWorkspace::handle(ctx).update(ctx, |workspace, _ctx| {
                        workspace.enable_lsp_server_for_path(&workspace_path, *server_type);
                        #[cfg(feature = "local_fs")]
                        workspace.execute_lsp_task(
                            crate::ai::persisted_workspace::LspTask::Spawn {
                                file_path: workspace_path,
                            },
                            _ctx,
                        );
                    });
                }
                ctx.notify();
            }
            CodeSettingsPageAction::RestartLspServer { server } => {
                let server_name = server.as_ref(ctx).server_name();
                send_telemetry_from_ctx!(
                    LspTelemetryEvent::ControlAction {
                        action: LspControlActionType::Restart,
                        server_type: Some(server_name),
                    },
                    ctx
                );
                server.update(ctx, |server, ctx| {
                    server.restart(ctx);
                });
            }
            CodeSettingsPageAction::OpenLspLogs { log_path } => {
                send_telemetry_from_ctx!(
                    LspTelemetryEvent::ControlAction {
                        action: LspControlActionType::OpenLogs,
                        server_type: None,
                    },
                    ctx
                );
                ctx.emit(CodeSettingsPageEvent::OpenLspLogs {
                    log_path: log_path.clone(),
                });
            }
            CodeSettingsPageAction::OpenProjectRules { rule_paths } => {
                ctx.emit(CodeSettingsPageEvent::OpenProjectRules {
                    rule_paths: rule_paths.clone(),
                });
            }
            CodeSettingsPageAction::ToggleCodeReviewPanel => {
                TabSettings::handle(ctx).update(ctx, |settings, ctx| {
                    report_if_error!(settings.show_code_review_button.toggle_and_save_value(ctx));
                });
                ctx.notify();
            }
            CodeSettingsPageAction::ToggleShowCodeReviewDiffStats => {
                TabSettings::handle(ctx).update(ctx, |settings, ctx| {
                    report_if_error!(
                        settings
                            .show_code_review_diff_stats
                            .toggle_and_save_value(ctx)
                    );
                });
                ctx.notify();
            }
            CodeSettingsPageAction::ToggleProjectExplorer => {
                CodeSettings::handle(ctx).update(ctx, |settings, ctx| {
                    report_if_error!(settings.show_project_explorer.toggle_and_save_value(ctx));
                });
                ctx.notify();
            }
            CodeSettingsPageAction::ToggleGlobalSearch => {
                CodeSettings::handle(ctx).update(ctx, |settings, ctx| {
                    report_if_error!(settings.show_global_search.toggle_and_save_value(ctx));
                });
                ctx.notify();
            }
            CodeSettingsPageAction::ToggleShowHiddenFiles => {
                CodeSettings::handle(ctx).update(ctx, |settings, ctx| {
                    report_if_error!(settings.show_hidden_files.toggle_and_save_value(ctx));
                });
                ctx.notify();
            }
            CodeSettingsPageAction::ToggleFormatOnSave => {
                CodeSettings::handle(ctx).update(ctx, |settings, ctx| {
                    report_if_error!(settings.format_on_save.toggle_and_save_value(ctx));
                });
                ctx.notify();
            }
            CodeSettingsPageAction::ToggleAutoSave => {
                CodeSettings::handle(ctx).update(ctx, |settings, ctx| {
                    report_if_error!(settings.auto_save.toggle_and_save_value(ctx));
                });
                ctx.notify();
            }
            CodeSettingsPageAction::ToggleAutoOpenCodeReviewPane => {
                GeneralSettings::handle(ctx).update(ctx, |settings, ctx| {
                    report_if_error!(
                        settings
                            .auto_open_code_review_pane_on_first_agent_change
                            .toggle_and_save_value(ctx)
                    );
                });
                send_telemetry_from_ctx!(
                    TelemetryEvent::FeaturesPageAction {
                        action: "ToggleAutoOpenCodeReviewPane".to_string(),
                        value: format!(
                            "{}",
                            *GeneralSettings::as_ref(ctx)
                                .auto_open_code_review_pane_on_first_agent_change
                        )
                    },
                    ctx
                );
                ctx.notify();
            }
            CodeSettingsPageAction::InstallAndEnableLspServer {
                workspace_path,
                server_type,
            } => {
                send_telemetry_from_ctx!(
                    LspTelemetryEvent::ServerEnabled {
                        server_type: server_type.binary_name().to_string(),
                        source: LspEnablementSource::Settings,
                        needed_install: true,
                    },
                    ctx
                );
                #[cfg(feature = "local_fs")]
                {
                    let workspace_path = workspace_path.clone();
                    let server_type = *server_type;
                    PersistedWorkspace::handle(ctx).update(ctx, |workspace, _ctx| {
                        workspace.execute_lsp_task(
                            crate::ai::persisted_workspace::LspTask::Install {
                                file_path: workspace_path.clone(),
                                repo_root: workspace_path,
                                server_type,
                            },
                            _ctx,
                        );
                    });
                }
                #[cfg(not(feature = "local_fs"))]
                let _ = workspace_path;
                ctx.notify();
            }
            CodeSettingsPageAction::EnableSuggestedLspServer {
                workspace_path,
                server_type,
            } => {
                send_telemetry_from_ctx!(
                    LspTelemetryEvent::ServerEnabled {
                        server_type: server_type.binary_name().to_string(),
                        source: LspEnablementSource::Settings,
                        needed_install: false,
                    },
                    ctx
                );
                let workspace_path = workspace_path.clone();
                let server_type = *server_type;
                PersistedWorkspace::handle(ctx).update(ctx, |workspace, _ctx| {
                    workspace.enable_lsp_server_for_path(&workspace_path, server_type);
                    #[cfg(feature = "local_fs")]
                    workspace.execute_lsp_task(
                        crate::ai::persisted_workspace::LspTask::Spawn {
                            file_path: workspace_path,
                        },
                        _ctx,
                    );
                });
                ctx.notify();
            }
        }
    }
}

pub fn init_actions_from_parent_view<T: Action + Clone>(
    app: &mut AppContext,
    context: &ContextPredicate,
    builder: fn(SettingsAction) -> T,
) {
    if crate::ai::codebase_index_backend::is_codebase_index_feature_available(app) {
        ToggleSettingActionPair::add_toggle_setting_action_pairs_as_bindings(
            vec![ToggleSettingActionPair::new(
                "codebase index",
                builder(SettingsAction::Code(
                    CodeSettingsPageAction::ToggleCodebaseContext,
                )),
                &(context.clone() & id!(flags::IS_ANY_AI_ENABLED)),
                flags::IS_CODEBASE_INDEXING_ENABLED,
            )],
            app,
        );

        ToggleSettingActionPair::add_toggle_setting_action_pairs_as_bindings(
            vec![ToggleSettingActionPair::new(
                "auto-indexing",
                builder(SettingsAction::Code(
                    CodeSettingsPageAction::ToggleAutoIndexing,
                )),
                &(context.clone() & id!(flags::IS_CODEBASE_INDEXING_ENABLED)),
                flags::IS_AUTOINDEXING_ENABLED,
            )],
            app,
        );
    }

    if FeatureFlag::OpenWarpNewSettingsModes.is_enabled() {
        ToggleSettingActionPair::add_toggle_setting_action_pairs_as_bindings(
            vec![
                ToggleSettingActionPair::new(
                    "auto open code review panel",
                    builder(SettingsAction::Code(
                        CodeSettingsPageAction::ToggleAutoOpenCodeReviewPane,
                    )),
                    context,
                    flags::AUTO_OPEN_CODE_REVIEW_PANE_FLAG,
                ),
                ToggleSettingActionPair::new(
                    "code review button",
                    builder(SettingsAction::Code(
                        CodeSettingsPageAction::ToggleCodeReviewPanel,
                    )),
                    context,
                    flags::SHOW_CODE_REVIEW_BUTTON_FLAG,
                ),
                ToggleSettingActionPair::new(
                    "diff stats on code review button",
                    builder(SettingsAction::Code(
                        CodeSettingsPageAction::ToggleShowCodeReviewDiffStats,
                    )),
                    context,
                    flags::SHOW_CODE_REVIEW_DIFF_STATS_FLAG,
                ),
                ToggleSettingActionPair::new(
                    "project explorer",
                    builder(SettingsAction::Code(
                        CodeSettingsPageAction::ToggleProjectExplorer,
                    )),
                    context,
                    flags::SHOW_PROJECT_EXPLORER,
                ),
                ToggleSettingActionPair::new(
                    "global file search",
                    builder(SettingsAction::Code(
                        CodeSettingsPageAction::ToggleGlobalSearch,
                    )),
                    context,
                    flags::SHOW_GLOBAL_SEARCH,
                ),
                ToggleSettingActionPair::new(
                    "show hidden files in project explorer",
                    builder(SettingsAction::Code(
                        CodeSettingsPageAction::ToggleShowHiddenFiles,
                    )),
                    context,
                    flags::SHOW_HIDDEN_FILES,
                ),
            ],
            app,
        );
    }
}

struct CodePageWidget {
    switch_state: SwitchStateHandle,
    auto_index_switch_state: SwitchStateHandle,
    manual_add_directory_button: ViewHandle<ActionButton>,
}

/// Disabled-tooltip resolver shared by both the legacy `CodePageWidget` and the
/// new `CodebaseIndexingCategorizedWidget`. Returns `None` when the toggle
/// should be interactive. Auggie collapses every cloud admin/global-AI arm
/// down to a single "MCP unavailable" tooltip (or `None` when optimistic).
/// Composes the dim metadata bar shown beneath the indexing status row.
/// Returns `None` when the index has neither completed a sync nor failed.
fn build_index_stats_line(
    workspace: &WorkspaceMetadata,
    last_sync_result: Option<&CodebaseIndexFinishedStatus>,
) -> Option<String> {
    if let Some(CodebaseIndexFinishedStatus::Failed(err)) = last_sync_result {
        return Some(error_reason_text(err));
    }

    let mut parts: Vec<String> = Vec::new();
    if let Some(n) = workspace.file_count {
        parts.push(warp_i18n::t!("settings-code-stats-files", count = n));
    }
    if let Some(n) = workspace.fragment_count {
        parts.push(warp_i18n::t!("settings-code-stats-chunks", count = n));
    }
    if let Some(b) = workspace.index_bytes {
        parts.push(humanize_bytes_label(b));
    }
    if let Some(ts) = workspace.synced_at {
        parts.push(warp_i18n::t!(
            "settings-code-stats-synced",
            ago = relative_time_label(ts)
        ));
    }
    if workspace.query_count > 0 {
        let last = workspace
            .queried_ts
            .map(relative_time_label)
            .unwrap_or_default();
        parts.push(warp_i18n::t!(
            "settings-code-stats-queried",
            count = workspace.query_count,
            ago = last
        ));
    }
    (!parts.is_empty()).then(|| parts.join(" · "))
}

fn humanize_bytes_label(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    let formatted = if bytes < KB {
        format!("{} B", bytes)
    } else if bytes < MB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else if bytes < GB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    };
    warp_i18n::t!("settings-code-stats-bytes", size = formatted)
}

fn relative_time_label(ts: DateTime<Utc>) -> String {
    let secs = Utc::now().signed_duration_since(ts).num_seconds().max(0);
    if secs < 60 {
        warp_i18n::t!("settings-code-stats-just-now")
    } else if secs < 3_600 {
        warp_i18n::t!("settings-code-stats-minutes-ago", minutes = secs / 60)
    } else if secs < 86_400 {
        warp_i18n::t!("settings-code-stats-hours-ago", hours = secs / 3_600)
    } else {
        warp_i18n::t!("settings-code-stats-days-ago", days = secs / 86_400)
    }
}

/// Squeezes a free-form error message down to one short line so the metadata
/// bar never balloons across multiple rows. Falls back to "(no error message)"
/// when the source is empty.
const ERROR_REASON_MAX_LEN: usize = 200;

fn truncate_error_reason(raw: &str) -> String {
    let first_line = raw.lines().next().unwrap_or("").trim();
    if first_line.is_empty() {
        return "(no error message)".to_string();
    }
    if first_line.chars().count() <= ERROR_REASON_MAX_LEN {
        return first_line.to_string();
    }
    let truncated: String = first_line.chars().take(ERROR_REASON_MAX_LEN).collect();
    format!("{truncated}…")
}

fn error_reason_text(err: &CodebaseIndexingError) -> String {
    match err {
        CodebaseIndexingError::BuildTreeError => warp_i18n::t!("settings-code-error-build-tree"),
        CodebaseIndexingError::ExceededMaxFileLimit => {
            warp_i18n::t!("settings-code-error-too-many-files")
        }
        CodebaseIndexingError::MaxDepthExceeded => warp_i18n::t!("settings-code-error-max-depth"),
        CodebaseIndexingError::FailedToGenerateEmbeddings(metas) => {
            warp_i18n::t!("settings-code-error-embedding", count = metas.len())
        }
        CodebaseIndexingError::FailedToSyncIntermediateNodes(hashes) => {
            warp_i18n::t!("settings-code-error-sync", count = hashes.len())
        }
        CodebaseIndexingError::Other(e) => {
            warp_i18n::t!(
                "settings-code-error-other",
                reason = truncate_error_reason(&e.to_string())
            )
        }
    }
}

fn codebase_indexing_disabled_tooltip(
    admin_setting: AdminEnablementSetting,
    global_ai_enabled: bool,
    is_auggie_backend: bool,
    is_auggie_unavailable: bool,
) -> Option<String> {
    if is_auggie_backend {
        return is_auggie_unavailable.then(|| {
            warp_i18n::t!("settings-code-indexing-disabled-auggie-unavailable").to_string()
        });
    }
    match admin_setting {
        AdminEnablementSetting::Enable => {
            Some(warp_i18n::t!("settings-code-indexing-workspace-enabled-admin").to_string())
        }
        AdminEnablementSetting::Disable => {
            Some(warp_i18n::t!("settings-code-indexing-disabled-admin").to_string())
        }
        AdminEnablementSetting::RespectUserSetting if !global_ai_enabled => {
            Some(warp_i18n::t!("settings-code-indexing-disabled-global-ai").to_string())
        }
        AdminEnablementSetting::RespectUserSetting => None,
    }
}

impl SettingsWidget for CodePageWidget {
    type View = CodeSettingsPageView;

    fn search_terms(&self) -> &str {
        "code coding codebase repository index indexing indices context path lsp language server"
    }

    fn render(
        &self,
        view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let mut content = Flex::column();

        let global_ai_enabled = AISettings::as_ref(app).is_any_ai_enabled(app);

        // Main "Code" header
        content.add_child(self.render_code_header(appearance));

        // Initialization Settings section
        content.add_child(render_separator(appearance));
        content.add_child(self.render_initialization_settings_header(appearance));
        content.add_child(self.render_codebase_indexing_toggle_row(
            global_ai_enabled,
            appearance,
            app,
        ));
        content.add_child(self.render_settings_subtext(
            global_ai_enabled,
            warp_i18n::t!("settings-code-codebase-index-description"),
            appearance,
        ));
        content.add_child(self.render_settings_subtext(
            global_ai_enabled,
            warp_i18n::t!("settings-code-warp-indexing-ignore-description"),
            appearance,
        ));

        let codebase_context_enabled =
            crate::ai::codebase_index_backend::is_codebase_context_enabled_for_indexing(app);
        if codebase_context_enabled {
            content.add_children(self.render_autoindexing_rows(appearance, app));
        }

        // Initialized / indexed folders section
        content.add_child(render_separator(appearance));
        let mouse_states = InitializedFoldersMouseStates {
            codebase_manual_resync: view.codebase_manual_resync_mouse_states.clone(),
            codebase_delete: view.codebase_delete_mouse_states.clone(),
            #[cfg(not(target_family = "wasm"))]
            remote_codebase_manual_resync: view.remote_codebase_manual_resync_mouse_states.clone(),
            #[cfg(not(target_family = "wasm"))]
            remote_codebase_delete: view.remote_codebase_delete_mouse_states.clone(),
            lsp_rows: view.lsp_row_mouse_states.clone(),
            open_project_rules: view.open_project_rules_mouse_states.clone(),
        };

        content.add_child(self.render_initialized_folders(
            mouse_states,
            &view.suggested_server_statuses,
            appearance,
            app,
        ));

        Container::new(content.finish())
            .with_uniform_padding(24.0)
            .finish()
    }
}

impl CodePageWidget {
    fn render_autoindexing_rows(
        &self,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Vec<Box<dyn Element>> {
        let auto_indexing_enabled = *CodeSettings::as_ref(app).auto_indexing_enabled;
        let codebase_indexing_enabled =
            crate::ai::codebase_index_backend::is_codebase_context_enabled_for_indexing(app);

        let mut rows = vec![
            self.render_autoindex_row(
                warp_i18n::t_static!("settings-code-auto-index-feature-name"),
                self.auto_index_switch_state.clone(),
                auto_indexing_enabled,
                CodeSettingsPageAction::ToggleAutoIndexing,
                appearance,
            ),
            // Use subtext styling for description (gray color per Figma)
            self.render_settings_subtext(
                codebase_indexing_enabled,
                warp_i18n::t!("settings-code-auto-index-description"),
                appearance,
            ),
        ];

        if codebase_indexing_enabled && !CodebaseIndexManager::as_ref(app).can_create_new_indices()
        {
            rows.push(self.render_settings_subtext(
                false,
                warp_i18n::t!("settings-code-index-limit-reached"),
                appearance,
            ));
        }

        rows.push(
            Container::new(Empty::new().finish())
                .with_margin_bottom(16.0)
                .finish(),
        );
        rows
    }

    fn render_autoindex_row(
        &self,
        label: &'static str,
        switch_state: SwitchStateHandle,
        auto_indexing_enabled: bool,
        action: CodeSettingsPageAction,
        appearance: &Appearance,
    ) -> Box<dyn Element> {
        let ui_builder = appearance.ui_builder();
        let theme = appearance.theme();

        Container::new(
            Flex::row()
                .with_main_axis_size(MainAxisSize::Max)
                .with_main_axis_alignment(MainAxisAlignment::SpaceBetween)
                .with_child(
                    ui_builder
                        .span(label)
                        .with_style(UiComponentStyles {
                            font_size: Some(16.0),
                            font_weight: Some(Weight::Semibold),
                            font_color: Some(theme.active_ui_text_color().into()),
                            ..Default::default()
                        })
                        .build()
                        .finish(),
                )
                .with_child(
                    Container::new(
                        ui_builder
                            .switch(switch_state)
                            .check(auto_indexing_enabled)
                            .build()
                            .on_click(move |ctx, _, _| {
                                ctx.dispatch_typed_action(action.clone());
                            })
                            .finish(),
                    )
                    .with_padding_right(TOGGLE_BUTTON_RIGHT_PADDING)
                    .finish(),
                )
                .finish(),
        )
        .with_padding_bottom(6.)
        .finish()
    }

    /// Renders a settings subtext description (gray color per Figma).
    fn render_settings_subtext(
        &self,
        _active: bool,
        description: impl Into<Cow<'static, str>>,
        appearance: &Appearance,
    ) -> Box<dyn Element> {
        let ui_builder = appearance.ui_builder();
        let theme = appearance.theme();

        // Per Figma: subtext uses disabled_ui_text_color (#9b9b9b)
        ui_builder
            .paragraph(description)
            .with_style(UiComponentStyles {
                font_color: Some(theme.disabled_ui_text_color().into()),
                ..Default::default()
            })
            .build()
            .with_margin_bottom(8.0)
            .finish()
    }

    /// Renders the main "Code" header.
    fn render_code_header(&self, appearance: &Appearance) -> Box<dyn Element> {
        let ui_builder = appearance.ui_builder();
        let theme = appearance.theme();

        Container::new(
            ui_builder
                .span(warp_i18n::t!("settings-code-title"))
                .with_style(UiComponentStyles {
                    font_size: Some(24.0),
                    font_weight: Some(Weight::Bold),
                    font_color: Some(theme.active_ui_text_color().into()),
                    ..Default::default()
                })
                .build()
                .finish(),
        )
        .with_padding_bottom(15.)
        .finish()
    }

    /// Renders the "Initialization Settings" section header.
    fn render_initialization_settings_header(&self, appearance: &Appearance) -> Box<dyn Element> {
        let ui_builder = appearance.ui_builder();
        let theme = appearance.theme();

        Container::new(
            ui_builder
                .span(warp_i18n::t!("settings-code-init-settings-header"))
                .with_style(UiComponentStyles {
                    font_size: Some(18.0),
                    font_weight: Some(Weight::Semibold),
                    font_color: Some(theme.active_ui_text_color().into()),
                    ..Default::default()
                })
                .build()
                .finish(),
        )
        .with_margin_top(8.)
        .with_margin_bottom(12.)
        .finish()
    }

    /// Renders the "Codebase indexing" toggle row (legacy layout).
    fn render_codebase_indexing_toggle_row(
        &self,
        global_ai_enabled: bool,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let ui_builder = appearance.ui_builder();
        let theme = appearance.theme();
        let admin_setting = UserWorkspaces::as_ref(app).team_allows_codebase_context();

        let label = ui_builder
            .span(warp_i18n::t!("settings-code-codebase-indexing-label"))
            .with_style(UiComponentStyles {
                font_size: Some(16.0),
                font_weight: Some(Weight::Semibold),
                font_color: Some(theme.active_ui_text_color().into()),
                ..Default::default()
            })
            .build()
            .finish();

        let is_auggie_backend =
            crate::ai::codebase_index_backend::is_local_codebase_index_backend(app);
        let is_auggie_unavailable =
            crate::ai::codebase_index_backend::is_auggie_backend_unavailable(app);

        let switch = ui_builder.switch(self.switch_state.clone()).check(
            crate::ai::codebase_index_backend::is_codebase_context_enabled_for_indexing(app),
        );

        let disabled_tooltip_text = codebase_indexing_disabled_tooltip(
            admin_setting,
            global_ai_enabled,
            is_auggie_backend,
            is_auggie_unavailable,
        );

        let toggle_element = if let Some(tooltip_text) = disabled_tooltip_text {
            switch
                .with_tooltip(TooltipConfig {
                    text: tooltip_text,
                    styles: ui_builder.default_tool_tip_styles(),
                })
                .disable()
                .build()
                .finish()
        } else {
            switch
                .build()
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(CodeSettingsPageAction::ToggleCodebaseContext);
                })
                .finish()
        };

        let toggle = Container::new(toggle_element)
            .with_padding_right(TOGGLE_BUTTON_RIGHT_PADDING)
            .finish();

        Container::new(
            Flex::row()
                .with_main_axis_size(MainAxisSize::Max)
                .with_main_axis_alignment(MainAxisAlignment::SpaceBetween)
                .with_cross_axis_alignment(CrossAxisAlignment::Center)
                .with_child(label)
                .with_child(toggle)
                .finish(),
        )
        .with_padding_bottom(6.)
        .finish()
    }

    /// Renders the "Initialized / indexed folders" section.
    fn render_initialized_folders(
        &self,
        mouse_states: InitializedFoldersMouseStates,
        suggested_server_statuses: &HashMap<(PathBuf, LSPServerType), LspRepoStatus>,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let ui_builder = appearance.ui_builder();
        let theme = appearance.theme();

        let InitializedFoldersMouseStates {
            codebase_manual_resync: codebase_manual_resync_mouse_states,
            codebase_delete: codebase_delete_mouse_states,
            #[cfg(not(target_family = "wasm"))]
                remote_codebase_manual_resync: remote_codebase_manual_resync_mouse_states,
            #[cfg(not(target_family = "wasm"))]
                remote_codebase_delete: remote_codebase_delete_mouse_states,
            lsp_rows: lsp_row_mouse_states,
            open_project_rules: open_project_rules_mouse_states,
        } = mouse_states;

        let mut content = Flex::column();

        // Section header with "Index folder" button
        content.add_child(
            Container::new(
                Flex::row()
                    .with_main_axis_size(MainAxisSize::Max)
                    .with_main_axis_alignment(MainAxisAlignment::SpaceBetween)
                    .with_cross_axis_alignment(CrossAxisAlignment::Center)
                    .with_child(
                        ui_builder
                            .span(warp_i18n::t!("settings-code-initialized-folders"))
                            .with_style(UiComponentStyles {
                                font_size: Some(16.0),
                                font_weight: Some(Weight::Semibold),
                                font_color: Some(theme.active_ui_text_color().into()),
                                ..Default::default()
                            })
                            .build()
                            .finish(),
                    )
                    .with_child(ChildView::new(&self.manual_add_directory_button).finish())
                    .finish(),
            )
            .with_margin_top(8.)
            .with_margin_bottom(12.)
            .finish(),
        );

        // Get workspaces from PersistedWorkspace
        let workspaces: Vec<WorkspaceMetadata> =
            PersistedWorkspace::as_ref(app).workspaces().collect();
        #[cfg(not(target_family = "wasm"))]
        let remote_entries = if FeatureFlag::RemoteCodebaseIndexing.is_enabled() {
            RemoteCodebaseIndexModel::as_ref(app).entries_for_settings()
        } else {
            Vec::new()
        };

        let codebase_manager = CodebaseIndexManager::as_ref(app);
        let lsp_manager = LspManagerModel::as_ref(app);
        let persisted_workspace = PersistedWorkspace::as_ref(app);

        let mut lsp_mouse_index = 0;
        let mut rendered_folder = false;

        for (workspace_idx, workspace) in workspaces.iter().enumerate() {
            let workspace_path = &workspace.path;

            // Get codebase index status if it exists
            let index_status =
                codebase_manager.get_codebase_index_status_for_path(workspace_path, app);

            // Get all LSP servers (enabled + disabled + suggested) for this workspace
            let all_servers: Vec<(LSPServerType, EnablementState)> = persisted_workspace
                .all_lsp_servers(workspace_path, true)
                .map(|iter| iter.collect())
                .unwrap_or_default();

            // Get mouse states for this workspace
            let resync_mouse = codebase_manual_resync_mouse_states
                .get(workspace_idx)
                .cloned()
                .unwrap_or_default();
            let delete_mouse = codebase_delete_mouse_states
                .get(workspace_idx)
                .cloned()
                .unwrap_or_default();

            // Skip workspaces that have neither an index nor any LSP servers
            if index_status.is_none() && all_servers.is_empty() {
                continue;
            }

            rendered_folder = true;

            // Get LSP server mouse states
            let lsp_mouse_states: Vec<LspServerRowMouseStates> = all_servers
                .iter()
                .map(|_| {
                    let state = lsp_row_mouse_states
                        .get(lsp_mouse_index)
                        .cloned()
                        .unwrap_or_default();

                    lsp_mouse_index += 1;

                    state
                })
                .collect();

            let open_rules_mouse = open_project_rules_mouse_states
                .get(workspace_idx)
                .cloned()
                .unwrap_or_default();

            content.add_child(self.render_workspace_row(
                workspace,
                index_status.as_ref(),
                &all_servers,
                lsp_manager,
                resync_mouse,
                delete_mouse,
                lsp_mouse_states,
                open_rules_mouse,
                suggested_server_statuses,
                appearance,
                app,
            ));
        }
        #[cfg(not(target_family = "wasm"))]
        for (entry_idx, entry) in remote_entries.iter().enumerate() {
            rendered_folder = true;
            let resync_mouse = remote_codebase_manual_resync_mouse_states
                .get(entry_idx)
                .cloned()
                .unwrap_or_default();
            let delete_mouse = remote_codebase_delete_mouse_states
                .get(entry_idx)
                .cloned()
                .unwrap_or_default();

            content.add_child(self.render_remote_workspace_row(
                entry,
                resync_mouse,
                delete_mouse,
                appearance,
            ));
        }

        if !rendered_folder {
            content.add_child(
                Container::new(
                    appearance
                        .ui_builder()
                        .paragraph(warp_i18n::t!("settings-code-no-folders-initialized"))
                        .build()
                        .finish(),
                )
                .with_margin_bottom(MAIN_SECTION_MARGIN)
                .finish(),
            );
        }

        content.finish()
    }

    /// Renders a single workspace row with its indexing status and LSP servers.
    #[allow(clippy::too_many_arguments)]
    fn render_workspace_row(
        &self,
        workspace: &WorkspaceMetadata,
        index_status: Option<&CodebaseIndexStatus>,
        all_servers: &[(LSPServerType, EnablementState)],
        lsp_manager: &LspManagerModel,
        resync_mouse: MouseStateHandle,
        delete_mouse: MouseStateHandle,
        lsp_mouse_states: Vec<LspServerRowMouseStates>,
        open_rules_mouse: MouseStateHandle,
        suggested_server_statuses: &HashMap<(PathBuf, LSPServerType), LspRepoStatus>,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let ui_builder = appearance.ui_builder();
        let theme = appearance.theme();
        let workspace_path = workspace.path.as_path();

        let mut workspace_content = Flex::column().with_spacing(MAIN_SECTION_MARGIN);

        // Workspace path header with "Open project rules" button
        let home_dir =
            dirs::home_dir().and_then(|home_dir| home_dir.to_str().map(|s| s.to_owned()));
        let user_friendly = user_friendly_path(
            workspace_path.to_string_lossy().as_ref(),
            home_dir.as_deref(),
        )
        .to_string();

        // Query ProjectContextModel for rules under this workspace
        let workspace_rule_paths =
            ProjectContextModel::as_ref(app).rules_for_workspace(workspace_path);

        // Only show "Open project rules" button if rules exist for this workspace
        let open_rules_button: Option<Box<dyn Element>> = if !workspace_rule_paths.is_empty() {
            let open_rules_button = ui_builder
                .button(ButtonVariant::Secondary, open_rules_mouse)
                .with_style(UiComponentStyles {
                    font_size: Some(12.),
                    padding: Some(Coords {
                        top: 4.,
                        bottom: 4.,
                        left: 8.,
                        right: 8.,
                    }),
                    ..Default::default()
                })
                .with_hovered_styles(UiComponentStyles {
                    background: Some(theme.surface_3().into()),
                    ..Default::default()
                })
                .with_text_and_icon_label(
                    warpui::ui_components::button::TextAndIcon::new(
                        warpui::ui_components::button::TextAndIconAlignment::IconFirst,
                        warp_i18n::t!("settings-code-open-project-rules"),
                        warpui::elements::Icon::new(
                            "bundled/svg/file-code-02.svg",
                            theme.foreground(),
                        ),
                        warpui::elements::MainAxisSize::Min,
                        warpui::elements::MainAxisAlignment::Center,
                        pathfinder_geometry::vector::vec2f(14., 14.),
                    )
                    .with_inner_padding(4.),
                )
                .build()
                .with_cursor(Cursor::PointingHand)
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(CodeSettingsPageAction::OpenProjectRules {
                        rule_paths: workspace_rule_paths.clone(),
                    });
                })
                .finish();
            Some(open_rules_button)
        } else {
            None
        };

        workspace_content.add_child(self.render_workspace_header(
            user_friendly,
            open_rules_button,
            appearance,
        ));

        // Indexing section (always rendered per design)
        workspace_content.add_child(self.render_indexing_subsection(
            workspace,
            index_status,
            resync_mouse,
            delete_mouse,
            appearance,
        ));

        // LSP Servers section (if any servers known)
        if !all_servers.is_empty() {
            workspace_content.add_child(self.render_lsp_servers_subsection(
                workspace_path,
                all_servers,
                lsp_manager,
                lsp_mouse_states,
                suggested_server_statuses,
                appearance,
                app,
            ));
        }

        self.render_workspace_row_container(workspace_content.finish(), appearance)
    }

    #[cfg(not(target_family = "wasm"))]
    fn render_remote_workspace_row(
        &self,
        entry: &RemoteCodebaseIndexSettingsEntry,
        resync_mouse: MouseStateHandle,
        delete_mouse: MouseStateHandle,
        appearance: &Appearance,
    ) -> Box<dyn Element> {
        let mut workspace_content = Flex::column().with_spacing(MAIN_SECTION_MARGIN);

        let remote_path_label = format!("{}:{}", entry.host_label, entry.remote_path.path.as_str());
        workspace_content.add_child(self.render_workspace_header(
            remote_path_label,
            None,
            appearance,
        ));

        workspace_content.add_child(self.render_indexing_subsection_for_target(
            self.remote_indexing_status_presentation(&entry.status, appearance),
            Some(LocalOrRemotePath::Remote(entry.remote_path.clone())),
            resync_mouse,
            delete_mouse,
            appearance,
        ));

        self.render_workspace_row_container(workspace_content.finish(), appearance)
    }

    fn render_workspace_header(
        &self,
        label: String,
        action_button: Option<Box<dyn Element>>,
        appearance: &Appearance,
    ) -> Box<dyn Element> {
        let theme = appearance.theme();
        let path_label = Shrinkable::new(
            1.,
            appearance
                .ui_builder()
                .span(label)
                .with_style(UiComponentStyles {
                    font_family_id: Some(appearance.monospace_font_family()),
                    font_size: Some(appearance.ui_font_size()),
                    font_weight: Some(Weight::Bold),
                    font_color: Some(theme.active_ui_text_color().into()),
                    ..Default::default()
                })
                .build()
                .finish(),
        )
        .finish();

        let mut header_row = Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_main_axis_alignment(MainAxisAlignment::SpaceBetween)
            .with_cross_axis_alignment(CrossAxisAlignment::Center);
        header_row.add_child(Expanded::new(1., path_label).finish());
        if let Some(action_button) = action_button {
            header_row.add_child(action_button);
        }
        header_row.finish()
    }

    fn render_workspace_row_container(
        &self,
        workspace_content: Box<dyn Element>,
        appearance: &Appearance,
    ) -> Box<dyn Element> {
        let theme = appearance.theme();
        Container::new(workspace_content)
            .with_uniform_padding(MAIN_SECTION_MARGIN)
            .with_background(theme.surface_1())
            .with_corner_radius(CornerRadius::with_all(Radius::Pixels(4.)))
            .with_margin_bottom(MAIN_SECTION_MARGIN)
            .finish()
    }

    /// Renders the indexing subsection within a workspace row.
    fn render_indexing_subsection(
        &self,
        workspace: &WorkspaceMetadata,
        index_status: Option<&CodebaseIndexStatus>,
        resync_mouse: MouseStateHandle,
        delete_mouse: MouseStateHandle,
        appearance: &Appearance,
    ) -> Box<dyn Element> {
        let workspace_path = workspace.path.as_path();
        let subsection = self.render_indexing_subsection_for_target(
            self.local_indexing_status_presentation(index_status, appearance),
            Some(LocalOrRemotePath::Local(workspace_path.to_path_buf())),
            resync_mouse,
            delete_mouse,
            appearance,
        );

        // Fork feature: dim metadata bar ("{N} files · {M} chunks · {SIZE} · synced {ago} …")
        // beneath the local indexing status row. Failure reason replaces the bar on error.
        let Some(stats_line) = index_status
            .and_then(|status| build_index_stats_line(workspace, status.last_sync_result()))
        else {
            return subsection;
        };

        let theme = appearance.theme();
        let mut column = Flex::column().with_spacing(SUB_SECTION_MARGIN);
        column.add_child(subsection);
        column.add_child(
            appearance
                .ui_builder()
                .label(stats_line)
                .with_style(UiComponentStyles {
                    font_size: Some(11.5),
                    font_color: Some(theme.disabled_ui_text_color().into()),
                    ..Default::default()
                })
                .build()
                .finish(),
        );
        column.finish()
    }

    fn render_indexing_subsection_for_target(
        &self,
        presentation: IndexingStatusPresentation,
        action_target: Option<LocalOrRemotePath>,
        resync_mouse: MouseStateHandle,
        delete_mouse: MouseStateHandle,
        appearance: &Appearance,
    ) -> Box<dyn Element> {
        let ui_builder = appearance.ui_builder();
        let theme = appearance.theme();

        let mut column = Flex::column().with_spacing(SUB_SECTION_MARGIN);
        column.add_child(
            ui_builder
                .span(warp_i18n::t!("settings-code-indexing-section"))
                .with_style(UiComponentStyles {
                    font_size: Some(11.0),
                    font_weight: Some(Weight::Semibold),
                    font_color: Some(theme.disabled_ui_text_color().into()),
                    ..Default::default()
                })
                .build()
                .finish(),
        );

        let (status_label, action_buttons) = self.render_index_status_parts(
            presentation,
            action_target,
            resync_mouse,
            delete_mouse,
            appearance,
        );
        column.add_child(
            Flex::row()
                .with_main_axis_size(MainAxisSize::Max)
                .with_main_axis_alignment(MainAxisAlignment::SpaceBetween)
                .with_cross_axis_alignment(CrossAxisAlignment::Center)
                .with_child(status_label)
                .with_child(action_buttons)
                .finish(),
        );

        column.finish()
    }

    fn local_indexing_status_presentation(
        &self,
        index_state: Option<&CodebaseIndexStatus>,
        appearance: &Appearance,
    ) -> IndexingStatusPresentation {
        let theme = appearance.theme();
        let Some(index_state) = index_state else {
            return IndexingStatusPresentation {
                text: Cow::from(warp_i18n::t!("settings-code-no-index-created").to_string()),
                color: theme.disabled_ui_text_color().into_solid(),
                icon: Some(Icon::SlashCircle),
                refresh_action: None,
                show_delete: false,
            };
        };

        if index_state.has_pending() {
            let text: Cow<'static, str> = match index_state.sync_progress() {
                Some(SyncProgress::Discovering { total_nodes }) => Cow::from(
                    warp_i18n::t!(
                        "settings-code-status-discovered-chunks",
                        count = total_nodes
                    )
                    .to_string(),
                ),
                Some(SyncProgress::Syncing {
                    completed_nodes,
                    total_nodes,
                }) => Cow::from(
                    warp_i18n::t!(
                        "settings-code-status-syncing-progress",
                        completed = completed_nodes,
                        total = total_nodes
                    )
                    .to_string(),
                ),
                None => Cow::from(warp_i18n::t!("settings-code-status-syncing").to_string()),
            };

            return IndexingStatusPresentation {
                text,
                color: theme.disabled_ui_text_color().into_solid(),
                icon: None,
                refresh_action: None,
                show_delete: true,
            };
        }

        if let Some(completed_successfully) = index_state.last_sync_successful() {
            let (text, color, icon): (Cow<'static, str>, _, _) = if completed_successfully {
                (
                    Cow::from(warp_i18n::t!("settings-code-status-synced").to_string()),
                    theme.ansi_fg_green(),
                    Icon::Check,
                )
            } else if let Some(CodebaseIndexFinishedStatus::Failed(
                CodebaseIndexingError::ExceededMaxFileLimit
                | CodebaseIndexingError::MaxDepthExceeded,
            )) = index_state.last_sync_result()
            {
                (
                    Cow::from(warp_i18n::t!("settings-code-status-codebase-too-large").to_string()),
                    theme.ui_warning_color(),
                    Icon::AlertTriangle,
                )
            } else if index_state.has_synced_version() {
                (
                    Cow::from(warp_i18n::t!("settings-code-status-stale").to_string()),
                    theme.nonactive_ui_detail().into_solid(),
                    Icon::ClockRefresh,
                )
            } else {
                (
                    Cow::from(warp_i18n::t!("settings-code-status-failed").to_string()),
                    theme.ui_error_color(),
                    Icon::AlertTriangle,
                )
            };

            return IndexingStatusPresentation {
                text,
                color,
                icon: Some(icon),
                refresh_action: Some(IndexingRefreshAction::Resync),
                show_delete: true,
            };
        }

        log::warn!("No index state for codebase");
        IndexingStatusPresentation {
            text: Cow::from(warp_i18n::t!("settings-code-status-no-index-built").to_string()),
            color: theme.nonactive_ui_text_color().into_solid(),
            icon: None,
            refresh_action: None,
            show_delete: true,
        }
    }

    #[cfg(not(target_family = "wasm"))]
    fn remote_indexing_status_presentation(
        &self,
        status: &RemoteCodebaseIndexStatus,
        appearance: &Appearance,
    ) -> IndexingStatusPresentation {
        let theme = appearance.theme();

        match status.state {
            RemoteCodebaseIndexState::NotEnabled => IndexingStatusPresentation {
                text: Cow::from(warp_i18n::t!("settings-code-no-index-created").to_string()),
                color: theme.disabled_ui_text_color().into_solid(),
                icon: Some(Icon::SlashCircle),
                refresh_action: Some(IndexingRefreshAction::RequestRemote),
                show_delete: true,
            },
            RemoteCodebaseIndexState::Unavailable => {
                let limit_reached = remote_codebase_index_limit_reached(status);
                IndexingStatusPresentation {
                    text: Cow::from(if limit_reached {
                        warp_i18n::t!("settings-code-status-remote-index-limit-reached").to_string()
                    } else {
                        warp_i18n::t!("settings-code-status-remote-unavailable").to_string()
                    }),
                    color: if limit_reached {
                        theme.ui_warning_color()
                    } else {
                        theme.disabled_ui_text_color().into_solid()
                    },
                    icon: Some(if limit_reached {
                        Icon::AlertTriangle
                    } else {
                        Icon::SlashCircle
                    }),
                    refresh_action: Some(IndexingRefreshAction::RequestRemote),
                    show_delete: true,
                }
            }
            RemoteCodebaseIndexState::Disabled => IndexingStatusPresentation {
                text: Cow::from(warp_i18n::t!("settings-code-status-remote-disabled").to_string()),
                color: theme.disabled_ui_text_color().into_solid(),
                icon: Some(Icon::SlashCircle),
                refresh_action: Some(IndexingRefreshAction::RequestRemote),
                show_delete: true,
            },
            RemoteCodebaseIndexState::Queued => IndexingStatusPresentation {
                text: Cow::from(warp_i18n::t!("settings-code-status-remote-queued").to_string()),
                color: theme.disabled_ui_text_color().into_solid(),
                icon: None,
                refresh_action: None,
                show_delete: true,
            },
            RemoteCodebaseIndexState::Indexing => {
                let text: Cow<'static, str> =
                    match (status.progress_completed, status.progress_total) {
                        (Some(completed), Some(total)) => Cow::from(
                            warp_i18n::t!(
                                "settings-code-status-remote-indexing-progress",
                                completed = completed,
                                total = total
                            )
                            .to_string(),
                        ),
                        (Some(completed), None) => Cow::from(
                            warp_i18n::t!(
                                "settings-code-status-remote-indexing-count",
                                completed = completed
                            )
                            .to_string(),
                        ),
                        (None, Some(total)) => Cow::from(
                            warp_i18n::t!(
                                "settings-code-status-remote-indexing-total",
                                total = total
                            )
                            .to_string(),
                        ),
                        (None, None) => {
                            Cow::from(warp_i18n::t!("settings-code-status-remote-indexing").to_string())
                        }
                    };

                IndexingStatusPresentation {
                    text,
                    color: theme.disabled_ui_text_color().into_solid(),
                    icon: None,
                    refresh_action: None,
                    show_delete: true,
                }
            }
            RemoteCodebaseIndexState::Ready => IndexingStatusPresentation {
                text: Cow::from(warp_i18n::t!("settings-code-status-synced").to_string()),
                color: theme.ansi_fg_green(),
                icon: Some(Icon::Check),
                refresh_action: Some(IndexingRefreshAction::Resync),
                show_delete: true,
            },
            RemoteCodebaseIndexState::Stale => IndexingStatusPresentation {
                text: Cow::from(warp_i18n::t!("settings-code-status-stale").to_string()),
                color: theme.nonactive_ui_detail().into_solid(),
                icon: Some(Icon::ClockRefresh),
                refresh_action: Some(IndexingRefreshAction::Resync),
                show_delete: true,
            },
            RemoteCodebaseIndexState::Failed => IndexingStatusPresentation {
                text: Cow::from(warp_i18n::t!("settings-code-status-failed").to_string()),
                color: theme.ui_error_color(),
                icon: Some(Icon::AlertTriangle),
                refresh_action: Some(IndexingRefreshAction::Resync),
                show_delete: true,
            },
        }
    }

    /// Returns (status_label, action_buttons) as separate elements for the indexing row.
    fn render_index_status_parts(
        &self,
        presentation: IndexingStatusPresentation,
        action_target: Option<LocalOrRemotePath>,
        manual_resync_mouse_state: MouseStateHandle,
        delete_mouse_state: MouseStateHandle,
        appearance: &Appearance,
    ) -> (Box<dyn Element>, Box<dyn Element>) {
        let theme = appearance.theme();
        let ui_builder = appearance.ui_builder();

        let mut label_row = Flex::row()
            .with_main_axis_size(MainAxisSize::Min)
            .with_cross_axis_alignment(CrossAxisAlignment::Center);
        if let Some(status_icon) = presentation.icon {
            label_row.add_child(
                Container::new(
                    ConstrainedBox::new(
                        status_icon
                            .to_warpui_icon(ThemeFill::Solid(presentation.color))
                            .finish(),
                    )
                    .with_width(STATUS_ICON_SIZE)
                    .with_height(STATUS_ICON_SIZE)
                    .finish(),
                )
                .with_margin_right(4.)
                .finish(),
            );
        }

        label_row.add_child(
            ui_builder
                .label(presentation.text)
                .with_style(UiComponentStyles {
                    font_color: Some(presentation.color),
                    font_size: Some(12.),
                    ..Default::default()
                })
                .build()
                .finish(),
        );

        let mut buttons_row = Flex::row()
            .with_main_axis_size(MainAxisSize::Min)
            .with_cross_axis_alignment(CrossAxisAlignment::Center)
            .with_spacing(4.);
        if let Some(refresh_action) = presentation.refresh_action
            && let Some(action_target) = action_target.clone()
        {
            buttons_row.add_child(
                icon_button(appearance, Icon::Refresh, false, manual_resync_mouse_state)
                    .with_active_styles(UiComponentStyles {
                        background: Some(theme.surface_1().into()),
                        ..Default::default()
                    })
                    .build()
                    .on_click(move |ctx, _, _| match (&action_target, &refresh_action) {
                        (
                            LocalOrRemotePath::Local(codebase_path),
                            IndexingRefreshAction::Resync,
                        ) => {
                            ctx.dispatch_typed_action(CodeSettingsPageAction::ManualResync(
                                codebase_path.clone(),
                            ));
                        }
                        (LocalOrRemotePath::Local(_), IndexingRefreshAction::RequestRemote) => {}
                        #[cfg(not(target_family = "wasm"))]
                        (LocalOrRemotePath::Remote(remote_path), IndexingRefreshAction::Resync) => {
                            ctx.dispatch_typed_action(CodeSettingsPageAction::ManualResyncRemote(
                                remote_path.clone(),
                            ));
                        }
                        #[cfg(not(target_family = "wasm"))]
                        (
                            LocalOrRemotePath::Remote(remote_path),
                            IndexingRefreshAction::RequestRemote,
                        ) => {
                            ctx.dispatch_typed_action(CodeSettingsPageAction::RequestRemoteIndex(
                                remote_path.clone(),
                            ));
                        }
                        #[cfg(target_family = "wasm")]
                        (LocalOrRemotePath::Remote(_), _) => {}
                    })
                    .finish(),
            );
        }

        if presentation.show_delete
            && let Some(action_target) = action_target
        {
            buttons_row.add_child(
                icon_button(appearance, Icon::Trash, false, delete_mouse_state)
                    .with_active_styles(UiComponentStyles {
                        background: Some(theme.surface_1().into()),
                        ..Default::default()
                    })
                    .build()
                    .on_click(move |ctx, _, _| match &action_target {
                        LocalOrRemotePath::Local(codebase_path) => {
                            ctx.dispatch_typed_action(CodeSettingsPageAction::DeleteIndex(
                                codebase_path.clone(),
                            ));
                        }
                        #[cfg(not(target_family = "wasm"))]
                        LocalOrRemotePath::Remote(remote_path) => {
                            ctx.dispatch_typed_action(CodeSettingsPageAction::DeleteRemoteIndex(
                                remote_path.clone(),
                            ));
                        }
                        #[cfg(target_family = "wasm")]
                        LocalOrRemotePath::Remote(_) => {}
                    })
                    .finish(),
            );
        }

        (label_row.finish(), buttons_row.finish())
    }

    /// Renders the LSP servers subsection within a workspace row.
    #[allow(clippy::too_many_arguments)]
    fn render_lsp_servers_subsection(
        &self,
        workspace_path: &Path,
        all_servers: &[(LSPServerType, EnablementState)],
        lsp_manager: &LspManagerModel,
        lsp_mouse_states: Vec<LspServerRowMouseStates>,
        suggested_server_statuses: &HashMap<(PathBuf, LSPServerType), LspRepoStatus>,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let ui_builder = appearance.ui_builder();
        let theme = appearance.theme();

        let mut content = Flex::column().with_spacing(SUB_SECTION_MARGIN);

        // "LSP SERVERS" label
        content.add_child(
            ui_builder
                .span(warp_i18n::t!("settings-code-lsp-servers-section"))
                .with_style(UiComponentStyles {
                    font_size: Some(11.0),
                    font_weight: Some(Weight::Semibold),
                    font_color: Some(theme.disabled_ui_text_color().into()),
                    ..Default::default()
                })
                .build()
                .finish(),
        );

        // Get the actual server models for this workspace
        let server_models = lsp_manager.servers_for_workspace(workspace_path);

        for (idx, (server_type, enablement_state)) in all_servers.iter().enumerate() {
            let mouse_states = lsp_mouse_states.get(idx).cloned().unwrap_or_default();

            if *enablement_state == EnablementState::Suggested {
                // Render the "available for download" suggested server row.
                let repo_status = suggested_server_statuses
                    .get(&(workspace_path.to_path_buf(), *server_type))
                    .cloned();
                content.add_child(self.render_suggested_lsp_server_row(
                    workspace_path,
                    *server_type,
                    repo_status,
                    mouse_states,
                    appearance,
                ));
            } else {
                let is_enabled = *enablement_state == EnablementState::Yes;

                // Find the corresponding server model (only exists if enabled and running)
                let server_model = server_models.and_then(|servers| {
                    servers
                        .iter()
                        .find(|s| s.as_ref(app).server_type() == *server_type)
                });

                content.add_child(self.render_lsp_server_row(
                    workspace_path,
                    *server_type,
                    server_model,
                    is_enabled,
                    mouse_states,
                    appearance,
                    app,
                ));
            }
        }

        content.finish()
    }

    /// Renders a suggested LSP server row with "+" install/enable button.
    fn render_suggested_lsp_server_row(
        &self,
        workspace_path: &Path,
        server_type: LSPServerType,
        repo_status: Option<LspRepoStatus>,
        mouse_states: LspServerRowMouseStates,
        appearance: &Appearance,
    ) -> Box<dyn Element> {
        let theme = appearance.theme();
        let ui_builder = appearance.ui_builder();

        let mut row = Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_main_axis_alignment(MainAxisAlignment::SpaceBetween)
            .with_cross_axis_alignment(CrossAxisAlignment::Center);

        // Left side: language initial badge + name/description column
        let mut left_content = Flex::row().with_cross_axis_alignment(CrossAxisAlignment::Center);

        // Language initial badge (no status dot for suggested servers)
        let badge_size = 36.0;
        let avatar = Avatar::new(
            AvatarContent::DisplayName(server_type.binary_name().to_string()),
            UiComponentStyles {
                width: Some(badge_size),
                height: Some(badge_size),
                border_radius: Some(CornerRadius::with_all(Radius::Percentage(50.))),
                font_family_id: Some(appearance.ui_font_family()),
                font_weight: Some(Weight::Bold),
                background: Some(theme.surface_3().into()),
                font_size: Some(16.),
                font_color: Some(theme.active_ui_text_color().into()),
                ..Default::default()
            },
        );

        left_content.add_child(
            Container::new(avatar.build().finish())
                .with_margin_right(8.)
                .finish(),
        );

        // Name + description
        let mut name_desc_column = Flex::column().with_spacing(4.);

        name_desc_column.add_child(
            ui_builder
                .span(server_type.binary_name())
                .with_style(UiComponentStyles {
                    font_size: Some(12.0),
                    font_color: Some(theme.active_ui_text_color().into()),
                    ..Default::default()
                })
                .build()
                .finish(),
        );

        let (description, is_installing): (Cow<'static, str>, bool) = match &repo_status {
            Some(LspRepoStatus::DisabledAndInstalled { .. }) => (
                Cow::from(warp_i18n::t!("settings-code-lsp-installed").to_string()),
                false,
            ),
            Some(LspRepoStatus::Installing { .. }) => (
                Cow::from(warp_i18n::t!("settings-code-lsp-installing").to_string()),
                true,
            ),
            Some(LspRepoStatus::CheckingForInstallation) => (
                Cow::from(warp_i18n::t!("settings-code-lsp-checking").to_string()),
                true,
            ),
            _ => (
                Cow::from(warp_i18n::t!("settings-code-lsp-available-for-download").to_string()),
                false,
            ),
        };

        name_desc_column.add_child(
            ui_builder
                .label(description)
                .with_style(UiComponentStyles {
                    font_color: Some(theme.disabled_ui_text_color().into()),
                    font_size: Some(12.),
                    ..Default::default()
                })
                .build()
                .finish(),
        );

        left_content.add_child(name_desc_column.finish());
        row.add_child(left_content.finish());

        // Right side: "+" button to install/enable
        if !is_installing {
            let workspace_path_clone = workspace_path.to_path_buf();
            let needs_install = matches!(
                &repo_status,
                None | Some(LspRepoStatus::DisabledAndNotInstalled { .. })
            );
            let install_button = icon_button(appearance, Icon::Plus, false, mouse_states.install)
                .with_style(UiComponentStyles {
                    border_width: Some(1.),
                    border_color: Some(theme.surface_3().into()),
                    ..Default::default()
                })
                .build()
                .with_cursor(Cursor::PointingHand)
                .on_click(move |ctx, _, _| {
                    if needs_install {
                        ctx.dispatch_typed_action(
                            CodeSettingsPageAction::InstallAndEnableLspServer {
                                workspace_path: workspace_path_clone.clone(),
                                server_type,
                            },
                        );
                    } else {
                        ctx.dispatch_typed_action(
                            CodeSettingsPageAction::EnableSuggestedLspServer {
                                workspace_path: workspace_path_clone.clone(),
                                server_type,
                            },
                        );
                    }
                })
                .finish();

            row.add_child(install_button);
        }

        Container::new(row.finish())
            .with_uniform_padding(12.)
            .with_background(theme.surface_2())
            .with_corner_radius(CornerRadius::with_all(Radius::Pixels(4.)))
            .finish()
    }

    /// Renders a single LSP server row with language initial icon, status, and toggle.
    #[allow(clippy::too_many_arguments)]
    fn render_lsp_server_row(
        &self,
        workspace_path: &Path,
        server_type: LSPServerType,
        server_model: Option<&warpui::ModelHandle<LspServerModel>>,
        is_enabled: bool,
        mouse_states: LspServerRowMouseStates,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let theme = appearance.theme();
        let ui_builder = appearance.ui_builder();

        let mut row = Flex::row()
            .with_main_axis_size(MainAxisSize::Max)
            .with_main_axis_alignment(MainAxisAlignment::SpaceBetween)
            .with_cross_axis_alignment(CrossAxisAlignment::Center);

        // Left side: language initial badge + name/status column
        let mut left_content = Flex::row().with_cross_axis_alignment(CrossAxisAlignment::Center);

        // Language initial badge with status dot overlay (using Avatar component)
        let (status_color, status_text) = self.get_lsp_status_info(server_model, app, theme);
        let is_failed = server_model
            .is_some_and(|model| matches!(model.as_ref(app).state(), LspState::Failed { .. }));

        // Language initial badge with status dot overlay (using Avatar component)
        let badge_size = 36.0;
        let mut avatar = Avatar::new(
            AvatarContent::DisplayName(server_type.binary_name().to_string()),
            UiComponentStyles {
                width: Some(badge_size),
                height: Some(badge_size),
                border_radius: Some(CornerRadius::with_all(Radius::Percentage(50.))),
                font_family_id: Some(appearance.ui_font_family()),
                font_weight: Some(Weight::Bold),
                background: Some(theme.surface_3().into()),
                font_size: Some(16.),
                font_color: Some(theme.active_ui_text_color().into()),
                ..Default::default()
            },
        );

        avatar = avatar.with_status_element_with_offset(
            StatusElementTypes::Circle,
            UiComponentStyles {
                width: Some(LSP_STATUS_INDICATOR_SIZE),
                height: Some(LSP_STATUS_INDICATOR_SIZE),
                border_radius: Some(CornerRadius::with_all(Radius::Percentage(50.))),
                background: Some(Fill::Solid(status_color)),
                ..Default::default()
            },
            -5.,
            5.,
        );

        left_content.add_child(
            Container::new(avatar.build().finish())
                .with_margin_right(8.)
                .finish(),
        );

        // Name + status on separate lines
        let mut name_status_column = Flex::column().with_spacing(4.);

        // Server name
        name_status_column.add_child(
            ui_builder
                .span(server_type.binary_name())
                .with_style(UiComponentStyles {
                    font_size: Some(12.0),
                    font_color: Some(theme.active_ui_text_color().into()),
                    ..Default::default()
                })
                .build()
                .finish(),
        );

        // Status text
        let status_text_color = if is_failed {
            Some(status_color)
        } else {
            Some(theme.disabled_ui_text_color().into())
        };

        name_status_column.add_child(
            ui_builder
                .label(status_text)
                .with_style(UiComponentStyles {
                    font_color: status_text_color,
                    font_size: Some(12.),
                    ..Default::default()
                })
                .build()
                .finish(),
        );

        left_content.add_child(name_status_column.finish());
        row.add_child(left_content.finish());

        // Right side: restart/logs buttons (if failed) + toggle switch (always)
        let mut right_content = Flex::row()
            .with_spacing(8.)
            .with_cross_axis_alignment(CrossAxisAlignment::Center);

        if is_failed && let Some(server_handle) = server_model.cloned() {
            let server_for_action = server_handle.clone();
            let restart_button = ui_builder
                .button(ButtonVariant::Secondary, mouse_states.restart)
                .with_style(UiComponentStyles {
                    font_size: Some(12.),
                    ..Default::default()
                })
                .with_hovered_styles(UiComponentStyles {
                    background: Some(theme.surface_3().into()),
                    ..Default::default()
                })
                .with_text_label(warp_i18n::t!("settings-code-lsp-restart-server").to_string())
                .build()
                .with_cursor(Cursor::PointingHand)
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(CodeSettingsPageAction::RestartLspServer {
                        server: server_for_action.clone(),
                    });
                })
                .finish();

            right_content.add_child(restart_button);
        }

        // Show "View logs" when the server has been started (Available, Starting/Busy, or Failed)
        #[cfg(not(target_family = "wasm"))]
        {
            let has_logs = server_model.is_some_and(|model| {
                matches!(
                    model.as_ref(app).state(),
                    LspState::Available { .. } | LspState::Starting | LspState::Failed { .. }
                )
            });
            if has_logs {
                let log_path = crate::code::lsp_logs::log_file_path(server_type, workspace_path);
                let view_logs_button = ui_builder
                    .button(ButtonVariant::Accent, mouse_states.view_logs)
                    .with_style(UiComponentStyles {
                        font_size: Some(12.),
                        ..Default::default()
                    })
                    .with_text_label(warp_i18n::t!("settings-code-lsp-view-logs").to_string())
                    .build()
                    .with_cursor(Cursor::PointingHand)
                    .on_click(move |ctx, _, _| {
                        ctx.dispatch_typed_action(CodeSettingsPageAction::OpenLspLogs {
                            log_path: log_path.clone(),
                        });
                    })
                    .finish();

                right_content.add_child(view_logs_button);
            }
        }

        // Toggle switch (always shown)
        let workspace_path_clone = workspace_path.to_path_buf();
        let server_type_clone = server_type;
        right_content.add_child(
            ui_builder
                .switch(mouse_states.toggle)
                .check(is_enabled)
                .build()
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(CodeSettingsPageAction::ToggleLspServer {
                        workspace_path: workspace_path_clone.clone(),
                        server_type: server_type_clone,
                        currently_enabled: is_enabled,
                    });
                })
                .finish(),
        );

        row.add_child(right_content.finish());

        Container::new(row.finish())
            .with_uniform_padding(12.)
            .with_background(theme.surface_2())
            .with_corner_radius(CornerRadius::with_all(Radius::Pixels(4.)))
            .finish()
    }

    /// Gets the status color and text for an LSP server.
    fn get_lsp_status_info(
        &self,
        server_model: Option<&warpui::ModelHandle<LspServerModel>>,
        app: &AppContext,
        theme: &warp_core::ui::theme::WarpTheme,
    ) -> (ColorU, Cow<'static, str>) {
        match server_model {
            Some(model) => {
                let server = model.as_ref(app);
                match server.state() {
                    LspState::Available { .. } if !server.has_pending_tasks() => (
                        AnsiColorIdentifier::Green
                            .to_ansi_color(&theme.terminal_colors().normal)
                            .into(),
                        Cow::from(warp_i18n::t!("settings-code-lsp-state-available").to_string()),
                    ),
                    LspState::Starting | LspState::Available { .. } => (
                        AnsiColorIdentifier::Yellow
                            .to_ansi_color(&theme.terminal_colors().normal)
                            .into(),
                        Cow::from(warp_i18n::t!("settings-code-lsp-state-busy").to_string()),
                    ),
                    LspState::Failed { .. } => (
                        AnsiColorIdentifier::Red
                            .to_ansi_color(&theme.terminal_colors().normal)
                            .into(),
                        Cow::from(warp_i18n::t!("settings-code-lsp-state-failed").to_string()),
                    ),
                    LspState::Stopped { .. } | LspState::Stopping { .. } => (
                        theme.disabled_ui_text_color().into_solid(),
                        Cow::from(warp_i18n::t!("settings-code-lsp-state-stopped").to_string()),
                    ),
                }
            }
            None => (
                theme.disabled_ui_text_color().into_solid(),
                Cow::from(warp_i18n::t!("settings-code-lsp-state-not-running").to_string()),
            ),
        }
    }
}

struct CodebaseIndexingCategorizedWidget {
    inner: CodePageWidget,
}

impl SettingsWidget for CodebaseIndexingCategorizedWidget {
    type View = CodeSettingsPageView;

    fn search_terms(&self) -> &str {
        "codebase index indexing repository code context embedding auto-index lsp language server"
    }

    fn render(
        &self,
        view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let ui_builder = appearance.ui_builder();
        let global_ai_enabled = AISettings::as_ref(app).is_any_ai_enabled(app);
        let is_auggie_backend =
            crate::ai::codebase_index_backend::is_local_codebase_index_backend(app);
        let is_auggie_unavailable =
            crate::ai::codebase_index_backend::is_auggie_backend_unavailable(app);
        let codebase_context_enabled =
            crate::ai::codebase_index_backend::is_codebase_context_enabled_for_indexing(app);

        let mut content = Flex::column();

        // Codebase indexing toggle using render_body_item for consistent styling
        let admin_setting = UserWorkspaces::as_ref(app).team_allows_codebase_context();
        let switch = ui_builder
            .switch(self.inner.switch_state.clone())
            .check(codebase_context_enabled);

        let disabled_tooltip_text = codebase_indexing_disabled_tooltip(
            admin_setting,
            global_ai_enabled,
            is_auggie_backend,
            is_auggie_unavailable,
        );

        let toggle_element = if let Some(tooltip_text) = disabled_tooltip_text {
            switch
                .with_tooltip(TooltipConfig {
                    text: tooltip_text,
                    styles: ui_builder.default_tool_tip_styles(),
                })
                .disable()
                .build()
                .finish()
        } else {
            switch
                .build()
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(CodeSettingsPageAction::ToggleCodebaseContext);
                })
                .finish()
        };

        content.add_child(render_body_item::<CodeSettingsPageAction>(
            warp_i18n::t!("settings-code-codebase-indexing-label").to_string(),
            None,
            LocalOnlyIconState::Hidden,
            ToggleState::Enabled,
            appearance,
            toggle_element,
            Some(warp_i18n::t!("settings-code-codebase-index-description").to_string()),
        ));

        // Auto-indexing toggle (only shown when codebase indexing is enabled).
        // `codebase_context_enabled` is now backend-aware, so the bare global-AI
        // gate is redundant.
        if codebase_context_enabled {
            let auto_indexing_enabled = *CodeSettings::as_ref(app).auto_indexing_enabled;

            content.add_child(render_body_item::<CodeSettingsPageAction>(
                warp_i18n::t!("settings-code-auto-index-feature-name").to_string(),
                None,
                LocalOnlyIconState::Hidden,
                ToggleState::Enabled,
                appearance,
                ui_builder
                    .switch(self.inner.auto_index_switch_state.clone())
                    .check(auto_indexing_enabled)
                    .build()
                    .on_click(move |ctx, _, _| {
                        ctx.dispatch_typed_action(CodeSettingsPageAction::ToggleAutoIndexing);
                    })
                    .finish(),
                Some(warp_i18n::t!("settings-code-auto-index-description").to_string()),
            ));

            if !CodebaseIndexManager::as_ref(app).can_create_new_indices() {
                content.add_child(
                    ui_builder
                        .paragraph(warp_i18n::t!("settings-code-index-limit-reached"))
                        .with_style(UiComponentStyles {
                            font_color: Some(appearance.theme().disabled_ui_text_color().into()),
                            ..Default::default()
                        })
                        .build()
                        .with_margin_bottom(8.0)
                        .finish(),
                );
            }
        }

        // Initialized / indexed folders section
        let mouse_states = InitializedFoldersMouseStates {
            codebase_manual_resync: view.codebase_manual_resync_mouse_states.clone(),
            codebase_delete: view.codebase_delete_mouse_states.clone(),
            #[cfg(not(target_family = "wasm"))]
            remote_codebase_manual_resync: view.remote_codebase_manual_resync_mouse_states.clone(),
            #[cfg(not(target_family = "wasm"))]
            remote_codebase_delete: view.remote_codebase_delete_mouse_states.clone(),
            lsp_rows: view.lsp_row_mouse_states.clone(),
            open_project_rules: view.open_project_rules_mouse_states.clone(),
        };
        content.add_child(self.inner.render_initialized_folders(
            mouse_states,
            &view.suggested_server_statuses,
            appearance,
            app,
        ));

        content.finish()
    }
}

#[cfg(feature = "local_fs")]
struct ExternalEditorCodeWidget;

#[cfg(feature = "local_fs")]
impl SettingsWidget for ExternalEditorCodeWidget {
    type View = CodeSettingsPageView;

    fn search_terms(&self) -> &str {
        "code editor open files markdown AI conversations layout pane tab"
    }

    fn render(
        &self,
        view: &Self::View,
        _appearance: &Appearance,
        _app: &AppContext,
    ) -> Box<dyn Element> {
        if let Some(editor_view) = &view.external_editor_view {
            ChildView::new(editor_view).finish()
        } else {
            Empty::new().finish()
        }
    }
}

#[derive(Default)]
struct AutoOpenCodeReviewPaneCodeWidget {
    switch_state: SwitchStateHandle,
}

impl SettingsWidget for AutoOpenCodeReviewPaneCodeWidget {
    type View = CodeSettingsPageView;

    fn search_terms(&self) -> &str {
        "oz auto open code review pane panel agent mode change first time accepted diff view conversation"
    }

    fn render(
        &self,
        _view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let general_settings = GeneralSettings::as_ref(app);
        render_body_item::<CodeSettingsPageAction>(
            warp_i18n::t!("settings-code-auto-open-review-pane").to_string(),
            None,
            LocalOnlyIconState::Hidden,
            ToggleState::Enabled,
            appearance,
            appearance
                .ui_builder()
                .switch(self.switch_state.clone())
                .check(*general_settings.auto_open_code_review_pane_on_first_agent_change)
                .build()
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(CodeSettingsPageAction::ToggleAutoOpenCodeReviewPane);
                })
                .finish(),
            Some(warp_i18n::t!("settings-code-auto-open-review-pane-desc").to_string()),
        )
    }
}

impl SettingsPageMeta for CodeSettingsPageView {
    fn section() -> SettingsSection {
        SettingsSection::Code
    }

    fn update_filter(&mut self, query: &str, ctx: &mut ViewContext<Self>) -> MatchData {
        self.page.update_filter(query, ctx)
    }

    fn should_render(&self, ctx: &AppContext) -> bool {
        crate::ai::codebase_index_backend::is_codebase_index_feature_available(ctx)
            || FeatureFlag::OpenWarpNewSettingsModes.is_enabled()
    }

    fn on_page_selected(&mut self, _: bool, ctx: &mut ViewContext<Self>) {
        // We want to immediately see if the user is part of a workspace rather than wait for the next poll.
        std::mem::drop(
            TeamUpdateManager::handle(ctx)
                .update(ctx, |manager, ctx| manager.refresh_workspace_metadata(ctx)),
        );
    }

    fn scroll_to_widget(&mut self, widget_id: &'static str) {
        self.page.scroll_to_widget(widget_id)
    }

    fn clear_highlighted_widget(&mut self) {
        self.page.clear_highlighted_widget();
    }
}

impl From<ViewHandle<CodeSettingsPageView>> for SettingsPageViewHandle {
    fn from(view_handle: ViewHandle<CodeSettingsPageView>) -> Self {
        SettingsPageViewHandle::Code(view_handle)
    }
}

#[derive(Default)]
struct CodeReviewPanelToggleWidget {
    switch_state: SwitchStateHandle,
}

impl SettingsWidget for CodeReviewPanelToggleWidget {
    type View = CodeSettingsPageView;

    fn search_terms(&self) -> &str {
        "code review panel right side diff git"
    }

    fn render(
        &self,
        _view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let tab_settings = TabSettings::as_ref(app);

        render_body_item::<CodeSettingsPageAction>(
            warp_i18n::t!("settings-code-show-review-button").to_string(),
            None,
            LocalOnlyIconState::Hidden,
            ToggleState::Enabled,
            appearance,
            appearance
                .ui_builder()
                .switch(self.switch_state.clone())
                .check(*tab_settings.show_code_review_button)
                .build()
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(CodeSettingsPageAction::ToggleCodeReviewPanel);
                })
                .finish(),
            Some(warp_i18n::t!("settings-code-show-review-button-desc").to_string()),
        )
    }
}

#[derive(Default)]
struct CodeReviewDiffStatsToggleWidget {
    switch_state: SwitchStateHandle,
}

impl SettingsWidget for CodeReviewDiffStatsToggleWidget {
    type View = CodeSettingsPageView;

    fn search_terms(&self) -> &str {
        "code review diff stats lines added removed counts"
    }

    fn render(
        &self,
        _view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let tab_settings = TabSettings::as_ref(app);

        render_body_item::<CodeSettingsPageAction>(
            warp_i18n::t!("settings-code-show-diff-stats").to_string(),
            None,
            LocalOnlyIconState::Hidden,
            ToggleState::Enabled,
            appearance,
            appearance
                .ui_builder()
                .switch(self.switch_state.clone())
                .check(*tab_settings.show_code_review_diff_stats)
                .build()
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(
                        CodeSettingsPageAction::ToggleShowCodeReviewDiffStats,
                    );
                })
                .finish(),
            Some(warp_i18n::t!("settings-code-show-diff-stats-desc").to_string()),
        )
    }
}

#[derive(Default)]
struct ProjectExplorerToggleWidget {
    switch_state: SwitchStateHandle,
}

impl SettingsWidget for ProjectExplorerToggleWidget {
    type View = CodeSettingsPageView;

    fn search_terms(&self) -> &str {
        "project explorer file tree left panel tools"
    }

    fn render(
        &self,
        _view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let code_settings = CodeSettings::as_ref(app);

        render_body_item::<CodeSettingsPageAction>(
            warp_i18n::t!("settings-code-project-explorer").to_string(),
            None,
            LocalOnlyIconState::Hidden,
            ToggleState::Enabled,
            appearance,
            appearance
                .ui_builder()
                .switch(self.switch_state.clone())
                .check(*code_settings.show_project_explorer)
                .build()
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(CodeSettingsPageAction::ToggleProjectExplorer);
                })
                .finish(),
            Some(warp_i18n::t!("settings-code-project-explorer-desc").to_string()),
        )
    }
}

#[derive(Default)]
struct GlobalSearchToggleWidget {
    switch_state: SwitchStateHandle,
}

impl SettingsWidget for GlobalSearchToggleWidget {
    type View = CodeSettingsPageView;

    fn search_terms(&self) -> &str {
        "global search file search left panel tools"
    }

    fn render(
        &self,
        _view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let code_settings = CodeSettings::as_ref(app);

        render_body_item::<CodeSettingsPageAction>(
            warp_i18n::t!("settings-code-global-search").to_string(),
            None,
            LocalOnlyIconState::Hidden,
            ToggleState::Enabled,
            appearance,
            appearance
                .ui_builder()
                .switch(self.switch_state.clone())
                .check(*code_settings.show_global_search)
                .build()
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(CodeSettingsPageAction::ToggleGlobalSearch);
                })
                .finish(),
            Some(warp_i18n::t!("settings-code-global-search-desc").to_string()),
        )
    }
}

#[derive(Default)]
struct ShowHiddenFilesToggleWidget {
    switch_state: SwitchStateHandle,
}

impl SettingsWidget for ShowHiddenFilesToggleWidget {
    type View = CodeSettingsPageView;

    fn search_terms(&self) -> &str {
        "show hidden files dotfiles project explorer file tree"
    }

    fn render(
        &self,
        _view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let code_settings = CodeSettings::as_ref(app);

        render_body_item::<CodeSettingsPageAction>(
            "Show hidden files in project explorer".into(),
            None,
            LocalOnlyIconState::Hidden,
            ToggleState::Enabled,
            appearance,
            appearance
                .ui_builder()
                .switch(self.switch_state.clone())
                .check(*code_settings.show_hidden_files)
                .build()
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(CodeSettingsPageAction::ToggleShowHiddenFiles);
                })
                .finish(),
            Some(
                "Show dotfiles and hidden files (starting with .) in the project explorer.".into(),
            ),
        )
    }
}

#[derive(Default)]
struct FormatOnSaveToggleWidget {
    switch_state: SwitchStateHandle,
}

impl SettingsWidget for FormatOnSaveToggleWidget {
    type View = CodeSettingsPageView;

    fn search_terms(&self) -> &str {
        "format on save lsp language server formatting reformat editor"
    }

    fn render(
        &self,
        _view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let code_settings = CodeSettings::as_ref(app);

        render_body_item::<CodeSettingsPageAction>(
            "Format on save (requires an active language server)".into(),
            None,
            LocalOnlyIconState::Hidden,
            ToggleState::Enabled,
            appearance,
            appearance
                .ui_builder()
                .switch(self.switch_state.clone())
                .check(*code_settings.format_on_save)
                .build()
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(CodeSettingsPageAction::ToggleFormatOnSave);
                })
                .finish(),
            Some(
                "Only applies when a language server is active for the file. Automatically formats the file with the language server on save; other LSP features (hover, go-to-definition, references, diagnostics) are unaffected."
                    .into(),
            ),
        )
    }
}

#[derive(Default)]
struct AutoSaveToggleWidget {
    switch_state: SwitchStateHandle,
}

impl SettingsWidget for AutoSaveToggleWidget {
    type View = CodeSettingsPageView;

    fn search_terms(&self) -> &str {
        "auto save autosave automatically save editor files on type focus"
    }

    fn render(
        &self,
        _view: &Self::View,
        appearance: &Appearance,
        app: &AppContext,
    ) -> Box<dyn Element> {
        let code_settings = CodeSettings::as_ref(app);

        render_body_item::<CodeSettingsPageAction>(
            "Auto save".into(),
            None,
            LocalOnlyIconState::Hidden,
            ToggleState::Enabled,
            appearance,
            appearance
                .ui_builder()
                .switch(self.switch_state.clone())
                .check(*code_settings.auto_save)
                .build()
                .on_click(move |ctx, _, _| {
                    ctx.dispatch_typed_action(CodeSettingsPageAction::ToggleAutoSave);
                })
                .finish(),
            Some(
                "Automatically saves changes in the Warp text editor as you type and when the editor loses focus."
                    .into(),
            ),
        )
    }
}

#[cfg(test)]
mod i18n_tests {
    use super::*;
    use chrono::Duration as ChronoDuration;
    use serial_test::serial;

    fn ensure_en() {
        let _ = warp_i18n::init(warp_i18n::Locale::En);
        warp_i18n::set_locale(warp_i18n::Locale::En);
    }

    fn workspace_with(
        synced_at: Option<DateTime<Utc>>,
        queried_ts: Option<DateTime<Utc>>,
        query_count: u32,
    ) -> WorkspaceMetadata {
        WorkspaceMetadata {
            path: PathBuf::from("/tmp/test-repo"),
            file_count: Some(123),
            fragment_count: Some(456),
            index_bytes: Some(2 * 1024),
            synced_at,
            queried_ts,
            query_count,
            ..Default::default()
        }
    }

    #[test]
    fn truncate_error_reason_passes_through_short_text() {
        assert_eq!(truncate_error_reason("oops"), "oops");
    }

    #[test]
    fn truncate_error_reason_takes_first_line_only() {
        assert_eq!(
            truncate_error_reason("first failure\nsecond line\nthird"),
            "first failure"
        );
    }

    #[test]
    fn truncate_error_reason_caps_long_messages_with_ellipsis() {
        let long = "a".repeat(ERROR_REASON_MAX_LEN + 50);
        let out = truncate_error_reason(&long);
        assert!(out.ends_with('…'));
        assert_eq!(out.chars().count(), ERROR_REASON_MAX_LEN + 1);
    }

    #[test]
    fn truncate_error_reason_handles_empty_and_blank() {
        assert_eq!(truncate_error_reason(""), "(no error message)");
        assert_eq!(truncate_error_reason("   \n  "), "(no error message)");
    }

    #[test]
    #[serial]
    fn humanize_bytes_label_picks_correct_unit() {
        ensure_en();
        assert!(humanize_bytes_label(0).contains("0 B"));
        assert!(humanize_bytes_label(512).contains("512 B"));
        assert!(humanize_bytes_label(1024).contains("1.0 KB"));
        assert!(humanize_bytes_label(1024 * 1024).contains("1.0 MB"));
        assert!(humanize_bytes_label(1024u64 * 1024 * 1024).contains("1.0 GB"));
    }

    #[test]
    #[serial]
    fn relative_time_label_walks_through_buckets() {
        ensure_en();
        let now = Utc::now();
        assert!(relative_time_label(now - ChronoDuration::seconds(30)).contains("just now"));
        assert!(relative_time_label(now - ChronoDuration::minutes(5)).contains("5"));
        assert!(relative_time_label(now - ChronoDuration::hours(2)).contains("2"));
        assert!(relative_time_label(now - ChronoDuration::days(3)).contains("3"));
    }

    #[test]
    #[serial]
    fn build_index_stats_line_returns_none_when_never_synced() {
        ensure_en();
        let ws = WorkspaceMetadata {
            path: PathBuf::from("/tmp/empty"),
            ..Default::default()
        };
        assert_eq!(build_index_stats_line(&ws, None), None);
    }

    #[test]
    #[serial]
    fn build_index_stats_line_includes_all_present_fields() {
        ensure_en();
        let ws = workspace_with(Some(Utc::now()), Some(Utc::now()), 3);
        let line = build_index_stats_line(&ws, Some(&CodebaseIndexFinishedStatus::Completed))
            .expect("expected stats line");
        assert!(line.contains("123"), "missing file count: {line}");
        assert!(line.contains("456"), "missing fragment count: {line}");
        assert!(line.contains("KB"), "missing bytes label: {line}");
        assert!(line.contains("synced"), "missing synced label: {line}");
        assert!(line.contains("queried"), "missing queried label: {line}");
        assert!(line.contains(" · "), "missing separator: {line}");
    }

    #[test]
    #[serial]
    fn build_index_stats_line_skips_query_section_when_count_zero() {
        ensure_en();
        let ws = workspace_with(Some(Utc::now()), None, 0);
        let line = build_index_stats_line(&ws, Some(&CodebaseIndexFinishedStatus::Completed))
            .expect("stats line");
        assert!(!line.contains("queried"), "should not show queried: {line}");
    }

    #[test]
    #[serial]
    fn build_index_stats_line_replaces_with_error_when_failed() {
        ensure_en();
        let ws = workspace_with(Some(Utc::now()), None, 0);
        let err = CodebaseIndexFinishedStatus::Failed(CodebaseIndexingError::ExceededMaxFileLimit);
        let line = build_index_stats_line(&ws, Some(&err)).expect("error line");
        // Must NOT include the stats fields when failed.
        assert!(
            !line.contains("123"),
            "stats leaked into error line: {line}"
        );
        assert!(
            !line.contains(" · "),
            "separator leaked into error line: {line}"
        );
    }

    #[test]
    #[serial]
    fn error_reason_text_covers_all_variants() {
        ensure_en();
        assert!(!error_reason_text(&CodebaseIndexingError::BuildTreeError).is_empty());
        assert!(!error_reason_text(&CodebaseIndexingError::ExceededMaxFileLimit).is_empty());
        assert!(!error_reason_text(&CodebaseIndexingError::MaxDepthExceeded).is_empty());
        assert!(
            !error_reason_text(&CodebaseIndexingError::FailedToGenerateEmbeddings(vec![]))
                .is_empty()
        );
        assert!(
            !error_reason_text(&CodebaseIndexingError::FailedToSyncIntermediateNodes(
                vec![]
            ))
            .is_empty()
        );
        let other_msg =
            error_reason_text(&CodebaseIndexingError::Other(anyhow::anyhow!("disk full")));
        assert!(other_msg.contains("disk full"), "other reason: {other_msg}");
    }
}
