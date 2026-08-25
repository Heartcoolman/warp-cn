# 云平台设置页字符串（环境、Oz Cloud API 密钥）。
# Keys MUST start with `settings-platform-`.

settings-cloud-title = 云平台
settings-platform-new-api-key = 新建 API 密钥
settings-platform-save-your-key = 保存密钥
settings-platform-never = 从未
settings-platform-no-api-keys = 无 API 密钥
settings-platform-create-key-description = 创建密钥以管理对 Warp 的外部访问
settings-platform-documentation-link = 文档。
settings-platform-col-name = 名称
settings-platform-col-key = 密钥
settings-platform-col-scope = 范围
settings-platform-col-created = 创建时间
settings-platform-col-last-used = 最后使用
settings-platform-col-expires-at = 过期时间
settings-platform-scope-personal = 个人
settings-platform-scope-team = 团队
settings-platform-scope-agent = 智能体

settings-environments-no-match = 没有匹配您搜索的环境。
settings-environments-empty-title = 您尚未设置任何环境。
settings-environments-setup-prompt = 选择您希望如何设置环境：

settings-platform-api-key-secret-info = 该密钥仅显示一次，请复制并妥善保管。
settings-platform-api-key-name-label = 名称
settings-platform-api-key-type-label = 类型
settings-platform-api-key-agent-label = 智能体
settings-platform-api-key-expiration-label = 过期时间
settings-platform-no-agents-available = 暂无可用智能体，请先创建一个。
settings-platform-api-key-deleted = API 密钥已删除
settings-platform-create-api-key-help-line1 = 创建并管理 API 密钥，以允许云端智能体访问你的 Warp 账户。
settings-platform-create-api-key-help-line2-prefix = 详细信息请见

# 环境表单
settings-environments-form-setup-help = 设置命令会独立运行。每条命令都在工作区根目录（/workspace）下执行。若命令之间存在依赖，请用 && 连接。
settings-environments-form-description-label = 描述
settings-environments-form-repos-label = 仓库
settings-environments-form-loading = 加载中...
settings-environments-form-auth-with-github = 使用 GitHub 授权
settings-environments-form-retry = 重试
settings-environments-form-repo-input-help = 输入 owner/repo 并按回车添加，或从下拉框选择。
settings-environments-form-missing-repo = 缺少仓库？
settings-environments-form-configure-on-github = 在 GitHub 上配置访问权限
settings-environments-form-no-repos-found = 未找到仓库

# 环境页（列表 / 空状态 / 卡片）
settings-environments-page-title = 环境
settings-environments-page-description = 环境定义了你的 ambient 智能体在何处运行。可在数分钟内通过 GitHub（推荐）、Warp 辅助设置或手动配置完成搭建。
settings-environments-list-search-placeholder = 搜索环境...
settings-environments-empty-button-loading = 加载中...
settings-environments-empty-button-retry = 重试
settings-environments-empty-button-authorize = 授权
settings-environments-empty-button-get-started = 开始使用
settings-environments-empty-button-launch-agent = 启动智能体
settings-environments-empty-quick-setup-title = 快速设置
settings-environments-empty-quick-setup-badge = 推荐
settings-environments-empty-quick-setup-subtitle = 选择你想要使用的 GitHub 仓库，我们会为你推荐基础镜像和配置
settings-environments-empty-use-agent-title = 使用智能体
settings-environments-empty-use-agent-subtitle = 选择一个本地已配置的项目，我们将基于它帮你搭建环境
settings-environments-section-personal = 个人
settings-environments-section-shared-by-team = 由 Warp 和 {$team} 共享
settings-environments-section-shared-by-team-default = 由 Warp 和你的团队共享
settings-environments-card-env-id-prefix = 环境 ID
settings-environments-card-image-prefix = 镜像
settings-environments-card-repos-prefix = 仓库
settings-environments-card-setup-commands-prefix = 设置命令
settings-environments-card-last-edited = 上次编辑：{$time}
settings-environments-card-last-used = 上次使用：{$time}
settings-environments-card-last-used-never = 从未使用
settings-environments-card-tooltip-share = 共享
settings-environments-card-tooltip-edit = 编辑
settings-environments-card-link-view-runs = 查看我的运行记录
settings-environments-toast-updated = 已成功更新环境
settings-environments-toast-created = 已成功创建环境
settings-environments-toast-deleted = 已成功删除环境
settings-environments-toast-shared = 已成功共享环境
settings-environments-toast-share-failed = 共享环境到团队失败
settings-environments-toast-create-not-logged-in = 无法创建环境：尚未登录。
settings-environments-toast-save-not-exists = 无法保存：环境已不存在。
settings-environments-toast-share-not-on-team = 无法共享环境：你目前不在任何团队中。
settings-environments-toast-share-not-synced = 无法共享环境：环境尚未同步。

# update_environment_form.rs
settings-environments-update-form-name-placeholder = 环境名称
settings-environments-update-form-docker-image-placeholder = 例如 python:3.11, node:20-alpine
settings-environments-update-form-repos-placeholder-authed = 输入仓库（格式：owner/repo）
settings-environments-update-form-repos-placeholder-unauthed = 粘贴仓库 URL
settings-environments-update-form-button-create = 创建
settings-environments-update-form-button-save = 保存
settings-environments-update-form-button-create-environment = 创建环境
settings-environments-update-form-button-save-environment = 保存环境
settings-environments-update-form-button-edit-environment = 编辑环境
settings-environments-update-form-button-delete-environment = 删除环境
settings-environments-update-form-error-load-github-repos = 无法加载 GitHub 仓库。你可以粘贴仓库 URL，或重试。
settings-environments-update-form-error-load-github-repos-with-error = 加载 GitHub 仓库失败：{$error}
settings-environments-update-form-error-load-github-repositories = 加载 GitHub 仓库失败
settings-environments-update-form-error-suggest-docker-image = 推荐 Docker 镜像失败
settings-environments-update-form-error-suggest-docker-image-with-error = 推荐 Docker 镜像失败：{$error}
settings-environments-update-form-error-suggest-unknown-response = suggestCloudEnvironmentImage 返回了未知响应
settings-environments-update-form-share-with-team = 与团队共享
settings-environments-update-form-personal-warning = 个人环境无法使用外部集成或团队 API 密钥。建议改用共享环境以获得最佳体验。
settings-environments-update-form-label-name = 名称
settings-environments-update-form-label-setup-commands = 安装命令
settings-environments-update-form-label-docker-image-reference = Docker 镜像引用
settings-environments-update-form-suggest-image-generating = 生成中…
settings-environments-update-form-suggest-image-button = 推荐镜像
settings-environments-update-form-suggest-image-tooltip = Warp 将根据所选仓库推荐 Docker 镜像。
settings-environments-update-form-button-authenticate = 授权
settings-environments-update-form-grant-github-access = 你需要授权访问 GitHub 仓库才能让 Warp 推荐 Docker 镜像
settings-environments-update-form-button-launch-agent = 启动 Agent
settings-environments-update-form-no-good-match = 未找到合适的匹配。建议为这些仓库使用自定义 Docker 镜像。

# delete_environment_confirmation_dialog.rs
settings-environments-delete-dialog-title = 删除环境？
settings-environments-delete-dialog-description = 确定要删除 {$name} 环境吗？
settings-environments-delete-dialog-cancel = 取消
settings-environments-delete-dialog-confirm = 删除环境

# transfer_ownership_confirmation_modal.rs
settings-environments-transfer-ownership-modal-description = 确定要将团队所有权转交给 {$email} 吗？转交后你将不再是所有者，无法对该团队执行任何管理操作。
settings-environments-transfer-ownership-modal-cancel = 取消
settings-environments-transfer-ownership-modal-confirm = 转交

# agent_assisted_environment_modal.rs
settings-environments-agent-assisted-modal-add-repo = 添加仓库
settings-environments-agent-assisted-modal-cancel = 取消
settings-environments-agent-assisted-modal-create-environment = 创建环境
settings-environments-agent-assisted-modal-section-selected-repos = 已选仓库
settings-environments-agent-assisted-modal-section-available-repos = 可用的已索引仓库
settings-environments-agent-assisted-modal-loading-indexed-repos = 加载本地已索引仓库中…
settings-environments-agent-assisted-modal-no-indexed-repos = 暂无已索引的本地仓库。请先索引一个仓库后再试。
settings-environments-agent-assisted-modal-unavailable = 此构建版本不支持本地仓库选择。
settings-environments-agent-assisted-modal-description-indexed = 选择本地已索引的仓库，为环境创建 Agent 提供上下文。
settings-environments-agent-assisted-modal-description-default = 选择仓库为环境创建 Agent 提供上下文。
settings-environments-agent-assisted-modal-title = 为环境选择仓库
settings-platform-custom-endpoint-description = 在下方填写端点信息。你可以从该端点添加任意数量的模型，也可以为模型选择器中的输入设置别名。
settings-platform-no-api-keys-search-match = 没有与搜索匹配的 API 密钥
