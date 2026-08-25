# Code settings page strings (Indexing and projects, Editor and Code Review).
# Keys MUST start with `settings-code-`.

settings-code-title = Code

# Subpage titles
settings-code-subpage-indexing = Codebase Indexing
settings-code-subpage-editor-review = Editor and Code Review

# Categories
settings-code-category-indexing = Codebase Indexing
settings-code-category-editor-review = Code Editor and Review

# Initialization Settings section
settings-code-init-settings-header = Initialization Settings
settings-code-codebase-indexing-label = Codebase indexing
settings-code-codebase-index-description = Warp can automatically index code repositories as you navigate them, helping agents quickly understand context and provide solutions. Code is never stored on the server. If a codebase is unable to be indexed, Warp can still navigate your codebase and gain insights via grep and find tool calling.
settings-code-warp-indexing-ignore-description = To exclude specific files or directories from indexing, add them to the .warpindexingignore file in your repository directory. These files will still be accessible to AI features, but they won't be included in codebase embeddings.
settings-code-auto-index-feature-name = Index new folders by default
settings-code-auto-index-description = When set to true, Warp will automatically index code repositories as you navigate them - helping agents quickly understand context and provide targeted solutions.
settings-code-indexing-disabled-admin = Team admins have disabled codebase indexing.
settings-code-indexing-disabled-admin-team = Codebase indexing is unavailable because { $team } has disabled it.
settings-code-indexing-workspace-enabled-admin = Team admins have enabled codebase indexing.
settings-code-indexing-disabled-global-ai = AI Features must be enabled to use codebase indexing.
settings-code-index-limit-reached = You have reached the maximum number of codebase indices for your plan. Delete existing indices to auto-index new codebases.

# Auggie backend (warp-cn)
settings-code-indexing-backend-cloud = Backend: Warp Cloud
settings-code-indexing-backend-auggie = Backend: Auggie (local)
settings-code-indexing-backend-auggie-running = Running on Auggie MCP
settings-code-indexing-backend-auggie-starting = Starting Auggie...
settings-code-indexing-backend-auggie-unavailable = Auggie unavailable
settings-code-indexing-disabled-auggie-unavailable = Start auggie or configure AUGMENT_SESSION_AUTH to use local codebase indexing.
settings-code-codebase-index-description-auggie = Warp builds the local index and retrieves code context through your Auggie MCP server. No data is sent to Warp servers.

# Index folder action button
settings-code-index-new-folder = Index new folder

# Initialized folders section
settings-code-initialized-folders = Initialized / indexed folders
settings-code-no-folders-initialized = No folders have been initialized yet.
settings-code-open-project-rules = Open project rules
settings-code-indexing-section = INDEXING
settings-code-no-index-created = No index created

# Indexing status
settings-code-status-discovered-chunks = Discovered {$count} chunks
settings-code-status-syncing-progress = Syncing - {$completed} / {$total}
settings-code-status-syncing = Syncing...
settings-code-status-synced = Synced
settings-code-status-codebase-too-large = Codebase too large
settings-code-status-stale = Stale
settings-code-status-failed = Failed
settings-code-status-no-index-built = No index built

# Remote indexing status
settings-code-status-remote-index-limit-reached = Index limit reached
settings-code-status-remote-unavailable = Unavailable
settings-code-status-remote-disabled = Disabled
settings-code-status-remote-queued = Queued
settings-code-status-remote-indexing-progress = Indexing - {$completed} / {$total}
settings-code-status-remote-indexing-count = Indexing - {$completed}
settings-code-status-remote-indexing-total = Indexing - 0 / {$total}
settings-code-status-remote-indexing = Indexing...

# Index usage report ("metadata bar" beneath the status row)
settings-code-stats-files = {$count} files
settings-code-stats-chunks = {$count} chunks
settings-code-stats-bytes = {$size}
settings-code-stats-synced = synced {$ago}
settings-code-stats-queried = queried {$count}× · last {$ago}
settings-code-stats-just-now = just now
settings-code-stats-minutes-ago = {$minutes}m ago
settings-code-stats-hours-ago = {$hours}h ago
settings-code-stats-days-ago = {$days}d ago

# Indexing failure reasons (shown in the metadata bar when last sync failed)
settings-code-error-build-tree = Failed to build the index tree.
settings-code-error-too-many-files = Codebase exceeds the file count limit.
settings-code-error-max-depth = Codebase exceeds the maximum directory depth.
settings-code-error-embedding = Failed to generate embeddings for {$count} fragments.
settings-code-error-sync = Failed to sync {$count} intermediate nodes.
settings-code-error-other = {$reason}

# LSP servers section
settings-code-lsp-servers-section = LSP SERVERS
settings-code-lsp-installed = Installed
settings-code-lsp-installing = Installing...
settings-code-lsp-checking = Checking...
settings-code-lsp-available-for-download = Available for download
settings-code-lsp-restart-server = Restart server
settings-code-lsp-view-logs = View logs
settings-code-lsp-state-available = Available
settings-code-lsp-state-busy = Busy
settings-code-lsp-state-failed = Failed
settings-code-lsp-state-stopped = Stopped
settings-code-lsp-state-not-running = Not running

# Editor and Code Review widgets
settings-code-auto-open-review-pane = Auto open code review panel
settings-code-auto-open-review-pane-desc = When this setting is on, the code review panel will open on the first accepted diff of a conversation
settings-code-show-review-button = Show code review button
settings-code-show-review-button-desc = Show a button in the top right of the window to toggle the code review panel.
settings-code-show-diff-stats = Show diff stats on code review button
settings-code-show-diff-stats-desc = Show lines added and removed counts on the code review button.
settings-code-project-explorer = Project explorer
settings-code-project-explorer-desc = Adds an IDE-style project explorer / file tree to the left side tools panel.
settings-code-global-search = Global file search
settings-code-global-search-desc = Adds global file search to the left side tools panel.

# External editor
settings-code-choose-editor-file-links = Choose an editor to open file links
settings-code-choose-editor-code-panels = Choose an editor to open files from the code review panel, project explorer, and global search
settings-code-choose-layout = Choose a layout to open files in Warp
settings-code-group-files-header = Group files into single editor pane
settings-code-group-files-desc = When this setting is on, any files opened in the same tab will be automatically grouped into a single editor pane.
settings-code-open-markdown-viewer = Open Markdown files in Warp's Markdown Viewer by default
