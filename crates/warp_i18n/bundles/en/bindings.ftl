# Keyboard binding command names (command palette + Keyboard Shortcuts settings page).
# Keys MUST start with `binding-`. Group keys by source module: `binding-<scope>-<name>`.

# pane_group/mod.rs (14 commands)
binding-pane-group-close-current-session = Close Current Session
binding-pane-group-add-left = Split pane left
binding-pane-group-add-right = Split pane right
binding-pane-group-add-up = Split pane up
binding-pane-group-add-down = Split pane down
binding-pane-group-navigate-left = Switch panes left
binding-pane-group-navigate-right = Switch panes right
binding-pane-group-navigate-up = Switch panes up
binding-pane-group-navigate-down = Switch panes down
binding-pane-group-resize-left = Resize pane > Move divider left
binding-pane-group-resize-right = Resize pane > Move divider right
binding-pane-group-resize-up = Resize pane > Move divider up
binding-pane-group-resize-down = Resize pane > Move divider down
binding-pane-group-toggle-maximize-pane = Toggle Maximize Active Pane

# terminal/input.rs (12 commands; line 1782 edit-prompt already uses t!)
binding-input-show-history = Show History
binding-input-show-network-log = Show Warp network log
binding-input-clear-screen = Clear screen
binding-input-toggle-classic-completions = (Experimental) Toggle classic completions mode
binding-input-command-search = Command Search
binding-input-history-search = History Search
binding-input-open-completions = Open completions menu
binding-input-toggle-workflows = Workflows
binding-input-ai-command-suggestions = Open AI Command Suggestions
binding-input-new-agent-conversation = New agent conversation
binding-input-trigger-auto-detection = Trigger Auto Detection
binding-input-clear-ai-context-menu-query = Clear and reset AI context menu query

# code/editor/view/actions.rs (~32 EditableBinding + 7 FixedBinding::custom)
# Clipboard / undo (FixedBinding::custom)
binding-code-editor-copy = Copy
binding-code-editor-paste = Paste
binding-code-editor-cut = Cut
binding-code-editor-undo = Undo
binding-code-editor-redo = Redo
# Word navigation
binding-code-editor-move-backward-one-word = Move backward one word
binding-code-editor-move-forward-one-word = Move forward one word
# Cursor navigation
binding-code-editor-up = Move cursor up
binding-code-editor-down = Move cursor down
binding-code-editor-left = Move cursor left
binding-code-editor-right = Move cursor right
binding-code-editor-move-to-line-start = Move to line start
binding-code-editor-home = Home
binding-code-editor-move-to-line-end = Move to line end
binding-code-editor-end = End
binding-code-editor-cursor-at-buffer-start = Cursor at buffer start
binding-code-editor-cursor-at-buffer-end = Cursor at buffer end
# Selection
binding-code-editor-select-left-by-word = Select one word to the left
binding-code-editor-select-right-by-word = Select one word to the right
binding-code-editor-select-left = Select one character to the left
binding-code-editor-select-right = Select one character to the right
binding-code-editor-select-up = Select up
binding-code-editor-select-down = Select down
binding-code-editor-select-all = Select all
binding-code-editor-select-to-line-start = Select to start of line
binding-code-editor-select-to-line-end = Select to end of line
# Text manipulation
binding-code-editor-backspace = Remove the previous character
binding-code-editor-toggle-comment = Toggle comment
binding-code-editor-delete = Delete
binding-code-editor-cut-word-left = Cut word left
binding-code-editor-delete-word-left = Delete word left
binding-code-editor-cut-word-right = Cut word right
binding-code-editor-delete-word-right = Delete word right
binding-code-editor-cut-all-left = Cut all left
binding-code-editor-delete-all-left = Delete all left
binding-code-editor-cut-all-right = Cut all right
binding-code-editor-delete-all-right = Delete all right
# Vim / find / go to line
binding-code-editor-vim-exit-insert-mode = Exit Vim insert mode
binding-code-editor-find = Find in code editor
binding-code-editor-go-to-line = Go to line

# terminal/view/init.rs (72 EditableBinding commands)
binding-terminal-view-alternate-terminal-paste = Alternate terminal paste
binding-terminal-view-toggle-cli-agent-rich-input = Toggle CLI Agent Rich Input
binding-terminal-view-warpify-subshell = Warpify subshell
binding-terminal-view-warpify-ssh-session = Warpify ssh session
binding-terminal-view-accept-prompt-suggestion = Accept Prompt Suggestion
binding-terminal-view-cancel-active-process-mac-linux = Cancel active process
binding-terminal-view-cancel-active-process-windows = Copy text or cancel active process
binding-terminal-view-focus-input = Focus terminal input
binding-terminal-view-paste = Paste
binding-terminal-view-copy = Copy
binding-terminal-view-reinput-commands = Reinput selected commands
binding-terminal-view-reinput-commands-with-sudo = Reinput selected commands as root
binding-terminal-view-find-in-terminal = Find in Terminal
binding-terminal-view-select-bookmark-up = Select the closest bookmark up
binding-terminal-view-select-bookmark-down = Select the closest bookmark down
binding-terminal-view-open-block-list-context-menu = Open block context menu
binding-terminal-view-toggle-teams-modal = Toggle team workflows modal
binding-terminal-view-copy-git-branch = Copy git branch
binding-terminal-view-clear-blocks = Clear Blocks
binding-terminal-view-executing-command-move-word-left = Move cursor one word to the left within an executing command
binding-terminal-view-executing-command-move-word-right = Move cursor one word to the right within an executing command
binding-terminal-view-executing-command-move-home = Move cursor home within an executing command
binding-terminal-view-executing-command-move-end = Move cursor end within an executing command
binding-terminal-view-executing-command-delete-word-left = Delete word left within an executing command
binding-terminal-view-executing-command-delete-line-start = Delete to line start within an executing command
binding-terminal-view-executing-command-delete-line-end = Delete to line end within an executing command
binding-terminal-view-backward-tabulation = Backward tabulation within an executing command
binding-terminal-view-select-previous-block = Select previous block
binding-terminal-view-select-next-block = Select next block
binding-terminal-view-share-selected-block = Share selected block
binding-terminal-view-bookmark-selected-block = Bookmark selected block
binding-terminal-view-find-within-selected-block = Find within selected block
binding-terminal-view-copy-command-and-output = Copy command and output
binding-terminal-view-copy-command-output = Copy command output
binding-terminal-view-copy-command = Copy command
binding-terminal-view-scroll-up-one-line = Scroll terminal output up one line
binding-terminal-view-scroll-down-one-line = Scroll terminal output down one line
binding-terminal-view-scroll-to-top-of-selected-block = Scroll to top of selected block
binding-terminal-view-scroll-to-bottom-of-selected-block = Scroll to bottom of selected block
binding-terminal-view-select-all-blocks = Select all blocks
binding-terminal-view-expand-block-selection-above = Expand selected blocks above
binding-terminal-view-expand-block-selection-below = Expand selected blocks below
binding-terminal-view-ask-ai-about-selection = Ask Warp AI about Selection
binding-terminal-view-ask-ai-last-block = Ask Warp AI about last block
binding-terminal-view-ask-ai = Ask Warp AI
binding-terminal-view-insert-command-correction = Insert Command Correction
binding-terminal-view-debug-onboarding-warpinput-project = [Debug] Onboarding Callout: WarpInput - Project
binding-terminal-view-debug-onboarding-warpinput-no-project = [Debug] Onboarding Callout: WarpInput - No Project
binding-terminal-view-debug-onboarding-modality-project = [Debug] Onboarding Callout: Modality - Project
binding-terminal-view-debug-onboarding-modality-no-project = [Debug] Onboarding Callout: Modality - No Project
binding-terminal-view-debug-onboarding-modality-terminal = [Debug] Onboarding Callout: Modality - Terminal
binding-terminal-view-import-external-settings = Import External Settings
binding-terminal-view-share-current-session = Share current session
binding-terminal-view-stop-sharing-current-session = Stop sharing current session
binding-terminal-view-toggle-block-filter = Toggle block filter on selected or last block
binding-terminal-view-toggle-snackbar = Toggle Sticky Command Header in Active Pane
binding-terminal-view-toggle-autoexecute-mode = Toggle Auto-execute Mode
binding-terminal-view-toggle-queue-next-prompt = Toggle Queue Next Prompt
binding-terminal-view-debug-generate-codebase-index = [Debug] Generate codebase index
binding-terminal-view-load-agent-mode-conversation = Load agent mode conversation (from debug link in clipboard)
binding-terminal-view-toggle-session-recording = Toggle PTY Recording for Session
binding-terminal-view-set-input-mode-agent = Set Input Mode to Agent Mode
binding-terminal-view-set-input-mode-terminal = Set Input Mode to Terminal Mode
binding-terminal-view-toggle-hide-cli-responses = Toggle Hide CLI Responses

# notebooks/editor/view.rs (~51 commands; rich-text editor for notebooks/comments)
# Clipboard / undo (FixedBinding::custom — share semantics with code editor)
binding-notebooks-editor-copy = Copy
binding-notebooks-editor-paste = Paste
binding-notebooks-editor-cut = Cut
binding-notebooks-editor-undo = Undo
binding-notebooks-editor-redo = Redo
# Command-selection
binding-notebooks-editor-deselect-command = De-select shell commands
binding-notebooks-editor-select-command = Select shell command at cursor
binding-notebooks-editor-select-previous-command = Select previous command
binding-notebooks-editor-select-next-command = Select next command
binding-notebooks-editor-run-commands = Run selected commands
# Debug
binding-notebooks-editor-toggle-debug-mode = Toggle rich-text debug mode
binding-notebooks-editor-debug-copy-buffer = Copy rich-text buffer
binding-notebooks-editor-debug-copy-selection = Copy rich-text selection
binding-notebooks-editor-log-state = Log editor state
# Word navigation
binding-notebooks-editor-move-backward-one-word = Move backward one word
binding-notebooks-editor-move-forward-one-word = Move forward one word
# Cursor navigation
binding-notebooks-editor-up = Move cursor up
binding-notebooks-editor-down = Move cursor down
binding-notebooks-editor-left = Move cursor left
binding-notebooks-editor-right = Move cursor right
binding-notebooks-editor-move-to-paragraph-start = Move to start of paragraph
binding-notebooks-editor-home = Home
binding-notebooks-editor-move-to-paragraph-end = Move to end of paragraph
binding-notebooks-editor-end = End
# Selection
binding-notebooks-editor-select-left-by-word = Select one word to the left
binding-notebooks-editor-select-right-by-word = Select one word to the right
binding-notebooks-editor-select-left = Select one character to the left
binding-notebooks-editor-select-right = Select one character to the right
binding-notebooks-editor-select-up = Select up
binding-notebooks-editor-select-down = Select down
binding-notebooks-editor-select-all = Select all
binding-notebooks-editor-select-to-paragraph-start = Select to start of paragraph
binding-notebooks-editor-select-to-paragraph-end = Select to end of paragraph
binding-notebooks-editor-select-to-line-end = Select to end of line
binding-notebooks-editor-select-to-line-start = Select to start of line
# Text manipulation
binding-notebooks-editor-backspace = Remove the previous character
binding-notebooks-editor-delete = Delete
binding-notebooks-editor-cut-word-left = Cut word left
binding-notebooks-editor-delete-word-left = Delete word left
binding-notebooks-editor-cut-word-right = Cut word right
binding-notebooks-editor-delete-word-right = Delete word right
binding-notebooks-editor-cut-all-left = Cut all left
binding-notebooks-editor-delete-all-left = Delete all left
binding-notebooks-editor-cut-all-right = Cut all right
binding-notebooks-editor-delete-all-right = Delete all right
# Rich-text styling
binding-notebooks-editor-edit-link = Create or edit link
binding-notebooks-editor-inline-code = Toggle inline code styling
binding-notebooks-editor-strikethrough = Toggle strikethrough styling
binding-notebooks-editor-underline = Toggle underline styling
# Find bar
binding-notebooks-editor-find = Find in Notebook
binding-notebooks-editor-next-find-match = Focus next match
binding-notebooks-editor-previous-find-match = Focus previous match
binding-notebooks-editor-toggle-regex-find = Toggle regular expression search
binding-notebooks-editor-toggle-case-sensitive-find = Toggle case-sensitive search

# editor/view/mod.rs (~60 EditableBinding + 6 FixedBinding::custom; terminal command editor)
# Clipboard / undo (FixedBinding::custom)
binding-editor-view-copy = Copy
binding-editor-view-cut = Cut
binding-editor-view-paste = Paste
binding-editor-view-undo = Undo
binding-editor-view-redo = Redo
# Selection
binding-editor-view-select-left-by-word = Select one word to the left
binding-editor-view-select-right-by-word = Select one word to the right
binding-editor-view-select-left = Select one character to the left
binding-editor-view-select-right = Select one character to the right
binding-editor-view-select-up = Select up
binding-editor-view-select-down = Select down
binding-editor-view-select-all = Select all
binding-editor-view-select-to-line-start = Select to start of line
binding-editor-view-select-to-line-end = Select to end of line
binding-editor-view-select-left-by-subword = Select one subword to the left
binding-editor-view-select-right-by-subword = Select one subword to the right
binding-editor-view-clear-and-copy-lines = Copy and clear selected lines
binding-editor-view-add-next-occurrence = Add selection for next occurrence
# Cursor navigation
binding-editor-view-up = Move cursor up
binding-editor-view-down = Move cursor down
binding-editor-view-left = Move cursor left
binding-editor-view-right = Move cursor right
binding-editor-view-move-to-line-start = Move to start of line
binding-editor-view-move-to-line-end = Move to end of line
binding-editor-view-home = Home
binding-editor-view-end = End
binding-editor-view-cmd-down = Move cursor to the bottom
binding-editor-view-cmd-up = Move cursor to the top
binding-editor-view-move-to-and-select-buffer-start = Select and move to the top
binding-editor-view-move-to-and-select-buffer-end = Select and move to the bottom
binding-editor-view-move-forward-one-word = Move forward one word
binding-editor-view-move-backward-one-word = Move backward one word
binding-editor-view-move-forward-one-subword = Move Forward One Subword
binding-editor-view-move-backward-one-subword = Move Backward One Subword
binding-editor-view-move-to-paragraph-start = Move to the start of the paragraph
binding-editor-view-move-to-paragraph-end = Move to the end of the paragraph
binding-editor-view-move-to-buffer-start = Move to the start of the buffer
binding-editor-view-move-to-buffer-end = Move to the end of the buffer
# Buffer modifications
binding-editor-view-backspace = Remove the previous character
binding-editor-view-cut-word-left = Cut word left
binding-editor-view-delete-word-left = Delete word left
binding-editor-view-cut-word-right = Cut word right
binding-editor-view-delete = Delete
binding-editor-view-delete-word-right = Delete word right
binding-editor-view-clear-lines = Clear selected lines
binding-editor-view-cut-all-right = Cut all right
binding-editor-view-delete-all-right = Delete all right
binding-editor-view-delete-all-left = Delete all left
binding-editor-view-insert-newline = Insert newline
# Folds
binding-editor-view-fold = Fold
binding-editor-view-unfold = Unfold
binding-editor-view-fold-selected-ranges = Fold selected ranges
# Misc
binding-editor-view-insert-last-word-previous-command = Insert last word of previous command
binding-editor-view-accept-autosuggestion = Accept autosuggestion
binding-editor-view-inspect-command = Inspect Command
binding-editor-view-clear-buffer = Clear command editor
binding-editor-view-add-cursor-above = Add cursor above
binding-editor-view-add-cursor-below = Add cursor below
binding-editor-view-insert-nonexpanding-space = Insert non-expanding space
binding-editor-view-vim-exit-insert-mode = Exit Vim insert mode

# workspace/mod.rs (~67 string-literal descriptions; many other bindings here already use t! path)
# Debug bindings (gated by ChannelState::enable_debug_features)
binding-workspace-crash-mac = Crash the app (for testing sentry-cocoa)
binding-workspace-crash-other = Crash the app (for testing sentry-native)
binding-workspace-log-review-comment-send-status = [Debug] Log review comment send status for active tab
binding-workspace-panic = Trigger a panic (for testing sentry-rust)
binding-workspace-open-view-tree-debugger = Open view tree debugger
binding-workspace-debug-fte-view = [Debug] View first-time user experience
binding-workspace-debug-build-plan-migration-modal = [Debug] Open Build Plan Migration Modal
binding-workspace-debug-reset-build-plan-migration = [Debug] Reset Build Plan Migration Modal State
binding-workspace-debug-reset-aws-bedrock-banner = [Debug] Un-dismiss AWS login banner
binding-workspace-debug-open-oz-launch-modal = [Debug] Open Oz Launch Modal
binding-workspace-debug-reset-oz-launch-modal = [Debug] Reset Oz Launch Modal State
binding-workspace-debug-open-openwarp-launch-modal = [Debug] Open OpenWarp Launch Modal
binding-workspace-debug-reset-openwarp-launch-modal = [Debug] Reset OpenWarp Launch Modal State
binding-workspace-debug-install-opencode-warp-plugin = [Debug] Install OpenCode Warp plugin
binding-workspace-debug-use-local-opencode-warp-plugin = [Debug] Use local OpenCode Warp plugin (testing only)
binding-workspace-debug-open-session-config-modal = [Debug] Open Session Config Modal
binding-workspace-debug-show-hoa-onboarding = [Debug] Start HOA Onboarding Flow
binding-workspace-sample-process = Sample Process
binding-workspace-dump-heap-profile = Dump heap profile (can only be done once)
# Tab navigation (FixedBinding::custom)
binding-workspace-cycle-next-session = Switch to next tab
binding-workspace-cycle-prev-session = Switch to previous tab
binding-workspace-add-window = Create New Window
binding-workspace-new-file = New File
# Zoom / font size
binding-workspace-zoom-in = Zoom In
binding-workspace-zoom-out = Zoom Out
binding-workspace-reset-zoom = Reset Zoom
binding-workspace-increase-zoom-level = Increase zoom level
binding-workspace-decrease-zoom-level = Decrease zoom level
binding-workspace-reset-zoom-level = Reset zoom level to default
binding-workspace-increase-font-size = Increase font size
binding-workspace-decrease-font-size = Decrease font size
binding-workspace-reset-font-size = Reset font size to default
# Themes / tabs
binding-workspace-open-theme-picker = Open theme picker
binding-workspace-open-tab-configs-menu = Open tab configs menu
# Tab switching
binding-workspace-activate-tab-1 = Switch to 1st tab
binding-workspace-activate-tab-2 = Switch to 2nd tab
binding-workspace-activate-tab-3 = Switch to 3rd tab
binding-workspace-activate-tab-4 = Switch to 4th tab
binding-workspace-activate-tab-5 = Switch to 5th tab
binding-workspace-activate-tab-6 = Switch to 6th tab
binding-workspace-activate-tab-7 = Switch to 7th tab
binding-workspace-activate-tab-8 = Switch to 8th tab
binding-workspace-activate-tab-last = Switch to last tab
binding-workspace-activate-prev-tab = Activate previous tab
binding-workspace-activate-next-tab = Activate next tab
binding-workspace-activate-prev-pane = Activate previous pane
binding-workspace-activate-next-pane = Activate next pane
# Layout / palettes
binding-workspace-toggle-keybindings-page = Toggle keyboard shortcuts
binding-workspace-open-keybindings-editor = Open keybindings editor
binding-workspace-toggle-block-snackbar = Toggle sticky command header
# A11y
binding-workspace-a11y-concise = [a11y] Set concise accessibility announcements
binding-workspace-a11y-verbose = [a11y] Set verbose accessibility announcements
# Tab management
binding-workspace-rename-active-tab = Rename the current tab
binding-workspace-quit-warp = Quit Warp
binding-workspace-close-active-tab = Close the current tab
binding-workspace-close-other-tabs = Close other tabs
# Notifications
binding-workspace-toggle-notifications-on = Turn notifications on
binding-workspace-toggle-notifications-off = Turn notifications off
# Palettes
binding-workspace-launch-config-palette = Launch configuration palette
binding-workspace-toggle-files-palette = Toggle Files Palette
binding-workspace-save-launch-configuration = Save new launch configuration
binding-workspace-search-warp-drive = Search Warp Drive
# Updates
binding-workspace-update-and-relaunch = Install update and relaunch
binding-workspace-check-for-updates = Check for updates
# Misc
binding-workspace-log-out = Log out
binding-workspace-toggle-resource-center = Toggle resource center
binding-workspace-export-warp-drive-objects = Export all Warp Drive objects
binding-workspace-install-cli = Install Oz CLI globally for use outside of Warp
binding-workspace-uninstall-cli = Undo global Oz CLI installation (oz will still work within Warp)
binding-workspace-view-changelog = View latest changelog
binding-workspace-toggle-warp-ai = Toggle Warp AI
binding-workspace-shift-focus-left = Switch Focus to Left Panel
binding-workspace-shift-focus-right = Switch Focus to Right Panel
binding-workspace-import-to-personal-drive = Import To Personal Drive
binding-workspace-import-to-team-drive = Import To Team Drive
binding-workspace-copy-access-token = Copy access token to clipboard
binding-workspace-jump-to-latest-toast = Jump to latest agent task
binding-workspace-toggle-notification-mailbox = Toggle notification mailbox
binding-workspace-toggle-agent-management-view = Toggle the agent management view
# Settings page openers (only those still using literal — others use t! path)
binding-workspace-open-settings-account = Open Settings: Account
binding-workspace-open-settings-features = Open Settings: Features
binding-workspace-open-settings-file = Open settings file
# Overflow menu
binding-workspace-invite-people = Invite People...
binding-workspace-link-to-slack = Join our Slack community (opens external link)
binding-workspace-link-to-user-docs = View user docs (opens external link)
binding-workspace-view-warp-logs = View Warp logs
binding-workspace-link-to-privacy-policy = View privacy policy (opens external link)
binding-workspace-dump-debug-info = Dump debug info

# Scattered files (task #20)
# notebooks/notebook.rs (4 EditableBinding + 2 FixedBinding::custom)
binding-notebooks-increase-font-size = Increase notebook font size
binding-notebooks-decrease-font-size = Decrease notebook font size
binding-notebooks-reset-font-size = Reset notebook font size
binding-notebooks-focus-terminal-input = Focus Terminal Input from Notebook
binding-notebooks-fixed-increase-font-size = Increase font size
binding-notebooks-fixed-decrease-font-size = Decrease font size

# code/view.rs (4 EditableBinding) — SAVE_FILE_BINDING reused by ai_document_view
binding-code-view-save-file = Save file
binding-code-view-save-file-as = Save file as
binding-code-view-close-all-tabs = Close all tabs
binding-code-view-close-saved-tabs = Close saved tabs

# ai_assistant/panel.rs (1 FixedBinding::custom + 3 EditableBinding)
binding-ai-assistant-close-warp-ai = Close Warp AI
binding-ai-assistant-focus-terminal-input = Focus Terminal Input From Warp AI
binding-ai-assistant-restart-warp-ai = Restart Warp AI

# workspace/sync_inputs.rs (3 EditableBinding)
binding-workspace-disable-input-syncing = Stop Synchronizing Any Panes
binding-workspace-toggle-sync-inputs-tab = Toggle Synchronizing All Panes in Current Tab
binding-workspace-toggle-sync-inputs-all-tabs = Toggle Synchronizing All Panes in All Tabs

# code_review/mod.rs (2 EditableBinding + 1 FixedBinding::custom)
binding-code-review-save-all = Save all unsaved files in code review
binding-code-review-show-find-bar = Show find bar in code review
binding-code-review-undo-revert = Undo
binding-code-review-submit-review-comments = Send code review comments to agent

# workflows/workflow_view.rs (2 EditableBinding)
binding-workflow-save = Save workflow
binding-workflow-close = Close

# view_components/find.rs + code/editor/find/view.rs (shared keys)
binding-find-next-occurrence = Find the next occurrence of your search query
binding-find-prev-occurrence = Find the previous occurrence of your search query

# root_view.rs (2 EditableBinding)
binding-root-view-toggle-fullscreen = Toggle fullscreen
binding-root-view-debug-enter-onboarding = [Debug] Enter Onboarding State

# pane_group/pane/welcome_view.rs (2 EditableBinding)
binding-welcome-view-terminal-session = Terminal session
binding-welcome-view-add-repository = Add repository

# notebooks/file/mod.rs (2 EditableBinding)
binding-notebooks-file-focus-terminal-input = Focus Terminal Input from File
binding-notebooks-file-reload = Reload file

# coding_entrypoints/project_buttons.rs (2 EditableBinding)
binding-project-buttons-open-repository = Open repository
binding-project-buttons-create-new-project = Create new project

# workspace/view/right_panel.rs (1 EditableBinding)
binding-right-panel-toggle-maximize-code-review = Toggle Maximize Code Review Panel

# undo_close/mod.rs (1 EditableBinding)
binding-undo-close-reopen = Reopen closed session

# pane_group/pane/view/mod.rs (1 EditableBinding)
binding-pane-share-contents = Share pane

# pane_group/pane/get_started_view.rs (1 EditableBinding)
binding-get-started-terminal-session = Terminal session

# env_vars/view/env_var_collection.rs (1 EditableBinding)
binding-env-var-collection-close = Close

# ai/blocklist/inline_action/requested_command.rs (1 EditableBinding)
binding-requested-command-edit = Edit requested command

# ai/blocklist/inline_action/code_diff_view.rs (1 EditableBinding)
binding-code-diff-edit = Edit Code Diff

# ai/blocklist/block/cli.rs (1 EditableBinding)
binding-cli-take-control-of-running-command = Take control of running command
