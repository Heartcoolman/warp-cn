# 代码设置页字符串（索引与项目、编辑器与代码审查）。
# Keys MUST start with `settings-code-`.

settings-code-title = 代码

# Subpage titles
settings-code-subpage-indexing = 代码库索引
settings-code-subpage-editor-review = 编辑器与代码审查

# Categories
settings-code-category-indexing = 代码库索引
settings-code-category-editor-review = 代码编辑器与审查

# Initialization Settings section
settings-code-init-settings-header = 初始化设置
settings-code-codebase-indexing-label = 代码库索引
settings-code-codebase-index-description = Warp 可以在你浏览代码仓库时自动建立索引，帮助 Agent 快速理解上下文并提供解决方案。代码不会被存储到服务器。如果某个代码库无法建立索引，Warp 仍可通过 grep 和 find 等工具浏览代码并获取信息。
settings-code-warp-indexing-ignore-description = 若要将特定文件或目录排除在索引之外，请将其加入仓库目录下的 .warpindexingignore 文件。这些文件仍可供 AI 功能访问，但不会包含在代码库嵌入中。
settings-code-auto-index-feature-name = 默认对新文件夹建立索引
settings-code-auto-index-description = 启用后，Warp 会在你浏览代码仓库时自动建立索引，帮助 Agent 快速理解上下文并提供针对性的解决方案。
settings-code-indexing-disabled-admin = 团队管理员已禁用代码库索引。
settings-code-indexing-disabled-admin-team = 代码库索引不可用：{ $team } 已将其禁用。
settings-code-indexing-workspace-enabled-admin = 团队管理员已启用代码库索引。
settings-code-indexing-disabled-global-ai = 必须启用 AI 功能才能使用代码库索引。
settings-code-index-limit-reached = 你已达到当前套餐允许的代码库索引数量上限。请删除已有索引后再为新代码库自动建立索引。

# Auggie 后端（warp-cn）
settings-code-indexing-backend-cloud = 后端：Warp Cloud
settings-code-indexing-backend-auggie = 后端：Auggie（本地）
settings-code-indexing-backend-auggie-running = 正在使用 Auggie MCP
settings-code-indexing-backend-auggie-starting = 正在启动 Auggie…
settings-code-indexing-backend-auggie-unavailable = Auggie 不可用
settings-code-indexing-disabled-auggie-unavailable = 请启动 auggie，或配置 AUGMENT_SESSION_AUTH 后使用本地代码库索引。
settings-code-codebase-index-description-auggie = Warp 在本地建立索引，并通过你的 Auggie MCP 服务器检索代码上下文。不向 Warp 服务端发送任何数据。

# Index folder action button
settings-code-index-new-folder = 索引新文件夹

# Initialized folders section
settings-code-initialized-folders = 已初始化 / 已索引的文件夹
settings-code-no-folders-initialized = 还没有已初始化的文件夹。
settings-code-open-project-rules = 打开项目规则
settings-code-indexing-section = 索引
settings-code-no-index-created = 未创建索引

# Indexing status
settings-code-status-discovered-chunks = 已发现 {$count} 个片段
settings-code-status-syncing-progress = 同步中 - {$completed} / {$total}
settings-code-status-syncing = 同步中…
settings-code-status-synced = 已同步
settings-code-status-codebase-too-large = 代码库过大
settings-code-status-stale = 已过期
settings-code-status-failed = 失败
settings-code-status-no-index-built = 未建立索引

# 远程索引状态
settings-code-status-remote-index-limit-reached = 已达索引上限
settings-code-status-remote-unavailable = 不可用
settings-code-status-remote-disabled = 已禁用
settings-code-status-remote-queued = 排队中
settings-code-status-remote-indexing-progress = 索引中 - {$completed} / {$total}
settings-code-status-remote-indexing-count = 索引中 - {$completed}
settings-code-status-remote-indexing-total = 索引中 - 0 / {$total}
settings-code-status-remote-indexing = 索引中…

# 索引使用情况报告（状态行下方的统计条）
settings-code-stats-files = {$count} 个文件
settings-code-stats-chunks = {$count} 个片段
settings-code-stats-bytes = {$size}
settings-code-stats-synced = 同步于 {$ago}
settings-code-stats-queried = 已查询 {$count} 次 · 最近 {$ago}
settings-code-stats-just-now = 刚刚
settings-code-stats-minutes-ago = {$minutes} 分钟前
settings-code-stats-hours-ago = {$hours} 小时前
settings-code-stats-days-ago = {$days} 天前

# 上次同步失败原因（取代统计条显示）
settings-code-error-build-tree = 索引构建失败。
settings-code-error-too-many-files = 代码库超出文件数上限。
settings-code-error-max-depth = 代码库目录层级过深。
settings-code-error-embedding = 有 {$count} 个片段嵌入生成失败。
settings-code-error-sync = 有 {$count} 个中间节点同步失败。
settings-code-error-other = {$reason}

# LSP servers section
settings-code-lsp-servers-section = LSP 服务器
settings-code-lsp-installed = 已安装
settings-code-lsp-installing = 安装中…
settings-code-lsp-checking = 检查中…
settings-code-lsp-available-for-download = 可下载
settings-code-lsp-restart-server = 重启服务器
settings-code-lsp-view-logs = 查看日志
settings-code-lsp-state-available = 可用
settings-code-lsp-state-busy = 忙碌
settings-code-lsp-state-failed = 失败
settings-code-lsp-state-stopped = 已停止
settings-code-lsp-state-not-running = 未运行

# Editor and Code Review widgets
settings-code-auto-open-review-pane = 自动打开代码审查面板
settings-code-auto-open-review-pane-desc = 启用后，会在会话首次接受 diff 时自动打开代码审查面板。
settings-code-show-review-button = 显示代码审查按钮
settings-code-show-review-button-desc = 在窗口右上角显示一个按钮，用于切换代码审查面板。
settings-code-show-diff-stats = 在代码审查按钮上显示 diff 统计
settings-code-show-diff-stats-desc = 在代码审查按钮上显示新增与删除的行数。
settings-code-project-explorer = 项目浏览器
settings-code-project-explorer-desc = 在左侧工具面板加入 IDE 风格的项目浏览器 / 文件树。
settings-code-global-search = 全局文件搜索
settings-code-global-search-desc = 在左侧工具面板加入全局文件搜索。

# 外部编辑器
settings-code-choose-editor-file-links = 选择打开文件链接的编辑器
settings-code-choose-editor-code-panels = 选择从代码审查面板、项目浏览器和全局搜索打开文件的编辑器
settings-code-choose-layout = 选择在 Warp 中打开文件的布局
settings-code-group-files-header = 将文件归组到单个编辑器窗格
settings-code-group-files-desc = 启用后，在同一标签页中打开的文件将自动归组到单个编辑器窗格。
settings-code-open-markdown-viewer = 默认在 Warp 的 Markdown 查看器中打开 Markdown 文件
