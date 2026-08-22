# AI / Agent UI wrapper strings (non-model-response user-facing text).
# Keys MUST start with `ai-ui-`.

ai-ui-thinking = Thinking...
ai-ui-generating = Generating...
ai-ui-cancelled = Cancelled
ai-ui-error-occurred = An error occurred
ai-ui-retry = Retry
ai-ui-accept = Accept
ai-ui-reject = Reject

# Suggestion chip tooltips
ai-ui-tooltip-add-rule = Add rule: { $content }
ai-ui-tooltip-suggested-prompt =
    Suggested prompt:
    { $prompt }

# Todos popup
ai-ui-tasks-header = Tasks

# Agent type selector
ai-ui-choose-agent = Choose your agent
ai-ui-suggested-badge = Suggested

# Cloud setup guide
ai-ui-cloud-setup-title = Getting started with Oz cloud agents
ai-ui-cloud-setup-intro = Start Oz cloud agents directly in Warp from an integration (Linear, Slack), with an event (GitHub, built-in schedule), or programmatically with the Oz SDK or CLI.
ai-ui-cloud-setup-manual-section = Manual setup: Create a Slack or Linear integration with the Oz CLI
ai-ui-cloud-setup-create-env = Create an environment
ai-ui-cloud-setup-env-first = First, set up an environment to create an integration.
ai-ui-cloud-setup-custom-image = Or, supply your own existing docker image.
ai-ui-cloud-setup-create-integration = Create an integration

# Agent input footer
ai-ui-using-default-model = Now using Full Terminal Agent's default model.
ai-ui-enable-terminal-command-autodetection = Enable terminal command autodetection
ai-ui-disable-terminal-command-autodetection = Disable terminal command autodetection
ai-ui-voice-input = Voice input
ai-ui-attach-file = Attach file
ai-ui-auto-approve-on = Turn off auto-approve all agent actions
ai-ui-auto-approve-off = Auto-approve all agent actions for this task
ai-ui-auto-approve-locked = Fast forward is always enabled for cloud agent conversations
ai-ui-file-explorer = File explorer
ai-ui-open-file-explorer = Open file explorer
ai-ui-rich-input = Rich Input
ai-ui-open-rich-input = Open Rich Input
ai-ui-hide-rich-input = Hide Rich Input
ai-ui-open-coding-agent-settings = Open coding agent settings
ai-ui-enable-notifications = Enable notifications
ai-ui-enable-agent-notifications = Enable { $agent } notifications
ai-ui-install-plugin-tooltip = Install the Warp plugin to enable rich agent notifications within Warp
ai-ui-notifications-setup-instructions = Notifications setup instructions
ai-ui-plugin-install-instructions-tooltip = View instructions to install the Warp plugin
ai-ui-update-warp-plugin = Update Warp plugin
ai-ui-update-plugin-tooltip = A new version of the Warp plugin is available
ai-ui-plugin-update-instructions = Plugin update instructions
ai-ui-plugin-update-instructions-tooltip = View instructions to update the Warp plugin
ai-ui-dismiss = Dismiss
ai-ui-start-remote-control = Start remote control
ai-ui-start-remote-control-login-required = Log in to use /remote-control
ai-ui-stop-sharing = Stop sharing
ai-ui-context-window-usage = Context window usage
ai-ui-choose-environment = Choose an environment
ai-ui-show-version-history = Show version history
ai-ui-update-agent = Update Agent
ai-ui-update-agent-tooltip = This plan has changes the agent isn't aware of. { $shortcut } to stop the agent's current task and send the updated plan
ai-ui-restore = Restore
ai-ui-plan-save-sync-tooltip = Save and auto-sync this plan to your Warp Drive
ai-ui-show-in-warp-drive = Show in Warp Drive
ai-ui-save-markdown-file = Save as markdown file
ai-ui-attach-active-session = Attach to active session
ai-ui-copy-plan-id = Copy plan ID
ai-ui-attach-context = Attach context
ai-ui-slash-commands = Slash commands
ai-ui-at-context-no-objects = No available objects in the current context.
ai-ui-at-context-ssh = Not supported in SSH sessions without remote server
ai-ui-at-context-subshell = Not supported in subshells
ai-ui-at-context-filesystem-required = Requires a filesystem
ai-ui-at-context-disabled-terminal-mode = Disabled in terminal mode, re-enable in settings
ai-ui-choose-execution-profile = Choose an AI execution profile
ai-ui-choose-agent-model = Choose an agent model
ai-ui-manage = Manage
ai-ui-manage-api-keys = Manage API keys
ai-ui-profiles = Profiles
ai-ui-manage-profiles = Manage profiles
ai-ui-open-github = Open in GitHub
ai-ui-open-code-review = Open in code review
ai-ui-manage-rules = Manage rules
ai-ui-review-changes = Review changes
ai-ui-open-all-code-review = Open all in code review
ai-ui-dont-show-again = Don't show again
ai-ui-rewind = Rewind
ai-ui-rewind-before-block = Rewind to before this block
ai-ui-learn-more = Learn more

# Agent tips
ai-ui-tip-prefix = Tip:
ai-ui-agent-tip-slash-menu = `/` to open the slash-command menu and access quick agent actions.
ai-ui-agent-tip-toggle-input-mode = <keybinding> to toggle natural language detection and switch between agent and terminal input.
ai-ui-agent-tip-plan = `/plan` <prompt> to create a plan for the agent before executing.
ai-ui-agent-tip-command-palette = <keybinding> to open the Command Palette and access Warp actions and shortcuts.
ai-ui-agent-tip-warp-drive = Store reusable workflows, notebooks, and prompts in your
ai-ui-agent-tip-redirect-agent = Enter a new prompt to redirect the agent while it's running.
ai-ui-agent-tip-at-context = `@` to add context from files, blocks, or Warp Drive objects to your prompt.
ai-ui-agent-tip-prior-output-context = <keybinding> to attach the prior command output as agent context.
ai-ui-agent-tip-init-index = `/init` to index the repo so the agent can understand your codebase.
ai-ui-agent-tip-agent-profiles = Add agent profiles to customize permissions and models per session.
ai-ui-agent-tip-fork-block = Right-click a block to fork the conversation from that point.
ai-ui-agent-tip-copy-block-output = Right-click a block to copy a conversation's output.
ai-ui-agent-tip-drag-image = Drag an image into the pane to attach it as agent context.
ai-ui-agent-tip-interactive-tools = Prompt the agent to control interactive tools like node, python, postgres, gdb, or vim.
ai-ui-agent-tip-code-review-panel = <keybinding> to open the code review panel and review the agent's changes.
ai-ui-agent-tip-add-mcp = `/add-mcp` to add an MCP server to your workspace.
ai-ui-agent-tip-open-mcp-servers = `/open-mcp-servers` to view and share MCP servers with your team.
ai-ui-agent-tip-create-environment = `/create-environment` to turn a repo into a remote Docker environment an agent can run in.
ai-ui-agent-tip-add-prompt = `/add-prompt` to create a reusable prompt for repeatable workflows.
ai-ui-agent-tip-add-rule = `/add-rule` to create a global agent rule.
ai-ui-agent-tip-fork-command = `/fork` to create a fresh copy of the current conversation, optionally with a new prompt.
ai-ui-agent-tip-open-code-review = `/open-code-review` to open the code review panel and inspect agent-generated diffs.
ai-ui-agent-tip-new-conversation = `/new` to start a new agent conversation with clean context.
ai-ui-agent-tip-compact = `/compact` to summarize the current conversation and free up space in the context window.
ai-ui-agent-tip-usage = `/usage` to show your current AI credits usage.
ai-ui-agent-tip-oz-headless = Use the `oz` command to run an Oz agent in headless mode, useful for remote machines.
ai-ui-agent-tip-selected-text-context = Right-click selected text to attach it as agent context.
ai-ui-agent-tip-project-rules = Use `AGENTS.md` or `CLAUDE.md` to apply project-scoped rules.
ai-ui-agent-tip-url-context = Paste a URL to attach that webpage as context for the agent.
ai-ui-agent-tip-warpify-ssh = Warpify a remote SSH session to enable Oz inside that environment.
ai-ui-agent-tip-switch-profiles = Switch agent profiles to quickly change models and agent permissions.
ai-ui-agent-tip-init-rules = `/init` to generate a `WARP.md` file and define project rules for the agent.
ai-ui-agent-tip-auto-approve-session = <keybinding> to auto-approve the agent's commands and diffs for the rest of the session.
ai-ui-agent-tip-handoff = Type `&` or use the handoff chip to move a local conversation to the cloud.
ai-ui-agent-tip-desktop-notifications = Enable desktop notifications to get an alert when an agent needs your attention.
ai-ui-agent-tip-cancel-task = <keybinding> to cancel the current agent task.
ai-ui-agent-tip-voice-input = Hold <keybinding> to speak your prompt directly to the agent.
ai-ui-agent-tip-action-open-palette = Open palette
ai-ui-agent-tip-action-warp-drive = Warp Drive.
ai-ui-agent-tip-action-show-diff-view = Show diff view

# Agent view shortcuts
ai-ui-agent-shortcut-shell-command = input shell command
ai-ui-agent-shortcut-slash-commands = for slash commands
ai-ui-agent-shortcut-context = for file paths and attaching other context
ai-ui-agent-shortcut-code-review = open code review
ai-ui-agent-shortcut-toggle-conversation-list = toggle conversation list
ai-ui-agent-shortcut-search-conversations = search and continue conversations
ai-ui-agent-shortcut-new-conversation = start a new conversation
ai-ui-agent-shortcut-toggle-auto-accept = toggle auto-accept
ai-ui-agent-shortcut-pause-agent = pause agent
ai-ui-agent-shortcut-back-to-terminal = go back to terminal

# Zero state block
ai-ui-zero-isolated-cloud = Run your agent task in an isolated cloud environment.
ai-ui-zero-recent-activity = RECENT ACTIVITY
ai-ui-view-changelog = View changelog

# Common AI badges and statuses
ai-ui-recommended = Recommended
ai-ui-queued = Queued
ai-ui-check-now-suffix = { " · " }Check now
ai-ui-invalid-api-key = Provided API key is not valid
ai-ui-debug-output = Debug output

# AWS Bedrock credentials
ai-ui-aws-creds-error = AWS credentials expired or missing
ai-ui-always-auto-run = Always run automatically

# Code diff view
ai-ui-file-renamed-no-change = File renamed without changes

# Requested command
ai-ui-permission-always-ask = Your profile is set to always ask for permission to execute commands.

# Conversation details panel
ai-ui-conversation-error = Error
ai-ui-conversation-status = Status
ai-ui-conversation-harness = Harness
ai-ui-conversation-artifacts = Artifacts
ai-ui-conversation-env-setup = Environment setup commands
ai-ui-conversation-env-details = Environment details
ai-ui-conversation-credits-used = Credits used

# Execution profile editor
ai-ui-profile-name = Name
ai-ui-plan-auto-sync = Plan auto-sync
ai-ui-plan-auto-sync-desc = The plans this agent creates will be automatically added and synced to Warp Drive.
ai-ui-call-web-tools = Call web tools
ai-ui-call-web-tools-desc = The agent may use web search when helpful for completing tasks.
ai-ui-context-window-label = Context window
ai-ui-context-window-desc = The base model's working memory — how many tokens of your conversation, code, and documents it can consider at once. Larger windows enable longer conversations and more coherent responses over bigger codebases, at the cost of higher latency and compute usage.

# Ambient (cloud) agent UI
ai-ui-ambient-start-cloud-agent = Start a new Oz cloud agent
ai-ui-ambient-cloud-env-intro = Cloud agents require an environment that they'll run in to get their task done. Create your first environment below. You'll be able to edit the environment later, or add new environments when you need them.
ai-ui-ambient-free-credits = Free credits
ai-ui-ambient-failed-start-env = Failed to start environment
ai-ui-ambient-github-auth-required = GitHub Authentication Required
ai-ui-ambient-github-auth-msg = Please authenticate with GitHub to continue
ai-ui-ambient-cancelled-title = Cloud Agent Run Cancelled
ai-ui-ambient-no-cloud-env = No cloud environment was started

# Conversation list / Codex modal
ai-ui-no-conversations = No conversations yet
ai-ui-conversation-list-search = Search
ai-ui-conversation-list-view-all = View all
ai-ui-conversation-list-show-less = Show less
ai-ui-conversation-list-empty-desc = Your active and past conversations with local and ambient agents will appear here.
ai-ui-conversation-list-new-conversation = New conversation
ai-ui-conversation-list-active-section = ACTIVE
ai-ui-conversation-list-past-section = PAST
ai-ui-conversation-list-delete = Delete
ai-ui-conversation-list-cannot-delete = This conversation cannot be deleted
ai-ui-conversation-list-share = Share conversation
ai-ui-conversation-list-fork-new-pane = Fork in new pane
ai-ui-conversation-list-fork-new-tab = Fork in new tab
ai-ui-codex-new = New

# Agent management
ai-ui-agent-owner-all = All
ai-ui-agent-owner-all-tooltip = View your agent tasks plus all shared team tasks
ai-ui-agent-owner-personal = Personal
ai-ui-agent-owner-personal-tooltip = View agent tasks you created
ai-ui-agent-get-started = Get started
ai-ui-agent-view-agents = View Agents
ai-ui-agent-clear-filters = Clear filters
ai-ui-agent-clear-all = Clear all
ai-ui-agent-search = Search
ai-ui-agent-new-agent = New agent
ai-ui-agent-filter-header = { $prefix }: { $value }
ai-ui-agent-filter-all = All
ai-ui-agent-filter-none = None
ai-ui-agent-filter-status = Status
ai-ui-agent-filter-working = Working
ai-ui-agent-filter-done = Done
ai-ui-agent-filter-failed = Failed
ai-ui-agent-filter-source = Source
ai-ui-agent-filter-created-on = Created on
ai-ui-agent-filter-last-24-hours = Last 24 hours
ai-ui-agent-filter-past-3-days = Past 3 days
ai-ui-agent-filter-last-week = Last week
ai-ui-agent-filter-has-artifact = Has artifact
ai-ui-agent-filter-pull-request = Pull Request
ai-ui-agent-filter-plan = Plan
ai-ui-agent-filter-screenshot = Screenshot
ai-ui-agent-filter-file = File
ai-ui-agent-filter-harness = Harness
ai-ui-agent-filter-environment = Environment
ai-ui-agent-filter-created-by = Created by
ai-ui-agent-source-linear = Linear
ai-ui-agent-source-api = API
ai-ui-agent-source-slack = Slack
ai-ui-agent-source-cli = CLI
ai-ui-agent-source-scheduled = Scheduled
ai-ui-agent-source-warp-app = Warp App
ai-ui-agent-source-oz-web = Oz Web
ai-ui-agent-source-github-action = GitHub Action
ai-ui-agent-source-github-webhook = GitHub Webhook
ai-ui-agent-source-orchestration = Orchestration
ai-ui-agent-copied-branch-name = Copied branch name
ai-ui-agent-session-expired = Session expired
ai-ui-agent-session-expired-tooltip = Sessions expire after one week and cannot be opened.
ai-ui-agent-no-session-available = No session available
ai-ui-agent-unknown-creator = Unknown
ai-ui-agent-metadata-source = Source: { $source }
ai-ui-agent-metadata-harness = Harness: { $harness }
ai-ui-agent-metadata-run-time = Run time: { $run_time }
ai-ui-agent-metadata-credits-used = Credits used: { $usage }
ai-ui-agent-runs = Runs
ai-ui-agent-loading-cloud-runs = Loading cloud agent runs
ai-ui-agent-loading-agents = Loading agents...
ai-ui-agent-no-filter-results = No results matched your filters

# Agent notifications
ai-ui-notifications-close = Close
ai-ui-notifications-mark-all-read = Mark all as read
ai-ui-notifications-title = Notifications
ai-ui-notifications-filter-all-tabs = All tabs
ai-ui-notifications-filter-unread = Unread
ai-ui-notifications-filter-errors = Errors
ai-ui-notifications-filter-with-count = { $label } ({ $count })
ai-ui-notifications-empty = No notifications

# Orchestration controls
ai-ui-orchestration-agent-location = Agent location
ai-ui-orchestration-view-details = View details
# warp-cn: 2026-06 sync — newly localized upstream strings
ai-ui-conversation-details-agent = Agent
ai-ui-orchestration-modal-title = Orchestrate any agent, anywhere
ai-ui-orchestration-modal-description = We've made major improvements to Warp's cloud agent orchestration platform, Oz.
ai-ui-openai-base-url-prompt = OpenAI base URL (optional, press Enter to skip):
ai-ui-queued-prompts-to-send = to send

# warp-cn: 2026-08 sync — newly localized upstream strings
ai-ui-agent-cli-modal-badge = New
ai-ui-agent-cli-modal-title = Introducing the Warp Agent CLI: A coding agent that does what others can't
ai-ui-agent-cli-modal-whats-new-label = What's new:
ai-ui-agent-cli-modal-whats-new-title = Use the Warp Agent anywhere
ai-ui-agent-cli-modal-whats-new-desc = Warp's state of the art agent is now available in any terminal through the CLI.
ai-ui-agent-cli-modal-whats-special-label = What's special:
ai-ui-agent-cli-modal-whats-special-title = Built-in terminal multiplexer
ai-ui-agent-cli-modal-whats-special-desc = Each Warp Agent creates its own PTY, enabling better behavior for REPLS, ssh, directory switching, and more.
ai-ui-auto-handoff-modal-badge = Run Connection Lost
ai-ui-auto-handoff-modal-title = Enable auto-handoff?
ai-ui-auto-handoff-modal-desc = Give Warp the option to automatically move active local agents to the cloud when your computer sleeps.
ai-ui-auto-handoff-modal-enable = Enable
ai-ui-run-agents-child-agents-note = These agents may start their own child agents
ai-ui-context-window-other-tooltip = Includes other request context and temporary instructions added to help the agent better respond.
