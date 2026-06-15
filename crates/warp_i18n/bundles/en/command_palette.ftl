# Command palette: command display names and short descriptions.
# Keys MUST start with `command-`.

# Tab / window lifecycle.
command-new-tab = Create new tab
command-new-terminal-tab = New Terminal Tab
command-new-agent-tab = New Agent Tab
command-new-cloud-agent-tab = New Cloud Agent Tab
command-close-window = Close Window
command-move-tab-left = Move tab left
command-move-tab-right = Move tab right
command-close-tabs-right = Close tabs to the right

# Side panels and palette toggles.
command-open-left-panel = Open Left Panel
command-close-focused-panel = Close focused panel
command-toggle-project-explorer = Toggle project explorer
command-toggle-vertical-tabs-panel = Toggle vertical tabs panel
command-toggle-code-review = Toggle code review
command-toggle-warp-drive = Toggle Warp Drive
command-toggle-agent-conversation-list = Toggle Agent conversation list view
command-toggle-command-palette = Toggle command palette
command-toggle-navigation-palette = Toggle navigation palette
command-open-global-search = Open global search
command-left-panel-agent-conversations = Left Panel: Agent conversations
command-left-panel-project-explorer = Left Panel: Project explorer
command-left-panel-global-search = Left Panel: Global search
command-left-panel-warp-drive = Left Panel: Warp Drive

# Drive: notebooks / workflows / folders / prompts / env vars.
command-new-team-notebook = Create a new team notebook
command-new-personal-notebook = Create a new personal notebook
command-new-team-workflow = Create a new team workflow
command-new-personal-workflow = Create a new personal workflow
command-new-team-folder = Create a new team folder
command-new-personal-folder = Create a new personal folder
command-new-team-env-vars = Create new team environment variables
command-new-personal-env-vars = Create new personal environment variables
command-new-personal-prompt = Create a new personal prompt
command-new-team-prompt = Create a new team prompt

# Repository / configuration.
command-open-repository = Open repository
command-open-ai-rules = Open AI Rules
command-open-mcp-servers = Open MCP Servers

# Settings entry points.
command-open-settings = Open Settings
command-open-settings-appearance = Open Settings: Appearance
command-open-settings-shared-blocks = Open Settings: Shared Blocks
command-open-settings-keyboard-shortcuts = Open Settings: Keyboard Shortcuts
command-open-settings-about = Open Settings: About
command-open-settings-teams = Open Settings: Teams
command-open-settings-privacy = Open Settings: Privacy
command-open-settings-warpify = Open Settings: Warpify
command-open-settings-ai = Open Settings: AI
command-open-settings-billing = Open Settings: Billing and usage
command-open-settings-code = Open Settings: Code
command-open-settings-referrals = Open Settings: Referrals
command-open-settings-environments = Open Settings: Environments
command-open-settings-mcp-servers = Open Settings: MCP Servers

# Feedback / external.
command-send-feedback = Send feedback (opens external link)
command-send-feedback-with-oz = Send feedback with Oz

# Terminal-side commands.
command-edit-prompt = Edit Prompt
command-attach-block-as-agent-context = Attach Selected Block as Agent Context
command-attach-text-as-agent-context = Attach Selected Text as Agent Context
command-write-codebase-index-snapshot = Write current codebase index snapshot
command-initiate-project = Initiate project for warp
command-add-folder-as-project = Add current folder as project

# macOS menu bar short forms (MAC_MENUS_CONTEXT). Often title-cased and shorter
# than the command palette descriptions; live alongside their long-form siblings.
command-project-explorer-mac = Project Explorer
command-new-team-notebook-mac = New Team Notebook
command-new-personal-notebook-mac = New Personal Notebook
command-new-team-workflow-mac = New Team Workflow
command-new-personal-workflow-mac = New Personal Workflow
command-new-team-folder-mac = New Team Folder
command-new-personal-folder-mac = New Personal Folder
command-toggle-code-review-mac = Toggle Code Review
command-toggle-vertical-tabs-panel-mac = Toggle Vertical Tabs Panel
command-global-search-mac = Global Search
command-warp-drive-mac = Warp Drive
command-agent-conversation-list-mac = Agent conversation list view
command-close-focused-panel-mac = Close focused panel
command-command-palette-mac = Command Palette
command-close-window-mac = Close Window
command-navigation-palette-mac = Navigation Palette
command-new-team-env-vars-mac = New Team Environment Variables
command-new-personal-env-vars-mac = New Personal Environment Variables
command-new-personal-prompt-mac = New Personal Prompt
command-new-team-prompt-mac = New Team Prompt
command-open-repository-mac = Open Repository
command-open-ai-rules-mac = Open AI Rules
command-open-mcp-servers-mac = Open MCP Servers
command-settings-mac = Settings
command-appearance-mac = Appearance...
command-view-shared-blocks-mac = View Shared Blocks...
command-configure-keyboard-shortcuts-mac = Configure Keyboard Shortcuts...
command-about-warp-mac = About Warp
command-open-team-settings-mac = Open Team Settings
command-configure-warpify-mac = Configure Warpify...
command-attach-selection-as-agent-context-mac = Attach Selection as Agent Context

# Static slash command descriptions.
command-slash-agent-desc = Start a new conversation
command-slash-cloud-agent-desc = Start a new cloud agent conversation
command-slash-add-mcp-desc = Add a new MCP server via the MCP settings page
command-slash-pr-comments-desc = Pull GitHub PR review comments
command-slash-create-environment-desc = Create an Oz environment (Docker image + repos) via guided setup
command-slash-docker-sandbox-desc = Create a new docker sandbox terminal session
command-slash-create-new-project-desc = Have Oz walk you through creating a new coding project
command-slash-open-skill-desc = Open a skill's markdown file in Warp's built-in editor
command-slash-skills-desc = Invoke a skill
command-slash-add-prompt-desc = Add new Agent prompt
command-slash-add-rule-desc = Add a new global rule for the agent
command-slash-open-file-desc = Open a file in Warp's code editor
command-slash-rename-tab-desc = Rename the current tab
command-slash-rename-conversation-desc = Rename the current conversation
command-slash-set-tab-color-desc = Set the color of the current tab
command-slash-fork-desc = Fork the current conversation in a new pane or a new tab
command-slash-open-code-review-desc = Open code review
command-slash-index-desc = Index this codebase
command-slash-init-desc = Index this codebase and generate an AGENTS.md file
command-slash-open-project-rules-desc = Open the project rules file (AGENTS.md)
command-slash-open-mcp-servers-desc = Open MCP servers
command-slash-open-settings-file-desc = Open settings file (TOML)
command-slash-changelog-desc = Open the latest changelog
command-slash-feedback-desc = Send feedback
command-slash-open-repo-desc = Switch to another indexed repository
command-slash-open-rules-desc = View all of your global and project rules
command-slash-new-desc = Start a new conversation (alias for /agent)
command-slash-model-desc = Switch the base agent model
command-slash-host-desc = Switch the cloud agent execution host
command-slash-harness-desc = Switch the cloud agent harness
command-slash-environment-desc = Switch the cloud agent environment
command-slash-profile-desc = Switch the active execution profile
command-slash-plan-desc = Prompt the agent to do some research and create a plan for a task
command-slash-orchestrate-desc = Break a task into subtasks and run them in parallel with multiple agents
command-slash-compact-desc = Free up context by summarizing convo history
command-slash-compact-and-desc = Compact conversation and then send a follow-up prompt
command-slash-queue-desc = Queue a prompt to send after the agent finishes responding
command-slash-fork-and-compact-desc = Fork current conversation and compact it in the forked copy
command-slash-fork-from-desc = Fork conversation from a specific query
command-slash-continue-locally-desc = Continue this cloud conversation locally
command-slash-usage-desc = Open billing and usage settings
command-slash-remote-control-desc = Start remote control for this session
command-slash-cost-desc = Toggle credit usage details
command-slash-conversations-desc = Open conversation history
command-slash-prompts-desc = Search saved prompts
command-slash-rewind-desc = Rewind to a previous point in the conversation
command-slash-export-to-clipboard-desc = Export current conversation to clipboard in markdown format
command-slash-export-to-file-desc = Export current conversation to a markdown file
command-slash-handoff-desc = Hand off this conversation to a cloud agent
