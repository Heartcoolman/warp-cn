# AI / Agents settings page strings (WarpAgent, AgentProfiles, Knowledge, ThirdPartyCLIAgents).
# Keys MUST start with `settings-ai-`.

settings-ai-title = AI
settings-ai-remote-session-disallowed = Your organization disallows AI when the active pane contains content from a remote session
settings-ai-next-command-description = Let AI suggest the next command to run based on your command history, outputs, and common workflows.
settings-ai-prompt-suggestions-description = Let AI suggest natural language prompts, as inline banners in the input, based on recent commands and their outputs.
settings-ai-code-banners-description = Let AI suggest code diffs and queries as inline banners in the blocklist, based on recent commands and their outputs.
settings-ai-autosuggestions-description = Let AI suggest natural language autosuggestions, based on recent commands and their outputs.
settings-ai-shared-block-title-description = Let AI generate a title for your shared block based on the command and output.
settings-ai-git-autogen-description = Let AI generate commit messages and pull request titles and descriptions.
settings-ai-commands-placeholder = Commands, comma separated
settings-ai-agent-decides = Agent decides
settings-ai-always-allow = Always allow
settings-ai-always-ask = Always ask
settings-ai-allow-specific-dirs = Allow in specific directories
settings-ai-show-oz-changelog-agent = Show Oz changelog in new agent conversation view
settings-ai-hide-oz-changelog-agent = Hide Oz changelog in new agent conversation view
settings-ai-show-oz-changelog = Show Oz changelog in new conversation view
settings-ai-show-use-agent-footer = Show "Use Agent" footer
settings-ai-hide-use-agent-footer = Hide "Use Agent" footer
settings-ai-voice-input-toggle-key-error = Could not find current VoiceInputToggleKey value in dropdown option list
settings-ai-edit = Edit
settings-ai-models = MODELS
settings-ai-permissions = PERMISSIONS
settings-ai-base-model = Base model:
settings-ai-full-terminal-use = Full terminal use:
settings-ai-computer-use = Computer use:
settings-ai-apply-code-diffs = Apply code diffs:
settings-ai-read-files = Read files:
settings-ai-execute-commands = Execute commands:
settings-ai-interact-running-commands = Interact with running commands:
settings-ai-ask-questions = Ask questions:
settings-ai-call-mcp-servers = Call MCP servers:
settings-ai-call-web-tools = Call web tools:
settings-ai-auto-sync-plans = Auto-sync plans to Warp Drive:
settings-ai-ask-on-first-write = Ask on first write
settings-ai-never = Never
settings-ai-never-ask = Never ask
settings-ai-ask-unless-auto-approve = Ask unless auto-approve
settings-ai-on = On
settings-ai-off = Off
settings-ai-none = None
settings-ai-directory-allowlist = Directory allowlist:
settings-ai-command-allowlist = Command allowlist:
settings-ai-command-denylist = Command denylist:
settings-ai-mcp-allowlist = MCP allowlist:
settings-ai-mcp-denylist = MCP denylist:
settings-ai-auto = Auto
settings-ai-unknown = Unknown

# Section / toggle headings
settings-ai-section-permissions = Permissions
settings-ai-perm-apply-diffs = Apply code diffs
settings-ai-perm-read-files = Read files
settings-ai-perm-execute = Execute commands
settings-ai-perm-interact = Interact with running commands
settings-ai-perm-managed-by-workspace = Some of your permissions are managed by your workspace.
settings-ai-cmd-denylist = Command denylist
settings-ai-cmd-denylist-desc = Regular expressions to match commands that the Warp Agent should always ask permission to execute.
settings-ai-cmd-allowlist = Command allowlist
settings-ai-cmd-allowlist-desc = Regular expressions to match commands that can be automatically executed by the Warp Agent.
settings-ai-dir-allowlist = Directory allowlist
settings-ai-dir-allowlist-desc = Give the agent file access to certain directories.

settings-ai-base-model-title = Base model
settings-ai-base-model-desc = This model serves as the primary engine behind the Warp Agent. It powers most interactions and invokes other models for tasks like planning or code generation when necessary. Warp may automatically switch to alternate models based on model availability or for auxiliary tasks such as conversation summarization.

settings-ai-codebase-context = Codebase Context
settings-ai-codebase-context-desc = Allow the Warp Agent to generate an outline of your codebase that can be used for context. No code is ever stored on our servers.
settings-ai-learn-more = Learn more

settings-ai-mcp-call-servers = Call MCP servers
settings-ai-mcp-allowlist-desc = Allow the Warp Agent to call these MCP servers.
settings-ai-mcp-denylist-desc = The Warp Agent will always ask for permission before calling any MCP servers on this list.
settings-ai-mcp-add-server = Add a server
settings-ai-mcp-manage = Manage MCP servers
settings-ai-mcp-autospawn = Auto-spawn servers from third-party agents
settings-ai-mcp-autospawn-desc = Automatically detect and spawn MCP servers from globally-scoped third-party AI agent configuration files (e.g. in your home directory). Servers detected inside a repository are never spawned automatically and must be enabled individually from the MCP settings page.
settings-ai-mcp-supported-providers = See supported providers.

settings-ai-input-hint-text = Show input hint text
settings-ai-show-agent-tips = Show agent tips
settings-ai-include-agent-cmds-history = Include agent-executed commands in history

settings-ai-natural-lang-detection = Natural language detection
settings-ai-natural-lang-detection-desc = Enabling natural language detection will detect when natural language is written in the terminal input, and then automatically switch to Agent Mode for AI queries.
settings-ai-natural-lang-denylist = Natural language denylist
settings-ai-natural-lang-denylist-desc = Commands listed here will never trigger natural language detection.
settings-ai-let-us-know = Let us know
settings-ai-autodetect-prompts-terminal = Autodetect agent prompts in terminal input
settings-ai-autodetect-cmds-agent = Autodetect terminal commands in agent input

settings-ai-rules-help-desc = Rules help the Warp Agent follow your conventions, whether for codebases or specific workflows.
settings-ai-suggested-rules = Suggested Rules
settings-ai-suggested-rules-desc = Let AI suggest rules to save based on your interactions.
settings-ai-warp-drive-context = Warp Drive as agent context
settings-ai-warp-drive-context-desc = The Warp Agent can leverage your Warp Drive Contents to tailor responses to your personal and team developer workflows and environments. This includes any Workflows, Notebooks, and Environment Variables.
settings-ai-knowledge = Knowledge
settings-ai-manage-rules = Manage rules
settings-ai-voice-input = Voice Input
settings-ai-voice-input-desc = Voice input allows you to control Warp by speaking directly to your terminal (powered by
settings-ai-voice-input-key = Key for Activating Voice Input
settings-ai-voice-input-key-tip = Press and hold to activate.

settings-ai-show-conv-history = Show conversation history in tools panel
settings-ai-thinking-display = Agent thinking display
settings-ai-thinking-display-desc = Controls how reasoning/thinking traces are displayed.
settings-ai-existing-conv-layout = Preferred layout when opening existing agent conversations
settings-ai-show-coding-toolbar = Show coding agent toolbar
settings-ai-show-coding-toolbar-desc = Show a toolbar with quick actions when running coding agents like
settings-ai-third-party-cli-agents = Third party CLI agents
settings-ai-rich-input-auto-show = Auto show/hide Rich Input based on agent status
settings-ai-rich-input-requires-plugin = Requires the Warp plugin for your coding agent
settings-ai-rich-input-auto-open = Auto open Rich Input when a coding agent session starts
settings-ai-rich-input-auto-dismiss = Auto dismiss Rich Input after prompt submission
settings-ai-toolbar-regex-desc = Add regex patterns to show the coding agent toolbar for matching commands.

settings-ai-attribution-toggle = Enable agent attribution
settings-ai-attribution-section = Agent Attribution
settings-ai-attribution-desc = Oz can add attribution to commit messages and pull requests it creates

settings-ai-cloud-computer-use = Computer use in Cloud Agents
settings-ai-experimental = Experimental
settings-ai-cloud-computer-use-desc = Enable computer use in cloud agent conversations started from the Warp app.
settings-ai-orchestration = Orchestration
settings-ai-orchestration-desc = Enable multi-agent orchestration, allowing the agent to spawn and coordinate parallel sub-agents.

settings-ai-byok-desc = Use your own API keys from model providers for the Warp Agent to use. API keys are stored locally and never synced to the cloud. Using auto models or models from providers you have not provided API keys for will consume Warp credits.
settings-ai-openai-key = OpenAI API Key
settings-ai-anthropic-key = Anthropic API Key
settings-ai-google-key = Google API Key
settings-ai-upgrade-build = Upgrade to the Build plan
settings-ai-credit-fallback = Warp credit fallback

settings-ai-bedrock-managed-by-org = Warp loads and sends local AWS CLI credentials for Bedrock-supported models. This setting is managed by your organization.
settings-ai-bedrock-toggle = Use AWS Bedrock credentials
settings-ai-bedrock-login-cmd = Login Command
settings-ai-bedrock-auto-run-login = Automatically run login command
settings-ai-bedrock-auto-run-login-desc = When enabled, the login command will run automatically when AWS Bedrock credentials expire.

settings-ai-toolbar-layout = Toolbar layout
settings-ai-show-model-picker = Show model picker in prompt
settings-ai-toolbar-cmds-enable-span = Commands that enable the toolbar
settings-ai-active-ai = Active AI
settings-ai-next-command = Next Command
settings-ai-split-pane = Split Pane
settings-ai-read-only-permission = Read only
settings-ai-supervised-permission = Supervised
settings-ai-restricted-billing = Restricted due to billing issue
settings-ai-unlimited = Unlimited
settings-ai-prompt-suggestions = Prompt Suggestions
settings-ai-suggested-code-banners = Suggested Code Banners
settings-ai-natural-lang-autosuggestions = Natural Language Autosuggestions
settings-ai-shared-block-title-gen = Shared Block Title Generation
settings-ai-warp-agent = Warp Agent
settings-ai-create-account-prompt = To use AI features, please create an account.
settings-ai-profiles = Profiles
settings-ai-set-boundaries-desc = Set the boundaries for how your Agent operates. Choose what it can access, how much autonomy it has, and when it must ask for your approval. You can also fine-tune behavior around natural language input, codebase awareness, and more.
settings-ai-mcp-select-servers-header = Select MCP servers
settings-ai-other = Other
settings-ai-select-coding-agent = Select coding agent
settings-ai-resets-after = Resets {$time}

# Command palette suffixes
settings-ai-cmd-suffix-ai = AI
settings-ai-cmd-suffix-terminal-cmd-autodetect-agent = terminal command autodetection in agent input
settings-ai-cmd-suffix-natural-language-detection = natural language detection
settings-ai-cmd-suffix-agent-prompt-autodetect-terminal = agent prompt autodetection in terminal input
settings-ai-cmd-suffix-prompt-suggestions = prompt suggestions
settings-ai-cmd-suffix-code-suggestions = code suggestions
settings-ai-cmd-suffix-natural-language-autosuggestions = natural language autosuggestions
settings-ai-cmd-suffix-shared-block-title-generation = shared block title generation
settings-ai-cmd-suffix-voice-input = voice input
settings-ai-cmd-suffix-codebase-index = codebase index
settings-ai-cmd-suffix-git-operations-autogen = commit and pull request generation
settings-ai-cmd-suffix-include-agent-commands-in-history = include agent-executed commands in history
settings-ai-cmd-suffix-conversation-history-in-tools-panel = conversation history in tools panel
settings-ai-cmd-suffix-model-picker-in-prompt = model picker in prompt
settings-ai-cmd-suffix-coding-agent-toolbar = coding agent toolbar
settings-ai-cmd-suffix-rules = Rules
settings-ai-cmd-suffix-suggested-rules = Suggested Rules
settings-ai-cmd-suffix-warp-drive-as-agent-context = Warp Drive as agent context
settings-ai-cmd-suffix-auto-spawn-third-party-mcp = Auto-spawn servers from third-party agents
settings-ai-cmd-suffix-warp-credit-fallback = Warp credit fallback
settings-ai-cmd-suffix-auto-toggle-rich-input = auto show or hide Rich Input based on agent status
settings-ai-cmd-suffix-auto-open-rich-input-on-cli-agent-start = auto open Rich Input when a coding agent session starts
settings-ai-cmd-suffix-auto-dismiss-rich-input-after-submit = auto dismiss Rich Input after prompt submission
settings-ai-supergrok-connect-description = Connect your SuperGrok subscription to use Grok models in the Warp Agent through your xAI account.
