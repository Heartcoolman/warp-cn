# Settings top-level navigation labels and shared UI strings.
# Keys MUST start with `settings-` (spec: client-string-localization, Bundle namespace).

# Top-level section labels (sidebar nav).
settings-section-title = Settings
settings-section-account = Account
settings-section-billing-usage = Billing and usage
settings-section-teams = Teams
settings-section-appearance = Appearance
settings-section-features = Features
settings-section-keybindings = Keyboard shortcuts
settings-section-warpify = Warpify
settings-section-scripting = Scripting
settings-section-referrals = Referrals
settings-section-shared-blocks = Shared blocks
settings-section-warp-drive = Warp Drive
settings-section-mcp-servers = MCP Servers
settings-section-privacy = Privacy
settings-section-about = About

# Umbrella labels.
settings-umbrella-agents = Agents
settings-umbrella-code = Code
settings-umbrella-cloud-platform = Cloud platform

# Umbrella subpage labels.
settings-subpage-warp-agent = Warp Agent
settings-subpage-agent-profiles = Profiles
settings-subpage-agent-mcp-servers = MCP servers
settings-subpage-knowledge = Knowledge
settings-subpage-third-party-cli-agents = Third party CLI agents
settings-subpage-code-indexing = Indexing and projects
settings-subpage-editor-code-review = Editor and Code Review
settings-subpage-cloud-environments = Environments
settings-subpage-oz-cloud-api-keys = Oz Cloud API Keys

# Shared settings UI.
settings-search-placeholder = Search settings
settings-close-button = Close
settings-reset-default = Reset to default
settings-unsaved-changes = You have unsaved changes
settings-save-changes = Save changes
settings-discard-changes = Discard changes

# Search empty state.
settings-search-no-results = No settings match your search.
settings-search-no-results-hint = You may want to try using different keywords or checking for any possible typos.

# Agent assisted environment modal.
settings-agent-env-no-repos-selected = No repos selected yet
settings-agent-env-all-repos-selected = All locally indexed repos are already selected.

# Settings import flow
settings-import-restart-hint = Some settings will take effect when you open a new session.

# Environments page
settings-environments-new-button = New environment

# MCP edit page
settings-mcp-edit-format-json = JSON

# Context menu - pane operations
settings-menu-split-right = Split pane right
settings-menu-split-left = Split pane left
settings-menu-split-down = Split pane down
settings-menu-split-up = Split pane up
settings-menu-close-pane = Close pane

# Command palette: composed labels for toggle setting action pairs.
# `{$suffix}` is the per-toggle description (already localized).
settings-action-enable-suffix = Enable {$suffix}
settings-action-disable-suffix = Disable {$suffix}

# Developer command palette suffixes (used in settings_view/mod.rs init_actions_from_parent_view)
settings-cmd-suffix-recording-mode = recording mode
settings-cmd-suffix-in-band-generators = in-band generators for new sessions
settings-cmd-suffix-debug-network-status = debug network status
settings-cmd-suffix-memory-statistics = memory statistics

# settings_page.rs (shared infrastructure)
settings-page-tooltip-default-learn-more = Click to learn more in docs
settings-page-tooltip-local-only-default = This setting is not synced to your other devices
settings-page-button-reset-to-default = Reset to default

# settings_file_footer.rs
settings-file-footer-open-settings-file = Open settings file
settings-file-footer-alert-open-file = Open file
settings-file-footer-alert-fix-with-oz = Fix with Oz
