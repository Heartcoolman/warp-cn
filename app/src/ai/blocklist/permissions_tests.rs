use std::path::PathBuf;

use uuid::Uuid;
use warp_core::execution_mode::ExecutionMode;
use warp_util::path::EscapeChar;
use warpui::{App, EntityId, ModelHandle};

use super::{BlocklistAIHistoryModel, BlocklistAIPermissions};
use crate::ai::active_agent_views_model::ActiveAgentViewsModel;
use crate::ai::agent::conversation::AIConversationId;
use crate::ai::blocklist::permissions::{
    CommandExecutionPermission, CommandExecutionPermissionDeniedReason, FileReadPermission,
    FileReadPermissionAllowedReason, FileReadPermissionDeniedReason, FileWritePermission,
    FileWritePermissionAllowedReason, FileWritePermissionDeniedReason,
};
use crate::ai::blocklist::CommandExecutionPermissionAllowedReason;
use crate::ai::execution_profiles::profiles::AIExecutionProfilesModel;
use crate::ai::execution_profiles::{ActionPermission, WriteToPtyPermission};
use crate::ai::mcp::templatable_manager::TemplatableMCPServerManager;
use crate::auth::AuthStateProvider;
use crate::cloud_object::model::persistence::CloudModel;
use crate::network::NetworkStatus;
use crate::server::cloud_objects::update_manager::UpdateManager;
use crate::server::sync_queue::SyncQueue;
use crate::settings::{AgentModeCommandExecutionPredicate, PrivacySettings};
use crate::terminal::cli_agent_sessions::CLIAgentSessionsModel;
use crate::test_util::settings::initialize_settings_for_tests_with_mode;
use crate::workspaces::team_tester::TeamTesterStatus;
use crate::workspaces::user_workspaces::UserWorkspaces;
use crate::workspaces::workspace::SandboxedAgentSettings;
use crate::{
    AgentNotificationsModel, GlobalResourceHandles, GlobalResourceHandlesProvider, LaunchMode,
};

struct PermissionsTestState {
    convo_id: AIConversationId,
    permissions: ModelHandle<BlocklistAIPermissions>,
    history: ModelHandle<BlocklistAIHistoryModel>,
    terminal_view_id: EntityId,
    user_workspaces: ModelHandle<UserWorkspaces>,
    profile_model: ModelHandle<AIExecutionProfilesModel>,
}

fn initialize_permissions_test(app: &mut App) -> PermissionsTestState {
    initialize_permissions_test_with_mode(app, ExecutionMode::App, false)
}

fn initialize_permissions_test_sandboxed(app: &mut App) -> PermissionsTestState {
    let state = initialize_permissions_test_with_mode(app, ExecutionMode::Sdk, true);
    state.profile_model.update(app, |model, ctx| {
        let profile_id = *model.default_profile(ctx).id();
        model.apply_cli_profile_defaults_for_test(profile_id, true, ctx);
    });
    state
}

fn initialize_permissions_test_with_mode(
    app: &mut App,
    mode: ExecutionMode,
    is_sandboxed: bool,
) -> PermissionsTestState {
    initialize_settings_for_tests_with_mode(app, mode, is_sandboxed);
    let global_resource_handles = GlobalResourceHandles::mock(app);
    app.add_singleton_model(|_| GlobalResourceHandlesProvider::new(global_resource_handles));
    let history = app.add_singleton_model(|_| BlocklistAIHistoryModel::new(vec![], &[]));
    app.add_singleton_model(|_| CLIAgentSessionsModel::new());
    app.add_singleton_model(|_| ActiveAgentViewsModel::new());
    app.add_singleton_model(AgentNotificationsModel::new);
    let permissions = app.add_singleton_model(BlocklistAIPermissions::new);
    let terminal_view_id = EntityId::new();
    app.add_singleton_model(|_| AuthStateProvider::new_for_test());
    app.add_singleton_model(SyncQueue::mock);
    app.add_singleton_model(|_| NetworkStatus::new());
    app.add_singleton_model(TeamTesterStatus::mock);
    app.add_singleton_model(UpdateManager::mock);
    app.add_singleton_model(CloudModel::mock);
    app.add_singleton_model(|_| TemplatableMCPServerManager::default());
    let profile_model = app.add_singleton_model(|ctx| {
        AIExecutionProfilesModel::new(&LaunchMode::new_for_unit_test(), ctx)
    });
    app.add_singleton_model(PrivacySettings::mock);
    let user_workspaces = app.add_singleton_model(UserWorkspaces::default_mock);

    let conversation_id = history.update(app, |history_model, ctx| {
        history_model.start_new_conversation(terminal_view_id, false, false, false, ctx)
    });

    PermissionsTestState {
        convo_id: conversation_id,
        permissions,
        history,
        terminal_view_id,
        user_workspaces,
        profile_model,
    }
}

#[test]
fn test_can_read_files_empty_paths() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        permissions.read(&app, |model, ctx| {
            let result = model.can_read_files_with_conversation(
                &convo_id,
                vec![],
                Some(terminal_view_id),
                ctx,
            );
            assert!(result.is_allowed());
            assert!(matches!(
                result,
                FileReadPermission::Allowed(FileReadPermissionAllowedReason::ExplicitlyAllowlisted)
            ));
        });
    })
}

// warp-cn fork: under `direct_llm_backend` we coerce AlwaysAsk for
// the read_files / mcp / write / pty gates the same way we do for
// execute_commands. The upstream assertions expecting AlwaysAsk to
// deny no longer hold; skip under the fork feature.
#[cfg_attr(feature = "direct_llm_backend", ignore = "see fork-specific coercion")]
#[test]
fn test_can_read_files_workspace_settings_override_profile() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            user_workspaces,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        profile_model.update(&mut app, |model, ctx| {
            model.set_read_files(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AlwaysAllow,
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            let result = model.can_read_files_with_conversation(
                &convo_id,
                vec![PathBuf::from("/test/file.txt")],
                Some(terminal_view_id),
                ctx,
            );
            assert!(result.is_allowed());
            assert!(matches!(
                result,
                FileReadPermission::Allowed(
                    FileReadPermissionAllowedReason::AutoreadSettingEnabled
                )
            ));
        });

        // Now set the workspace to AlwaysAsk
        user_workspaces.update(&mut app, |model, ctx| {
            model.setup_test_workspace(ctx);
            model.update_ai_autonomy_settings(
                |settings| {
                    settings.read_files_setting = Some(ActionPermission::AlwaysAsk);
                },
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            let result = model.can_read_files_with_conversation(
                &convo_id,
                vec![PathBuf::from("/test/file.txt")],
                Some(terminal_view_id),
                ctx,
            );
            assert!(!result.is_allowed());
            assert!(
                matches!(
                    result,
                    FileReadPermission::Denied(FileReadPermissionDeniedReason::AlwaysAskEnabled)
                ),
                "the workspace setting should override the profile setting"
            );
        });
    })
}

#[cfg_attr(feature = "direct_llm_backend", ignore = "see fork-specific coercion")]
#[test]
fn test_can_read_files_profile_workspace_allowlist_interaction() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            user_workspaces,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        // Set up profile with allowlist and AlwaysAsk
        profile_model.update(&mut app, |model, ctx| {
            model.set_read_files(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AlwaysAsk,
                ctx,
            );
            model.add_to_directory_allowlist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &PathBuf::from("/profile/allowed"),
                ctx,
            );
        });

        // Test that files in profile's allowlist are allowed
        permissions.read(&app, |model, ctx| {
            let result = model.can_read_files_with_conversation(
                &convo_id,
                vec![PathBuf::from("/profile/allowed/file.txt")],
                Some(terminal_view_id),
                ctx,
            );
            assert!(result.is_allowed());
            assert!(matches!(
                result,
                FileReadPermission::Allowed(FileReadPermissionAllowedReason::ExplicitlyAllowlisted)
            ));

            // Test that files not in profile's allowlist are denied
            let result = model.can_read_files_with_conversation(
                &convo_id,
                vec![PathBuf::from("/not/allowed/file.txt")],
                Some(terminal_view_id),
                ctx,
            );
            assert!(!result.is_allowed());
            assert!(matches!(
                result,
                FileReadPermission::Denied(FileReadPermissionDeniedReason::AlwaysAskEnabled)
            ));
        });

        // Set up workspace with AlwaysAsk but no allowlist
        user_workspaces.update(&mut app, |model, ctx| {
            model.setup_test_workspace(ctx);
            model.update_ai_autonomy_settings(
                |settings| {
                    settings.read_files_setting = Some(ActionPermission::AlwaysAsk);
                    settings.read_files_allowlist = None;
                },
                ctx,
            );
        });

        // Test that the user's profile is respected when there's no workspace allowlist
        permissions.read(&app, |model, ctx| {
            let result = model.can_read_files_with_conversation(
                &convo_id,
                vec![PathBuf::from("/profile/allowed/file.txt")],
                Some(terminal_view_id),
                ctx,
            );
            assert!(result.is_allowed());
            assert!(matches!(
                result,
                FileReadPermission::Allowed(FileReadPermissionAllowedReason::ExplicitlyAllowlisted)
            ));
        });

        // Set up workspace with AlwaysAsk and a different allowlist
        user_workspaces.update(&mut app, |model, ctx| {
            model.update_ai_autonomy_settings(
                |settings| {
                    settings.read_files_setting = Some(ActionPermission::AlwaysAsk);
                    settings.read_files_allowlist = Some(vec![PathBuf::from("/workspace/allowed")]);
                },
                ctx,
            );
        });

        // Test that workspace allowlist takes precedence
        permissions.read(&app, |model, ctx| {
            // Files in workspace allowlist should be allowed
            let result = model.can_read_files_with_conversation(
                &convo_id,
                vec![PathBuf::from("/workspace/allowed/file.txt")],
                Some(terminal_view_id),
                ctx,
            );
            assert!(result.is_allowed());
            assert!(matches!(
                result,
                FileReadPermission::Allowed(FileReadPermissionAllowedReason::ExplicitlyAllowlisted)
            ));

            // Files in profile allowlist but not workspace allowlist should be denied
            let result = model.can_read_files_with_conversation(
                &convo_id,
                vec![PathBuf::from("/profile/allowed/file.txt")],
                Some(terminal_view_id),
                ctx,
            );
            assert!(!result.is_allowed());
            assert!(matches!(
                result,
                FileReadPermission::Denied(FileReadPermissionDeniedReason::AlwaysAskEnabled)
            ));

            // Files in neither allowlist should be denied
            let result = model.can_read_files_with_conversation(
                &convo_id,
                vec![PathBuf::from("/not/allowed/file.txt")],
                Some(terminal_view_id),
                ctx,
            );
            assert!(!result.is_allowed());
            assert!(matches!(
                result,
                FileReadPermission::Denied(FileReadPermissionDeniedReason::AlwaysAskEnabled)
            ));
        });
    })
}

#[cfg_attr(feature = "direct_llm_backend", ignore = "see fork-specific coercion")]
#[test]
fn test_can_write_files() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            terminal_view_id,
            convo_id,
            permissions,
            profile_model,
            ..
        } = initialize_permissions_test(&mut app);

        // Test AgentDecides setting
        profile_model.update(&mut app, |model, ctx| {
            model.set_apply_code_diffs(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AgentDecides,
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            let result = model.can_write_files(&convo_id, &[], Some(terminal_view_id), ctx);
            assert!(!result.is_allowed());
            assert!(
                matches!(
                    result,
                    FileWritePermission::Denied(FileWritePermissionDeniedReason::AgentDecided)
                ),
                "not allowed because AgentDecides right now just means ask"
            );
        });

        // Test AlwaysAllow setting
        profile_model.update(&mut app, |model, ctx| {
            model.set_apply_code_diffs(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AlwaysAllow,
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            let result = model.can_write_files(&convo_id, &[], Some(terminal_view_id), ctx);
            assert!(result.is_allowed());
            assert!(matches!(
                result,
                FileWritePermission::Allowed(
                    FileWritePermissionAllowedReason::AutowriteSettingEnabled
                )
            ));
        });

        // Test AlwaysAsk setting
        profile_model.update(&mut app, |model, ctx| {
            model.set_apply_code_diffs(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AlwaysAsk,
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            let result = model.can_write_files(&convo_id, &[], Some(terminal_view_id), ctx);
            assert!(!result.is_allowed());
            assert!(matches!(
                result,
                FileWritePermission::Denied(FileWritePermissionDeniedReason::AlwaysAskEnabled)
            ));
        });
    })
}

#[cfg_attr(feature = "direct_llm_backend", ignore = "see fork-specific coercion")]
#[test]
fn test_can_write_files_workspace_settings_override_profile() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            user_workspaces,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        // Set profile to AlwaysAllow
        profile_model.update(&mut app, |model, ctx| {
            model.set_apply_code_diffs(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AlwaysAllow,
                ctx,
            );
        });

        // Test that profile setting is respected when no workspace setting
        permissions.read(&app, |model, ctx| {
            let result = model.can_write_files(&convo_id, &[], Some(terminal_view_id), ctx);
            assert!(result.is_allowed());
            assert!(matches!(
                result,
                FileWritePermission::Allowed(
                    FileWritePermissionAllowedReason::AutowriteSettingEnabled
                )
            ));
        });

        // Set workspace to AlwaysAsk
        user_workspaces.update(&mut app, |model, ctx| {
            model.setup_test_workspace(ctx);
            model.update_ai_autonomy_settings(
                |settings| {
                    settings.apply_code_diffs_setting = Some(ActionPermission::AlwaysAsk);
                },
                ctx,
            );
        });

        // Test that workspace setting overrides profile
        permissions.read(&app, |model, ctx| {
            let result = model.can_write_files(&convo_id, &[], Some(terminal_view_id), ctx);
            assert!(!result.is_allowed());
            assert!(matches!(
                result,
                FileWritePermission::Denied(FileWritePermissionDeniedReason::AlwaysAskEnabled)
            ));
        });
    })
}

#[test]
fn test_can_write_files_mcp_config_always_denied() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            terminal_view_id,
            convo_id,
            permissions,
            profile_model,
            ..
        } = initialize_permissions_test(&mut app);

        // Even with AlwaysAllow, writing to an MCP config must be denied.
        profile_model.update(&mut app, |model, ctx| {
            model.set_apply_code_diffs(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AlwaysAllow,
                ctx,
            );
        });

        let mcp_config_paths = vec![
            PathBuf::from("/project/.mcp.json"),
            PathBuf::from("/project/.warp/.mcp.json"),
            PathBuf::from("/project/.codex/config.toml"),
        ];

        for path in mcp_config_paths {
            permissions.read(&app, |model, ctx| {
                let result = model.can_write_files(
                    &convo_id,
                    std::slice::from_ref(&path),
                    Some(terminal_view_id),
                    ctx,
                );
                assert!(
                    !result.is_allowed(),
                    "expected MCP config path {path:?} to be denied"
                );
                assert!(
                    matches!(
                        result,
                        FileWritePermission::Denied(FileWritePermissionDeniedReason::ProtectedPath)
                    ),
                    "expected ProtectedPath denial for {path:?}, got {result:?}"
                );
            });
        }
    })
}

// warp-cn fork: under `direct_llm_backend` we coerce AlwaysAsk →
// AgentDecides for command execution, so the upstream assertion that
// AlwaysAsk denies non-allowlisted commands no longer holds for that
// build. Skip the test when the feature is on; the same scenario is
// covered by `test_direct_backend_coerces_always_ask_to_agent_decides`.
#[cfg_attr(feature = "direct_llm_backend", ignore = "see fork-specific coercion test")]
#[test]
fn test_can_autoexecute_command_workspace_settings_override_profile() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            user_workspaces,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        // Set profile to AlwaysAllow
        profile_model.update(&mut app, |model, ctx| {
            model.set_execute_commands(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AlwaysAllow,
                ctx,
            );
        });

        // Test that profile setting is respected when no workspace setting
        permissions.read(&app, |model, ctx| {
            let result = model.can_autoexecute_command(
                &convo_id,
                "git status",
                EscapeChar::Backslash,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(result.is_allowed());
            assert!(matches!(
                result,
                CommandExecutionPermission::Allowed(
                    CommandExecutionPermissionAllowedReason::AlwaysAllowed
                )
            ));
        });

        // Set workspace to AlwaysAsk
        user_workspaces.update(&mut app, |model, ctx| {
            model.setup_test_workspace(ctx);
            model.update_ai_autonomy_settings(
                |settings| {
                    settings.execute_commands_setting = Some(ActionPermission::AlwaysAsk);
                },
                ctx,
            );
        });

        // Test that workspace setting overrides profile
        permissions.read(&app, |model, ctx| {
            let result = model.can_autoexecute_command(
                &convo_id,
                "git status",
                EscapeChar::Backslash,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(!result.is_allowed());
            assert!(matches!(
                result,
                CommandExecutionPermission::Denied(
                    CommandExecutionPermissionDeniedReason::AlwaysAskEnabled
                )
            ));
        });
    })
}

#[test]
fn test_can_autoexecute_command_denylist_precedence() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            user_workspaces,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        // Set up profile with denylist
        profile_model.update(&mut app, |model, ctx| {
            model.add_to_command_denylist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &AgentModeCommandExecutionPredicate::new_regex("rm .*").unwrap(),
                ctx,
            );
        });

        // Test that profile denylist is respected when no workspace denylist
        permissions.read(&app, |model, ctx| {
            let result = model.can_autoexecute_command(
                &convo_id,
                "rm file.txt",
                EscapeChar::Backslash,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(!result.is_allowed());
            assert!(matches!(
                result,
                CommandExecutionPermission::Denied(
                    CommandExecutionPermissionDeniedReason::ExplicitlyDenylisted
                )
            ));
        });

        // Set workspace denylist
        user_workspaces.update(&mut app, |model, ctx| {
            model.setup_test_workspace(ctx);
            model.update_ai_autonomy_settings(
                |settings| {
                    settings.execute_commands_denylist =
                        Some(vec![AgentModeCommandExecutionPredicate::new_regex(
                            "git .*",
                        )
                        .unwrap()]);
                },
                ctx,
            );
        });

        // Org + user denylists are merged: both should be active
        permissions.read(&app, |model, ctx| {
            let result = model.can_autoexecute_command(
                &convo_id,
                "git status",
                EscapeChar::Backslash,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(!result.is_allowed());
            assert!(matches!(
                result,
                CommandExecutionPermission::Denied(
                    CommandExecutionPermissionDeniedReason::ExplicitlyDenylisted
                )
            ));

            let result = model.can_autoexecute_command(
                &convo_id,
                "rm file.txt",
                EscapeChar::Backslash,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(
                matches!(
                    result,
                    CommandExecutionPermission::Denied(
                        CommandExecutionPermissionDeniedReason::ExplicitlyDenylisted
                    )
                ),
                "user denylist entries should be merged with org denylist, not replaced"
            );
        });
    })
}

// warp-cn fork: same skip rationale as
// `test_can_autoexecute_command_workspace_settings_override_profile`.
#[cfg_attr(feature = "direct_llm_backend", ignore = "see fork-specific coercion test")]
#[test]
fn test_can_autoexecute_command_denylist_matches_env_prefixed_commands() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        profile_model.update(&mut app, |model, ctx| {
            let profile_id = *model.active_profile(Some(terminal_view_id), ctx).id();
            model.set_execute_commands(profile_id, &ActionPermission::AlwaysAllow, ctx);
            model.add_to_command_denylist(
                profile_id,
                &AgentModeCommandExecutionPredicate::new_regex("rm .*").unwrap(),
                ctx,
            );
        });

        for command in [
            "X=1 rm file.txt",
            "echo ok && X=1 rm file.txt",
            "echo $(X=1 rm file.txt)",
        ] {
            permissions.read(&app, |model, ctx| {
                let result = model.can_autoexecute_command(
                    &convo_id,
                    command,
                    EscapeChar::Backslash,
                    false,
                    None,
                    Some(terminal_view_id),
                    ctx,
                );
                assert!(
                    matches!(
                        result,
                        CommandExecutionPermission::Denied(
                            CommandExecutionPermissionDeniedReason::ExplicitlyDenylisted
                        )
                    ),
                    "{command:?} should be denied by the rm denylist, got {result:?}"
                );
            });
        }

        permissions.read(&app, |model, ctx| {
            let result = model.can_autoexecute_command(
                &convo_id,
                "X=1 git status",
                EscapeChar::Backslash,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(matches!(
                result,
                CommandExecutionPermission::Allowed(
                    CommandExecutionPermissionAllowedReason::AlwaysAllowed
                )
            ));
        });
    })
}

#[test]
fn test_can_autoexecute_command_allowlist_precedence() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            user_workspaces,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        // Set up profile with AlwaysAsk and allowlist
        profile_model.update(&mut app, |model, ctx| {
            model.set_execute_commands(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AlwaysAsk,
                ctx,
            );
            model.add_to_command_allowlist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &AgentModeCommandExecutionPredicate::new_regex("git .*").unwrap(),
                ctx,
            );
        });

        // Test that profile allowlist is respected when no workspace allowlist
        permissions.read(&app, |model, ctx| {
            let result = model.can_autoexecute_command(
                &convo_id,
                "git status",
                EscapeChar::Backslash,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(result.is_allowed());
            assert!(matches!(
                result,
                CommandExecutionPermission::Allowed(
                    CommandExecutionPermissionAllowedReason::ExplicitlyAllowlisted
                )
            ));
        });

        // Set workspace with AlwaysAsk and different allowlist
        user_workspaces.update(&mut app, |model, ctx| {
            model.setup_test_workspace(ctx);
            model.update_ai_autonomy_settings(
                |settings| {
                    settings.execute_commands_setting = Some(ActionPermission::AlwaysAsk);
                    settings.execute_commands_allowlist = Some(vec![
                        AgentModeCommandExecutionPredicate::new_regex("ls .*").unwrap(),
                    ]);
                },
                ctx,
            );
        });

        // Test that workspace allowlist overrides profile allowlist
        permissions.read(&app, |model, ctx| {
            // git commands should now be denied (not in workspace allowlist)
            let result = model.can_autoexecute_command(
                &convo_id,
                "git status",
                EscapeChar::Backslash,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(!result.is_allowed());
            assert!(matches!(
                result,
                CommandExecutionPermission::Denied(
                    CommandExecutionPermissionDeniedReason::AlwaysAskEnabled
                )
            ));

            // ls commands should now be allowed
            let result = model.can_autoexecute_command(
                &convo_id,
                "ls -l",
                EscapeChar::Backslash,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(result.is_allowed());
            assert!(matches!(
                result,
                CommandExecutionPermission::Allowed(
                    CommandExecutionPermissionAllowedReason::ExplicitlyAllowlisted
                )
            ));

            let result = model.can_autoexecute_command(
                &convo_id,
                "PATH=/tmp ls -l",
                EscapeChar::Backslash,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(
                matches!(
                    result,
                    CommandExecutionPermission::Denied(
                        CommandExecutionPermissionDeniedReason::AlwaysAskEnabled
                    )
                ),
                "env-prefixed commands should not be normalized for allowlist matching"
            );
        });
    })
}

#[test]
fn test_can_autoexecute_command_denylist_beats_run_to_completion() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            history,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        // Add a denylist rule that matches the test command.
        profile_model.update(&mut app, |model, ctx| {
            model.add_to_command_denylist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &AgentModeCommandExecutionPredicate::new_regex("rm .*").unwrap(),
                ctx,
            );
        });

        // Toggle run-to-completion override for this conversation.
        history.update(&mut app, |history, ctx| {
            history.toggle_autoexecute_override(&convo_id, terminal_view_id, ctx);
        });

        // Despite run-to-completion, denylist must take precedence and deny execution.
        permissions.read(&app, |model, ctx| {
            let result = model.can_autoexecute_command(
                &convo_id,
                "rm important.txt",
                EscapeChar::Backslash,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(!result.is_allowed());
            assert!(matches!(
                result,
                CommandExecutionPermission::Denied(
                    CommandExecutionPermissionDeniedReason::ExplicitlyDenylisted
                )
            ));
        });
    })
}

#[test]
fn test_can_autoexecute_command_run_to_completion_allows_non_denylisted() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            history,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        // Enable run-to-completion override for the conversation.
        history.update(&mut app, |history, ctx| {
            history.toggle_autoexecute_override(&convo_id, terminal_view_id, ctx);
        });

        // Since the command is not denylisted, the override should allow execution with RunToCompletion.
        permissions.read(&app, |model, ctx| {
            let result = model.can_autoexecute_command(
                &convo_id,
                "echo hello",
                EscapeChar::Backslash,
                true,        // read-only command
                Some(false), // not risky
                Some(terminal_view_id),
                ctx,
            );
            assert!(result.is_allowed());
            assert!(matches!(
                result,
                CommandExecutionPermission::Allowed(
                    CommandExecutionPermissionAllowedReason::RunToCompletion
                )
            ));
        });
    })
}

#[cfg_attr(feature = "direct_llm_backend", ignore = "see fork-specific coercion")]
#[test]
fn test_can_write_to_pty() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            user_workspaces,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        // Set profile to AlwaysAllow
        profile_model.update(&mut app, |model, ctx| {
            model.set_write_to_pty(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &WriteToPtyPermission::AlwaysAllow,
                ctx,
            );
        });

        // Test that profile setting is respected when no workspace setting
        permissions.read(&app, |model, ctx| {
            let result = model.can_write_to_pty(&convo_id, Some(terminal_view_id), ctx);
            assert_eq!(result, WriteToPtyPermission::AlwaysAllow);
        });

        // Set workspace to AlwaysAsk
        user_workspaces.update(&mut app, |model, ctx| {
            model.setup_test_workspace(ctx);
            model.update_ai_autonomy_settings(
                |settings| {
                    settings.write_to_pty_setting = Some(WriteToPtyPermission::AlwaysAsk);
                },
                ctx,
            );
        });

        // Test that workspace setting overrides profile
        permissions.read(&app, |model, ctx| {
            let result = model.can_write_to_pty(&convo_id, Some(terminal_view_id), ctx);
            assert_eq!(result, WriteToPtyPermission::AlwaysAsk);
        });
    })
}

#[test]
fn test_can_use_mcp_server_always_allow_no_denylist() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        let server_uuid = Uuid::new_v4();

        profile_model.update(&mut app, |model, ctx| {
            model.set_mcp_permissions(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AlwaysAllow,
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            // Any server should be allowed when AlwaysAllow and not denylisted.
            assert!(model.can_use_mcp_server(
                &convo_id,
                Some(server_uuid),
                Some(terminal_view_id),
                ctx
            ));
            // None UUID should also be allowed (no denylist match possible).
            assert!(model.can_use_mcp_server(&convo_id, None, Some(terminal_view_id), ctx));
        });
    })
}

#[test]
fn test_can_use_mcp_server_always_allow_with_denylist() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        let server_uuid = Uuid::new_v4();
        let other_uuid = Uuid::new_v4();

        profile_model.update(&mut app, |model, ctx| {
            model.set_mcp_permissions(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AlwaysAllow,
                ctx,
            );
            model.add_to_mcp_denylist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &server_uuid,
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            // Denylisted server should be denied.
            assert!(!model.can_use_mcp_server(
                &convo_id,
                Some(server_uuid),
                Some(terminal_view_id),
                ctx
            ));
            // Non-denylisted server should be allowed.
            assert!(model.can_use_mcp_server(
                &convo_id,
                Some(other_uuid),
                Some(terminal_view_id),
                ctx
            ));
        });
    })
}

#[test]
fn test_can_use_mcp_server_always_ask_with_allowlist() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        let server_uuid = Uuid::new_v4();
        let other_uuid = Uuid::new_v4();

        profile_model.update(&mut app, |model, ctx| {
            model.set_mcp_permissions(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AlwaysAsk,
                ctx,
            );
            model.add_to_mcp_allowlist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &server_uuid,
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            // Allowlisted server should be allowed.
            assert!(model.can_use_mcp_server(
                &convo_id,
                Some(server_uuid),
                Some(terminal_view_id),
                ctx
            ));
            // Non-allowlisted server should be denied.
            assert!(!model.can_use_mcp_server(
                &convo_id,
                Some(other_uuid),
                Some(terminal_view_id),
                ctx
            ));
            // None UUID should be denied.
            assert!(!model.can_use_mcp_server(&convo_id, None, Some(terminal_view_id), ctx));
        });
    })
}

#[test]
fn test_can_use_mcp_server_always_ask_denylist_overrides_allowlist() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        let server_uuid = Uuid::new_v4();

        profile_model.update(&mut app, |model, ctx| {
            model.set_mcp_permissions(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AlwaysAsk,
                ctx,
            );
            model.add_to_mcp_allowlist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &server_uuid,
                ctx,
            );
            model.add_to_mcp_denylist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &server_uuid,
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            // Both allowlisted and denylisted: denylist wins.
            assert!(!model.can_use_mcp_server(
                &convo_id,
                Some(server_uuid),
                Some(terminal_view_id),
                ctx
            ));
        });
    })
}

#[test]
fn test_can_use_mcp_server_agent_decides() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        let server_uuid = Uuid::new_v4();
        let other_uuid = Uuid::new_v4();

        profile_model.update(&mut app, |model, ctx| {
            model.set_mcp_permissions(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AgentDecides,
                ctx,
            );
            model.add_to_mcp_allowlist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &server_uuid,
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            // Allowlisted and not denylisted should be allowed.
            assert!(model.can_use_mcp_server(
                &convo_id,
                Some(server_uuid),
                Some(terminal_view_id),
                ctx
            ));
            // Not allowlisted should be denied.
            assert!(!model.can_use_mcp_server(
                &convo_id,
                Some(other_uuid),
                Some(terminal_view_id),
                ctx
            ));
        });
    })
}

#[test]
fn test_can_use_mcp_server_agent_decides_denylist_overrides_allowlist() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        let server_uuid = Uuid::new_v4();

        profile_model.update(&mut app, |model, ctx| {
            model.set_mcp_permissions(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &ActionPermission::AgentDecides,
                ctx,
            );
            model.add_to_mcp_allowlist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &server_uuid,
                ctx,
            );
            model.add_to_mcp_denylist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &server_uuid,
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            // Both allowlisted and denylisted: denylist wins.
            assert!(!model.can_use_mcp_server(
                &convo_id,
                Some(server_uuid),
                Some(terminal_view_id),
                ctx
            ));
        });
    })
}

#[test]
fn test_sandboxed_mode_allows_read_write_files() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            user_workspaces,
            terminal_view_id,
            ..
        } = initialize_permissions_test_sandboxed(&mut app);

        // Set workspace to AlwaysAsk
        user_workspaces.update(&mut app, |model, ctx| {
            model.setup_test_workspace(ctx);
            model.update_ai_autonomy_settings(
                |settings| {
                    settings.apply_code_diffs_setting = Some(ActionPermission::AlwaysAsk);
                    settings.read_files_setting = Some(ActionPermission::AlwaysAsk);
                },
                ctx,
            );
        });

        // In sandboxed mode the workspace read/write restrictions are bypassed,
        // so the profile's AlwaysAllow setting takes effect.
        permissions.read(&app, |model, ctx| {
            let result = model.can_write_files(&convo_id, &[], Some(terminal_view_id), ctx);
            assert!(
                result.is_allowed(),
                "write files should be allowed in sandboxed mode (workspace restriction bypassed)"
            );
            assert!(matches!(
                result,
                FileWritePermission::Allowed(
                    FileWritePermissionAllowedReason::AutowriteSettingEnabled
                )
            ));

            let result = model.can_read_files_with_conversation(
                &convo_id,
                vec![PathBuf::from("/test/file.txt")],
                Some(terminal_view_id),
                ctx,
            );
            assert!(
                result.is_allowed(),
                "read files should be allowed in sandboxed mode (workspace restriction bypassed)"
            );
            assert!(matches!(
                result,
                FileReadPermission::Allowed(
                    FileReadPermissionAllowedReason::AutoreadSettingEnabled
                )
            ));
        });
    })
}

#[test]
fn test_sandboxed_denylist_used_in_sandboxed_mode() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            user_workspaces,
            terminal_view_id,
            ..
        } = initialize_permissions_test_sandboxed(&mut app);

        user_workspaces.update(&mut app, |model, ctx| {
            model.setup_test_workspace(ctx);
            // Regular workspace denylist blocks "git .*".
            model.update_ai_autonomy_settings(
                |settings| {
                    settings.execute_commands_denylist =
                        Some(vec![AgentModeCommandExecutionPredicate::new_regex(
                            "git .*",
                        )
                        .unwrap()]);
                },
                ctx,
            );
            // Sandboxed denylist blocks "rm .*" instead.
            model.update_sandboxed_agent_settings(
                |settings| {
                    *settings = Some(SandboxedAgentSettings {
                        execute_commands_denylist: Some(vec![
                            AgentModeCommandExecutionPredicate::new_regex("rm .*").unwrap(),
                        ]),
                    });
                },
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            // "git status" should be allowed: the regular denylist is not consulted in
            // sandboxed mode, so only the sandboxed denylist ("rm .*") applies.
            let result = model.can_autoexecute_command(
                &convo_id,
                "git status",
                EscapeChar::Backslash,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(
                result.is_allowed(),
                "git status should be allowed in sandboxed mode (regular denylist bypassed)"
            );

            // "rm file.txt" should be denied by the sandboxed denylist.
            let result = model.can_autoexecute_command(
                &convo_id,
                "rm file.txt",
                EscapeChar::Backslash,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(!result.is_allowed());
            assert!(
                matches!(
                    result,
                    CommandExecutionPermission::Denied(
                        CommandExecutionPermissionDeniedReason::ExplicitlyDenylisted
                    )
                ),
                "rm file.txt should be denied by the sandboxed denylist"
            );
        });
    })
}

#[test]
fn test_merged_denylist_deduplication() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            permissions,
            user_workspaces,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        let rm_predicate = AgentModeCommandExecutionPredicate::new_regex("rm .*").unwrap();

        profile_model.update(&mut app, |model, ctx| {
            model.add_to_command_denylist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &rm_predicate,
                ctx,
            );
        });

        user_workspaces.update(&mut app, |model, ctx| {
            model.setup_test_workspace(ctx);
            model.update_ai_autonomy_settings(
                |settings| {
                    settings.execute_commands_denylist = Some(vec![
                        AgentModeCommandExecutionPredicate::new_regex("rm .*").unwrap(),
                        AgentModeCommandExecutionPredicate::new_regex("git .*").unwrap(),
                    ]);
                },
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            let denylist = model.get_execute_commands_denylist(ctx, Some(terminal_view_id));
            let rm_count = denylist.iter().filter(|p| p.to_string() == "rm .*").count();
            assert_eq!(rm_count, 1, "duplicate entries should be deduplicated");
            assert!(
                denylist.iter().any(|p| p.to_string() == "git .*"),
                "org entry 'git .*' should be in merged list"
            );
        });
    })
}

#[test]
fn test_get_org_execute_commands_denylist() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            permissions,
            user_workspaces,
            ..
        } = initialize_permissions_test(&mut app);

        permissions.read(&app, |_, ctx| {
            let org_list = BlocklistAIPermissions::get_org_execute_commands_denylist(ctx);
            assert!(org_list.is_empty());
        });

        user_workspaces.update(&mut app, |model, ctx| {
            model.setup_test_workspace(ctx);
            model.update_ai_autonomy_settings(
                |settings| {
                    settings.execute_commands_denylist =
                        Some(vec![AgentModeCommandExecutionPredicate::new_regex(
                            "git .*",
                        )
                        .unwrap()]);
                },
                ctx,
            );
        });

        permissions.read(&app, |_, ctx| {
            let org_list = BlocklistAIPermissions::get_org_execute_commands_denylist(ctx);
            assert_eq!(org_list.len(), 1);
            assert_eq!(org_list[0].to_string(), "git .*");
        });
    })
}

#[test]
fn test_empty_org_denylist_allows_user_entries() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            user_workspaces,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        profile_model.update(&mut app, |model, ctx| {
            model.add_to_command_denylist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &AgentModeCommandExecutionPredicate::new_regex("rm .*").unwrap(),
                ctx,
            );
        });

        user_workspaces.update(&mut app, |model, ctx| {
            model.setup_test_workspace(ctx);
            model.update_ai_autonomy_settings(
                |settings| {
                    settings.execute_commands_denylist = Some(vec![]);
                },
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            let result = model.can_autoexecute_command(
                &convo_id,
                "rm file.txt",
                EscapeChar::Backslash,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(
                !result.is_allowed(),
                "user denylist entry should be active even when org denylist is empty"
            );
        });
    })
}

#[test]
fn test_denylist_matches_multiline_commands() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            profile_model,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        // Add denylist rule for rm
        profile_model.update(&mut app, |model, ctx| {
            model.add_to_command_denylist(
                *model.active_profile(Some(terminal_view_id), ctx).id(),
                &AgentModeCommandExecutionPredicate::new_regex("rm .*").unwrap(),
                ctx,
            );
        });

        // Single-line rm command should be denied
        permissions.read(&app, |model, ctx| {
            let result = model.can_autoexecute_command(
                &convo_id,
                "rm file.txt",
                EscapeChar::Backslash,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(!result.is_allowed());
            assert!(matches!(
                result,
                CommandExecutionPermission::Denied(
                    CommandExecutionPermissionDeniedReason::ExplicitlyDenylisted
                )
            ));
        });

        // Multiline rm command with backslash continuations should also be denied (POSIX)
        permissions.read(&app, |model, ctx| {
            let result = model.can_autoexecute_command(
                &convo_id,
                "rm file1.txt \\\nfile2.txt \\\nfile3.txt",
                EscapeChar::Backslash,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(
                !result.is_allowed(),
                "multiline rm command should be denied by denylist"
            );
            assert!(matches!(
                result,
                CommandExecutionPermission::Denied(
                    CommandExecutionPermissionDeniedReason::ExplicitlyDenylisted
                )
            ));
        });

        // Env-prefixed multiline rm command should also be denied after normalization.
        permissions.read(&app, |model, ctx| {
            let result = model.can_autoexecute_command(
                &convo_id,
                "X=1 rm file1.txt \\\nfile2.txt \\\nfile3.txt",
                EscapeChar::Backslash,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(
                !result.is_allowed(),
                "env-prefixed multiline rm command should be denied by denylist"
            );
            assert!(matches!(
                result,
                CommandExecutionPermission::Denied(
                    CommandExecutionPermissionDeniedReason::ExplicitlyDenylisted
                )
            ));
        });

        // Multiline rm command with backtick continuations should also be denied (PowerShell)
        permissions.read(&app, |model, ctx| {
            let result = model.can_autoexecute_command(
                &convo_id,
                "rm file1.txt `\nfile2.txt `\nfile3.txt",
                EscapeChar::Backtick,
                false,
                None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(
                !result.is_allowed(),
                "multiline rm command with backtick continuations should be denied by denylist"
            );
            assert!(matches!(
                result,
                CommandExecutionPermission::Denied(
                    CommandExecutionPermissionDeniedReason::ExplicitlyDenylisted
                )
            ));
        });
    })
}

/// warp-cn fork: reproduces the v4-flash batch from the direct_llm_backend
/// `/plan` session — every command was correctly classified `read_only`
/// by the model but a subset still got ⊘ in the UI. Pinning the exact
/// command strings + the exact `(is_read_only=true, is_risky=None)` mapping
/// our adapter emits, against the *unmodified* default profile, lets us
/// regression-test the gate without restarting the bundle each time.
#[cfg(feature = "direct_llm_backend")]
#[test]
fn test_direct_backend_v4_flash_read_only_batch_under_default_profile() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        // Real `cd <abs> && <cmd>` strings v4-flash emitted, sampled from
        // both the ✓ and the ⊘ buckets in the user's screenshot trio.
        let cmds = [
            // Plain — these should always pass on `is_read_only=true`.
            "cd /Users/liji/warp && ls -la",
            "cd /Users/liji/warp && ls app/src/",
            "cd /Users/liji/warp && git log --oneline -15",
            "cd /Users/liji/warp && cat rust-toolchain.toml",
            "cd /Users/liji/warp && wc -l Cargo.toml",
            // Pipe — no redirection, just `|`.
            "cd /Users/liji/warp && ls crates/ | sort",
            "cd /Users/liji/warp && ls crates/ | wc -l && ls crates/ | sort",
            // /dev/null silenced stderr — covered by the
            // `only_dev_null_redirections` carve-out in permissions.rs.
            "cd /Users/liji/warp && cat app/src/features.rs 2>/dev/null | head -80",
            "cd /Users/liji/warp && ls -la docs/ 2>/dev/null; ls scripts/ 2>/dev/null",
            "cd /Users/liji/warp && git remote -v 2>/dev/null",
            r#"cd /Users/liji/warp && cat rust-toolchain.toml 2>/dev/null || echo "not found""#,
        ];

        permissions.read(&app, |model, ctx| {
            for cmd in cmds {
                let result = model.can_autoexecute_command(
                    &convo_id,
                    cmd,
                    EscapeChar::Backslash,
                    /* is_read_only = */ true,
                    /* is_risky    = */ None, // matches our adapter's mapping
                    Some(terminal_view_id),
                    ctx,
                );
                assert!(
                    result.is_allowed(),
                    "expected ALLOW for read-only cmd `{cmd}`, got {result:?}",
                );
            }
        });
    })
}

/// warp-cn fork: pin the exact ⊘ batch from screenshot #31 (v4-flash
/// session timestamp 11:46:34Z). All 7 commands are read_only with
/// pipes / `2>/dev/null` / `$()` substitutions and were rejected by
/// the runtime client. If this test PASSES, the gating code is
/// correct and the runtime ⊘ comes from a state outside the gate
/// (workspace override, conversation autoexecute_override flip,
/// stale profile cache). If it FAILS, we have an actual logic bug
/// in `decompose_command` / `only_dev_null_redirections` /
/// `contains_redirection` reporting and need to fix it here.
#[cfg(feature = "direct_llm_backend")]
#[test]
fn test_direct_backend_screenshot31_pipe_and_dev_null_batch() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        let cmds = [
            // Plain pipes, NO redirection.
            "find . -name \"*.rs\" -not -path \"*/target/*\" -not -path \"*/node_modules/*\" | wc -l",
            "ls -la app/src/ai/ | head -30",
            // Single command + 2>/dev/null silenced stderr.
            "git log --oneline -20 2>/dev/null",
            "git rev-list --count HEAD 2>/dev/null",
            // 2>/dev/null + ; separator + pipe.
            "cat specs/specs_list.txt 2>/dev/null; ls specs/ | wc -l",
            // $() substitution + 2>/dev/null inside + 2>/dev/null outside + pipe.
            r#"wc -l $(find . -name "Cargo.toml" -not -path "*/target/*" -not -path "*/node_modules/*") 2>/dev/null | tail -5"#,
            // $() with -not -path globs (the in-progress yellow one from screenshot).
            r#"wc -l $(find . -name "*.rs" -path "*/src/*" -not -path "*/target/*" -not -path "*/node_modules/*" -not -path "*/venv/*" 2>/dev/null) | tail -5"#,
        ];

        permissions.read(&app, |model, ctx| {
            let mut failures = Vec::new();
            for cmd in cmds {
                let result = model.can_autoexecute_command(
                    &convo_id,
                    cmd,
                    EscapeChar::Backslash,
                    /* is_read_only = */ true,
                    /* is_risky    = */ None,
                    Some(terminal_view_id),
                    ctx,
                );
                if !result.is_allowed() {
                    failures.push(format!("{cmd:?} → {result:?}"));
                }
            }
            assert!(
                failures.is_empty(),
                "expected ALLOW for every screenshot-#31 command but got:\n{}",
                failures.join("\n"),
            );
        });
    })
}

/// warp-cn fork: even when an upstream-style `AlwaysAsk` setting is
/// persisted (saved profile / workspace override), `direct_llm_backend`
/// must coerce it to `AgentDecides` so the user's own-LLM agent loop
/// isn't trapped on every read-only inspection. Without the coercion,
/// the v4-flash session reproduces the exact ⊘ pattern from the
/// screenshot trio: identical commands flip pass→deny across turns
/// because nothing in the agent path can satisfy the popup.
#[cfg(feature = "direct_llm_backend")]
#[test]
fn test_direct_backend_coerces_always_ask_to_agent_decides() {
    App::test((), |mut app| async move {
        let PermissionsTestState {
            convo_id,
            permissions,
            user_workspaces,
            terminal_view_id,
            ..
        } = initialize_permissions_test(&mut app);

        // Simulate the offending state: workspace forces AlwaysAsk.
        user_workspaces.update(&mut app, |model, ctx| {
            model.setup_test_workspace(ctx);
            model.update_ai_autonomy_settings(
                |settings| {
                    settings.execute_commands_setting = Some(ActionPermission::AlwaysAsk);
                },
                ctx,
            );
        });

        permissions.read(&app, |model, ctx| {
            let result = model.can_autoexecute_command(
                &convo_id,
                "cd /Users/liji/warp && ls app/src/",
                EscapeChar::Backslash,
                /* is_read_only = */ true,
                /* is_risky    = */ None,
                Some(terminal_view_id),
                ctx,
            );
            assert!(
                result.is_allowed(),
                "with direct_llm_backend, AlwaysAsk should be coerced to \
                 AgentDecides for read-only commands; got {result:?}",
            );
        });
    })
}
