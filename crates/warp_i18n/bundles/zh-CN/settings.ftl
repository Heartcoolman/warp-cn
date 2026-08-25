# 设置页顶层导航标签和公共 UI 字符串。
# Keys MUST start with `settings-` (spec: client-string-localization, Bundle namespace).

# 顶层导航标签。
settings-section-title = 设置
settings-section-account = 账户
settings-section-billing-usage = 计费与用量
settings-section-teams = 团队
settings-section-appearance = 外观
settings-section-features = 功能
settings-section-keybindings = 键盘快捷键
settings-section-warpify = Warpify
settings-section-scripting = 脚本
settings-section-referrals = 邀请有礼
settings-section-shared-blocks = 共享区块
settings-section-warp-drive = Warp 云盘
settings-section-mcp-servers = MCP 服务器
settings-section-privacy = 隐私
settings-section-about = 关于

# 分组标签。
settings-umbrella-agents = 智能体
settings-umbrella-code = 代码
settings-umbrella-cloud-platform = 云平台

# 子页标签。
settings-subpage-warp-agent = Warp 智能体
settings-subpage-agent-profiles = 配置文件
settings-subpage-agent-mcp-servers = MCP 服务器
settings-subpage-knowledge = 知识库
settings-subpage-third-party-cli-agents = 第三方 CLI 智能体
settings-subpage-code-indexing = 索引与项目
settings-subpage-editor-code-review = 编辑器与代码审查
settings-subpage-cloud-environments = 环境
settings-subpage-warp-cloud-agent-api-keys = API 密钥

# 公共设置页 UI。
settings-search-placeholder = 搜索设置
settings-close-button = 关闭
settings-reset-default = 恢复默认
settings-unsaved-changes = 有未保存的更改
settings-save-changes = 保存更改
settings-discard-changes = 放弃更改

# 搜索空状态。
settings-search-no-results = 未找到匹配的设置项。
settings-search-no-results-hint = 可尝试更换关键词，或检查是否有拼写错误。

# 智能体辅助环境弹窗。
settings-agent-env-no-repos-selected = 尚未选择任何仓库
settings-agent-env-all-repos-selected = 所有已本地索引的仓库都已被选中。

# 设置导入流程
settings-import-restart-hint = 部分设置将在你打开新会话后生效。

# 环境页面
settings-environments-new-button = 新建环境

# MCP 编辑页
settings-mcp-edit-format-json = JSON

# 上下文菜单 - 窗格操作
settings-menu-split-right = 向右拆分窗格
settings-menu-split-left = 向左拆分窗格
settings-menu-split-down = 向下拆分窗格
settings-menu-split-up = 向上拆分窗格
settings-menu-close-pane = 关闭窗格

# 命令面板：切换设置动作对的拼接标签。
# `{$suffix}` 是每个开关的本地化描述。
settings-action-enable-suffix = 启用 {$suffix}
settings-action-disable-suffix = 禁用 {$suffix}

# 开发者命令面板 suffix（用于 settings_view/mod.rs init_actions_from_parent_view）
settings-cmd-suffix-recording-mode = 录制模式
settings-cmd-suffix-in-band-generators = 新会话的内联生成器
settings-cmd-suffix-debug-network-status = 调试网络状态
settings-cmd-suffix-memory-statistics = 内存统计

# settings_page.rs（共享基础设施）
settings-page-tooltip-default-learn-more = 点击在文档中了解更多
settings-page-tooltip-local-only-default = 此设置不会同步到你的其他设备
settings-page-button-reset-to-default = 恢复默认

# settings_file_footer.rs
settings-file-footer-open-settings-file = 打开设置文件
settings-file-footer-alert-open-file = 打开文件
settings-file-footer-alert-fix-with-oz = 用 Oz 修复
