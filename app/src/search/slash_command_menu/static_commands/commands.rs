use std::collections::HashMap;
use std::sync::LazyLock;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp_core::features::FeatureFlag;

use super::Availability;
use crate::search::slash_command_menu::static_commands::Argument;
use crate::search::slash_command_menu::StaticCommand;
use crate::ui_components::color_dot;

pub static AGENT: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/agent",
    description_key: "command-slash-agent-desc",
    icon_path: "bundled/svg/oz.svg",
    availability: Availability::AI_ENABLED.union(Availability::NOT_CLOUD_AGENT),
    auto_enter_ai_mode: false,
    argument: Some(Argument::optional().with_execute_on_selection()),
});

pub static CLOUD_AGENT: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/cloud-agent",
    description_key: "command-slash-cloud-agent-desc",
    icon_path: "bundled/svg/oz-cloud.svg",
    availability: Availability::AI_ENABLED.union(Availability::NOT_CLOUD_AGENT),
    auto_enter_ai_mode: false,
    argument: Some(Argument::optional().with_execute_on_selection()),
});

pub const ADD_MCP: StaticCommand = StaticCommand {
    name: "/add-mcp",
    description_key: "command-slash-add-mcp-desc",
    icon_path: "bundled/svg/dataflow.svg",
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: None,
};

pub const PR_COMMENTS: StaticCommand = StaticCommand {
    name: "/pr-comments",
    description_key: "command-slash-pr-comments-desc",
    icon_path: "bundled/svg/github.svg",
    availability: Availability::REPOSITORY.union(Availability::AI_ENABLED),
    auto_enter_ai_mode: true,
    argument: None,
};

pub static CREATE_ENVIRONMENT: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/create-environment",
    description_key: "command-slash-create-environment-desc",
    icon_path: "bundled/svg/dataflow.svg",
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: Some(
        Argument::optional()
            .with_hint_text("<optional repo paths or GitHub URLs>")
            .with_execute_on_selection(),
    ),
});

pub const CREATE_DOCKER_SANDBOX: StaticCommand = StaticCommand {
    name: "/docker-sandbox",
    description_key: "command-slash-docker-sandbox-desc",
    icon_path: "bundled/svg/docker.svg",
    availability: Availability::LOCAL.union(Availability::AI_ENABLED),
    auto_enter_ai_mode: false,
    argument: None,
};

pub static CREATE_NEW_PROJECT: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/create-new-project",
    description_key: "command-slash-create-new-project-desc",
    icon_path: "bundled/svg/plus.svg",
    availability: Availability::LOCAL | Availability::AI_ENABLED,
    auto_enter_ai_mode: true,
    argument: Some(Argument::required().with_hint_text("<describe what you want to build>")),
});

pub static EDIT_SKILL: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/open-skill",
    description_key: "command-slash-open-skill-desc",
    icon_path: "bundled/svg/file-code-02.svg",
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: None,
});

pub static INVOKE_SKILL: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/skills",
    description_key: "command-slash-skills-desc",
    icon_path: "bundled/svg/stars-01.svg",
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: None,
});

pub static ADD_PROMPT: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/add-prompt",
    description_key: "command-slash-add-prompt-desc",
    icon_path: if FeatureFlag::AgentView.is_enabled() {
        "bundled/svg/prompt.svg"
    } else {
        "bundled/svg/agentmode.svg"
    },
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: None,
});

pub const ADD_RULE: StaticCommand = StaticCommand {
    name: "/add-rule",
    description_key: "command-slash-add-rule-desc",
    icon_path: "bundled/svg/book-open.svg",
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: None,
};

pub static EDIT: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/open-file",
    description_key: "command-slash-open-file-desc",
    icon_path: "bundled/svg/file-code-02.svg",
    availability: Availability::LOCAL,
    auto_enter_ai_mode: false,
    argument: Some(
        Argument::optional().with_hint_text("<path/to/file[:line[:col]]> or \"@\" to search"),
    ),
});

pub static RENAME_TAB: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/rename-tab",
    description_key: "command-slash-rename-tab-desc",
    icon_path: "bundled/svg/pencil-line.svg",
    availability: Availability::ALWAYS,
    auto_enter_ai_mode: false,
    argument: Some(Argument::required().with_hint_text("<tab name>")),
});

pub static RENAME_CONVERSATION: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/rename-conversation",
    description_key: "command-slash-rename-conversation-desc",
    icon_path: "bundled/svg/pencil-line.svg",
    availability: Availability::AGENT_VIEW
        | Availability::ACTIVE_CONVERSATION
        | Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: Some(Argument::required().with_hint_text("<new title>")),
});

static SET_TAB_COLOR_HINT: LazyLock<String> = LazyLock::new(|| {
    let mut hint = String::from("<");
    for color in color_dot::TAB_COLOR_OPTIONS {
        hint.push_str(&color.to_string().to_ascii_lowercase());
        hint.push('|');
    }
    hint.push_str("none>");
    hint
});

pub static SET_TAB_COLOR: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/set-tab-color",
    description_key: "command-slash-set-tab-color-desc",
    icon_path: "bundled/svg/ellipse.svg",
    availability: Availability::ALWAYS,
    auto_enter_ai_mode: false,
    argument: Some(Argument::required().with_hint_text(SET_TAB_COLOR_HINT.as_str())),
});

pub static FORK: LazyLock<StaticCommand> = LazyLock::new(|| {
    let hint_text = "<optional prompt to send in forked conversation>";
    StaticCommand {
        name: "/fork",
        description_key: "command-slash-fork-desc",
        icon_path: "bundled/svg/arrow-split.svg",
        availability: Availability::AGENT_VIEW
            | Availability::ACTIVE_CONVERSATION
            | Availability::NO_LRC_CONTROL
            | Availability::AI_ENABLED,
        auto_enter_ai_mode: true,
        argument: Some(Argument::optional().with_hint_text(hint_text)),
    }
});

pub static MOVE_TO_CLOUD: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/handoff",
    description_key: "command-slash-handoff-desc",
    icon_path: "bundled/svg/upload-cloud-01.svg",
    availability: Availability::AGENT_VIEW
        | Availability::ACTIVE_CONVERSATION
        | Availability::AI_ENABLED
        | Availability::NOT_CLOUD_AGENT,
    auto_enter_ai_mode: false,
    argument: Some(
        Argument::optional()
            .with_hint_text("<optional follow-up prompt>")
            .with_execute_on_selection(),
    ),
});

pub const OPEN_CODE_REVIEW: StaticCommand = StaticCommand {
    name: "/open-code-review",
    description_key: "command-slash-open-code-review-desc",
    icon_path: "bundled/svg/diff.svg",
    availability: Availability::REPOSITORY,
    auto_enter_ai_mode: false,
    argument: None,
};

pub const INDEX: StaticCommand = StaticCommand {
    name: "/index",
    description_key: "command-slash-index-desc",
    icon_path: "bundled/svg/find-all.svg",
    availability: Availability::REPOSITORY
        .union(Availability::CODEBASE_CONTEXT)
        .union(Availability::AI_ENABLED),
    auto_enter_ai_mode: false,
    argument: None,
};

pub const INIT: StaticCommand = StaticCommand {
    name: "/init",
    description_key: "command-slash-init-desc",
    icon_path: "bundled/svg/warp-2.svg",
    availability: Availability::REPOSITORY
        .union(Availability::AGENT_VIEW)
        .union(Availability::AI_ENABLED),
    auto_enter_ai_mode: true,
    argument: None,
};

pub const OPEN_PROJECT_RULES: StaticCommand = StaticCommand {
    name: "/open-project-rules",
    description_key: "command-slash-open-project-rules-desc",
    icon_path: "bundled/svg/file-code-02.svg",
    availability: Availability::REPOSITORY.union(Availability::AI_ENABLED),
    auto_enter_ai_mode: false,
    argument: None,
};

pub const OPEN_MCP_SERVERS: StaticCommand = StaticCommand {
    name: "/open-mcp-servers",
    description_key: "command-slash-open-mcp-servers-desc",
    icon_path: "bundled/svg/dataflow.svg",
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: None,
};

pub const OPEN_SETTINGS_FILE: StaticCommand = StaticCommand {
    name: "/open-settings-file",
    description_key: "command-slash-open-settings-file-desc",
    icon_path: "bundled/svg/file-code-02.svg",
    availability: Availability::LOCAL,
    auto_enter_ai_mode: false,
    argument: None,
};

pub const CHANGELOG: StaticCommand = StaticCommand {
    name: "/changelog",
    description_key: "command-slash-changelog-desc",
    icon_path: "bundled/svg/book-open.svg",
    availability: Availability::ALWAYS,
    auto_enter_ai_mode: false,
    argument: None,
};

// Accepts an optional argument so that buffers like `/feedback some text` still parse to
// this command (the trailing text is ignored on execution). Without this, typing any
// argument after `/feedback` would fall through and be treated as plain input.
pub static FEEDBACK: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/feedback",
    description_key: "command-slash-feedback-desc",
    icon_path: "bundled/svg/feedback.svg",
    availability: Availability::ALWAYS,
    auto_enter_ai_mode: false,
    argument: Some(Argument::optional().with_execute_on_selection()),
});

pub const OPEN_REPO: StaticCommand = StaticCommand {
    name: "/open-repo",
    description_key: "command-slash-open-repo-desc",
    icon_path: "bundled/svg/folder.svg",
    availability: Availability::LOCAL.union(Availability::AI_ENABLED),
    auto_enter_ai_mode: false,
    argument: None,
};

pub const OPEN_RULES: StaticCommand = StaticCommand {
    name: "/open-rules",
    description_key: "command-slash-open-rules-desc",
    icon_path: "bundled/svg/book-open.svg",
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: None,
};

pub static NEW: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/new",
    description_key: "command-slash-new-desc",
    icon_path: "bundled/svg/new-conversation.svg",
    availability: Availability::NO_LRC_CONTROL
        | Availability::AI_ENABLED
        | Availability::NOT_CLOUD_AGENT,
    auto_enter_ai_mode: false,
    argument: Some(Argument::optional().with_execute_on_selection()),
});

pub static MODEL: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/model",
    description_key: "command-slash-model-desc",
    icon_path: "bundled/svg/oz.svg",
    availability: Availability::AGENT_VIEW | Availability::AI_ENABLED,
    auto_enter_ai_mode: true,
    argument: None,
});

pub static HOST: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/host",
    description_key: "command-slash-host-desc",
    icon_path: "bundled/svg/oz-cloud.svg",
    availability: Availability::AGENT_VIEW
        | Availability::AI_ENABLED
        | Availability::CLOUD_MODE_V2_COMPOSER,
    auto_enter_ai_mode: true,
    argument: None,
});

pub static HARNESS: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/harness",
    description_key: "command-slash-harness-desc",
    icon_path: "bundled/svg/oz.svg",
    availability: Availability::AGENT_VIEW
        | Availability::AI_ENABLED
        | Availability::CLOUD_MODE_V2_COMPOSER,
    auto_enter_ai_mode: true,
    argument: None,
});

pub static ENVIRONMENT: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/environment",
    description_key: "command-slash-environment-desc",
    icon_path: "bundled/svg/globe-04.svg",
    availability: Availability::AGENT_VIEW
        | Availability::AI_ENABLED
        | Availability::CLOUD_MODE_V2_COMPOSER,
    auto_enter_ai_mode: true,
    argument: None,
});

pub static PROFILE: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/profile",
    description_key: "command-slash-profile-desc",
    icon_path: "bundled/svg/psychology.svg",
    availability: Availability::AGENT_VIEW
        | Availability::AI_ENABLED
        | Availability::NOT_CLOUD_AGENT,
    auto_enter_ai_mode: true,
    argument: None,
});

pub const PLAN_NAME: &str = "/plan";

pub static PLAN: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: PLAN_NAME,
    description_key: "command-slash-plan-desc",
    icon_path: "bundled/svg/file-06.svg",
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: true,
    argument: Some(Argument::optional().with_hint_text("<describe your task>")),
});

pub const ORCHESTRATE_NAME: &str = "/orchestrate";

pub static ORCHESTRATE: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: ORCHESTRATE_NAME,
    description_key: "command-slash-orchestrate-desc",
    icon_path: "bundled/svg/oz.svg",
    availability: Availability::LOCAL | Availability::AI_ENABLED,
    auto_enter_ai_mode: true,
    argument: Some(Argument::optional().with_hint_text("<describe your task>")),
});

/// If `query` starts with the given command `name` followed by a space,
/// returns the remainder of the query. Otherwise returns `None`.
pub fn strip_command_prefix(query: &str, name: &str) -> Option<String> {
    query
        .strip_prefix(name)
        .and_then(|rest| rest.strip_prefix(' '))
        .map(|rest| rest.to_string())
}

pub static COMPACT: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/compact",
    description_key: "command-slash-compact-desc",
    icon_path: "bundled/svg/collapse_content.svg",
    availability: Availability::AGENT_VIEW
        | Availability::ACTIVE_CONVERSATION
        | Availability::NO_LRC_CONTROL
        | Availability::AI_ENABLED
        | Availability::NOT_CLOUD_AGENT,
    auto_enter_ai_mode: true,
    argument: Some(
        Argument::optional().with_hint_text("<optional custom summarization instructions>"),
    ),
});

pub static COMPACT_AND: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/compact-and",
    description_key: "command-slash-compact-and-desc",
    icon_path: "bundled/svg/collapse_content.svg",
    availability: Availability::AGENT_VIEW
        | Availability::ACTIVE_CONVERSATION
        | Availability::NO_LRC_CONTROL
        | Availability::AI_ENABLED
        | Availability::NOT_CLOUD_AGENT,
    auto_enter_ai_mode: true,
    argument: Some(Argument::optional().with_hint_text("<prompt to send after compaction>")),
});

pub static QUEUE: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/queue",
    description_key: "command-slash-queue-desc",
    icon_path: "bundled/svg/clock-plus.svg",
    availability: Availability::AGENT_VIEW
        | Availability::ACTIVE_CONVERSATION
        | Availability::AI_ENABLED
        | Availability::NOT_CLOUD_AGENT,
    auto_enter_ai_mode: true,
    argument: Some(Argument::required().with_hint_text("<prompt to send when agent is done>")),
});

pub static FORK_AND_COMPACT: LazyLock<StaticCommand> = LazyLock::new(|| {
    let hint_text = "<optional prompt to send after compaction>";
    StaticCommand {
        name: "/fork-and-compact",
        description_key: "command-slash-fork-and-compact-desc",
        icon_path: "bundled/svg/fork_and_compact.svg",
        availability: Availability::AGENT_VIEW
            | Availability::ACTIVE_CONVERSATION
            | Availability::NO_LRC_CONTROL
            | Availability::AI_ENABLED
            | Availability::NOT_CLOUD_AGENT,
        auto_enter_ai_mode: true,
        argument: Some(Argument::optional().with_hint_text(hint_text)),
    }
});

pub const FORK_FROM: StaticCommand = StaticCommand {
    name: "/fork-from",
    description_key: "command-slash-fork-from-desc",
    icon_path: "bundled/svg/arrow-split.svg",
    availability: Availability::AGENT_VIEW
        .union(Availability::NO_LRC_CONTROL)
        .union(Availability::AI_ENABLED)
        .union(Availability::NOT_CLOUD_AGENT),
    auto_enter_ai_mode: true,
    argument: None,
};

pub static CONTINUE_LOCALLY: LazyLock<StaticCommand> = LazyLock::new(|| {
    let hint_text = "<optional prompt to send in local conversation>";
    StaticCommand {
        name: "/continue-locally",
        description_key: "command-slash-continue-locally-desc",
        icon_path: "bundled/svg/arrow-split.svg",
        availability: Availability::AGENT_VIEW
            | Availability::ACTIVE_CONVERSATION
            | Availability::AI_ENABLED
            | Availability::CLOUD_AGENT,
        auto_enter_ai_mode: true,
        argument: Some(Argument::optional().with_hint_text(hint_text)),
    }
});

pub const USAGE: StaticCommand = StaticCommand {
    name: "/usage",
    description_key: "command-slash-usage-desc",
    icon_path: "bundled/svg/bar-chart-04.svg",
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: None,
};

pub const REMOTE_CONTROL: StaticCommand = StaticCommand {
    name: "/remote-control",
    description_key: "command-slash-remote-control-desc",
    icon_path: "bundled/svg/phone-01.svg",
    availability: Availability::AI_ENABLED.union(Availability::NOT_CLOUD_AGENT),
    auto_enter_ai_mode: false,
    argument: None,
};

pub const COST: StaticCommand = StaticCommand {
    name: "/cost",
    description_key: "command-slash-cost-desc",
    icon_path: "bundled/svg/bar-chart-04.svg",
    availability: Availability::AGENT_VIEW
        .union(Availability::AI_ENABLED)
        .union(Availability::NOT_CLOUD_AGENT),
    auto_enter_ai_mode: false,
    argument: None,
};

pub const CONVERSATIONS: StaticCommand = StaticCommand {
    name: "/conversations",
    description_key: "command-slash-conversations-desc",
    icon_path: "bundled/svg/conversation.svg",
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: None,
};

pub static PROMPTS: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/prompts",
    description_key: "command-slash-prompts-desc",
    icon_path: "bundled/svg/prompt.svg",
    availability: Availability::AI_ENABLED,
    auto_enter_ai_mode: false,
    argument: None,
});

pub const REWIND: StaticCommand = StaticCommand {
    name: "/rewind",
    description_key: "command-slash-rewind-desc",
    icon_path: "bundled/svg/clock-rewind.svg",
    availability: Availability::AGENT_VIEW
        .union(Availability::AI_ENABLED)
        .union(Availability::NOT_CLOUD_AGENT),
    auto_enter_ai_mode: true,
    argument: None,
};

pub const EXPORT_TO_CLIPBOARD: StaticCommand = StaticCommand {
    name: "/export-to-clipboard",
    description_key: "command-slash-export-to-clipboard-desc",
    icon_path: "bundled/svg/copy.svg",
    availability: Availability::AGENT_VIEW
        .union(Availability::AI_ENABLED)
        .union(Availability::NOT_CLOUD_AGENT),
    auto_enter_ai_mode: true,
    argument: None,
};

pub static EXPORT_TO_FILE: LazyLock<StaticCommand> = LazyLock::new(|| StaticCommand {
    name: "/export-to-file",
    description_key: "command-slash-export-to-file-desc",
    icon_path: "bundled/svg/download-01.svg",
    availability: Availability::AGENT_VIEW
        | Availability::AI_ENABLED
        | Availability::NOT_CLOUD_AGENT,
    auto_enter_ai_mode: true,
    argument: Some(Argument::optional().with_hint_text("<optional filename>")),
});

pub static COMMAND_REGISTRY: LazyLock<Registry> = LazyLock::new(Registry::new);

/// A unique identifier for a static slash command.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct SlashCommandId(Uuid);

impl SlashCommandId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SlashCommandId {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Registry {
    commands: HashMap<SlashCommandId, StaticCommand>,
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Self {
        let mut commands = HashMap::new();
        for command in all_commands().into_iter() {
            debug_assert!(
                !command
                    .availability
                    .contains(Availability::TERMINAL_VIEW | Availability::AGENT_VIEW),
                "command `{}` sets both TERMINAL_VIEW and AGENT_VIEW, which is unsatisfiable",
                command.name,
            );
            commands.insert(SlashCommandId::new(), command);
        }
        Self { commands }
    }

    pub fn all_commands_by_id(&self) -> impl Iterator<Item = (SlashCommandId, &StaticCommand)> {
        self.commands.iter().map(|(id, cmd)| (*id, cmd))
    }

    pub fn all_commands(&self) -> impl Iterator<Item = &StaticCommand> {
        self.commands.values()
    }

    pub fn get_command(&self, id: &SlashCommandId) -> Option<&StaticCommand> {
        self.commands.get(id)
    }

    pub fn get_command_with_name(&self, name: &str) -> Option<&StaticCommand> {
        self.commands.values().find(|command| command.name == name)
    }

    #[cfg(test)]
    pub fn get_command_id_with_name(&self, name: &str) -> Option<&SlashCommandId> {
        self.commands
            .iter()
            .find(|(_, command)| command.name == name)
            .map(|(id, _)| id)
    }
}

fn all_commands() -> Vec<StaticCommand> {
    let mut commands = vec![
        ADD_MCP,
        ADD_PROMPT.clone(),
        ADD_RULE,
        COST,
        FEEDBACK.clone(),
        INDEX,
        INIT,
        OPEN_PROJECT_RULES,
        OPEN_MCP_SERVERS,
        OPEN_RULES,
        AGENT.clone(),
        NEW.clone(),
        PLAN.clone(),
        RENAME_CONVERSATION.clone(),
        RENAME_TAB.clone(),
        SET_TAB_COLOR.clone(),
        USAGE,
        CONVERSATIONS,
        EXPORT_TO_CLIPBOARD,
        MODEL.clone(),
    ];

    if FeatureFlag::LocalDockerSandbox.is_enabled() {
        commands.push(CREATE_DOCKER_SANDBOX);
    }

    if FeatureFlag::CreatingSharedSessions.is_enabled()
        && FeatureFlag::HOARemoteControl.is_enabled()
    {
        commands.push(REMOTE_CONTROL);
    }

    if FeatureFlag::Changelog.is_enabled() {
        commands.push(CHANGELOG);
    }

    if FeatureFlag::AgentView.is_enabled() {
        commands.push(PROMPTS.clone());
    }

    commands.push(OPEN_CODE_REVIEW);

    if FeatureFlag::CreateEnvironmentSlashCommand.is_enabled() {
        commands.push(CREATE_ENVIRONMENT.clone());
    }

    if FeatureFlag::CreateProjectFlow.is_enabled() {
        commands.push(CREATE_NEW_PROJECT.clone());
    }

    if FeatureFlag::SummarizationConversationCommand.is_enabled() {
        commands.push(COMPACT.clone());
        commands.push(COMPACT_AND.clone());
    }

    if FeatureFlag::QueueSlashCommand.is_enabled() {
        commands.push(QUEUE.clone());
    }

    if !cfg!(target_family = "wasm") {
        commands.extend([
            FORK.clone(),
            FORK_AND_COMPACT.clone(),
            CONTINUE_LOCALLY.clone(),
        ]);

        if FeatureFlag::ForkFromCommand.is_enabled() {
            commands.push(FORK_FROM);
        }
    }

    if !cfg!(target_family = "wasm") {
        commands.extend([EDIT.clone(), EXPORT_TO_FILE.clone()]);
    }

    if FeatureFlag::ListSkills.is_enabled() && !cfg!(target_family = "wasm") {
        commands.push(EDIT_SKILL.clone());
        commands.push(INVOKE_SKILL.clone());
    }

    if FeatureFlag::PRCommentsSlashCommand.is_enabled()
        && !FeatureFlag::PRCommentsSkill.is_enabled()
    {
        commands.push(PR_COMMENTS);
    }

    if FeatureFlag::CloudMode.is_enabled() && FeatureFlag::CloudModeFromLocalSession.is_enabled() {
        commands.push(CLOUD_AGENT.clone());
    }

    if FeatureFlag::OzHandoff.is_enabled()
        && FeatureFlag::HandoffLocalCloud.is_enabled()
        && cfg!(all(feature = "local_fs", not(target_family = "wasm")))
    {
        commands.push(MOVE_TO_CLOUD.clone());
    }

    if FeatureFlag::InlineProfileSelector.is_enabled() {
        commands.push(PROFILE.clone());
    }

    if FeatureFlag::RevertToCheckpoints.is_enabled() && FeatureFlag::RewindSlashCommand.is_enabled()
    {
        commands.push(REWIND);
    }

    if FeatureFlag::InlineRepoMenu.is_enabled() && !cfg!(target_family = "wasm") {
        commands.push(OPEN_REPO);
    }

    commands.push(ORCHESTRATE.clone());

    if FeatureFlag::SettingsFile.is_enabled() && cfg!(feature = "local_fs") {
        commands.push(OPEN_SETTINGS_FILE);
    }

    if FeatureFlag::CloudModeInputV2.is_enabled() {
        commands.push(HOST.clone());
        commands.push(HARNESS.clone());
        commands.push(ENVIRONMENT.clone());
    }

    commands
}

#[cfg(test)]
#[path = "commands_tests.rs"]
mod tests;
