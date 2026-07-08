# 代码评审模块 UI 文案（PR / diff 评审）。
# Key 前缀 `code-review-`，按文件分组：-view-、-comment-、-comment-list-、-diff-menu-、-diff-selector-、-git-、-git-commit-、-git-pr-、-git-push-。
#
# 术语对照：
#   commit → 提交
#   push → 推送
#   pull request / PR → 拉取请求 (按 GitHub 业界惯例 PR/拉取请求 二选一，全文一致)
#   branch → 分支
#   staged → 已暂存
#   diff → 差异
#   review → 评审

# diff_selector.rs（1 条）
code-review-diff-selector-uncommitted-changes = 未提交的更改

# diff_menu.rs（1 条）
code-review-diff-menu-search-placeholder = 搜索要比较的差异集或分支…

# comment_rendering.rs（1 条）
code-review-comment-fallback-title = 评审评论

# comment_list_view.rs（10 条）
code-review-comment-list-cancel = 取消
code-review-comment-list-no-non-outdated = 没有可发送的有效评论
code-review-comment-list-ai-disabled = 必须启用 AI 才能向智能体发送评论
code-review-comment-list-ai-credits-required = 智能体代码评审需要 AI 额度
code-review-comment-list-all-terminals-busy = 所有终端都在忙
code-review-comment-list-send-to-agent-tooltip = 将差异评论发送给智能体
code-review-comment-list-send-to-cli-tooltip = 将差异评论发送给 { $label }
code-review-comment-list-send-to-agent-button = 发送给智能体
code-review-comment-list-menu-copy-text = 复制文本
code-review-comment-list-menu-view-in-github = 在 GitHub 中查看
code-review-comment-list-menu-remove = 移除

# git_dialog/mod.rs（12 条：user_facing_git_error + 对话框标题/按钮 + 分支/更改标签）
code-review-git-error-no-changes = 没有可提交的更改。
code-review-git-error-no-staged-changes = 没有已暂存的更改可提交。
code-review-git-error-identity-not-configured = 未配置 Git 身份。请设置 user.name 和 user.email。
code-review-git-error-pull-before-push = 远程有新更改 — 请先拉取再推送。
code-review-git-error-no-remote = 此分支未配置远程仓库。
code-review-git-error-auth-failed = 认证失败。请检查你的 Git 凭据。
code-review-git-error-network = 网络错误。请检查你的连接。
code-review-git-error-repo-not-found = 未找到远程仓库。
code-review-git-error-gh-cli-missing = 未安装 GitHub CLI（gh）。详见 https://cli.github.com/。
code-review-git-error-gh-not-authenticated = 未登录 GitHub CLI。请运行 `gh auth login`。
code-review-git-error-git-operation-in-progress = 另一个 Git 操作正在进行中。请先完成或中止它。
code-review-git-error-generic = Git 操作失败。
code-review-git-button-confirm = 确认
code-review-git-button-cancel = 取消
code-review-git-title-commit = 提交你的更改
code-review-git-title-publish-branch = 发布分支
code-review-git-title-push-changes = 推送更改
code-review-git-title-create-pr = 创建拉取请求

# git_dialog/push.rs（3 条）
code-review-git-push-button-publish = 发布
code-review-git-push-button-push = 推送
code-review-git-push-loading-label = 正在推送…
code-review-git-push-publishing-label = 正在发布…

# git_dialog/pr.rs（4 条）
code-review-git-pr-button-create = 创建 PR
code-review-git-pr-loading-label = 正在创建…
code-review-git-pr-title-placeholder = PR 标题
code-review-git-pr-body-placeholder = PR 正文（可选）

# git_dialog/commit.rs（8 条）
code-review-git-commit-generating-placeholder = 正在生成提交信息…
code-review-git-commit-fallback-placeholder = 输入提交信息
code-review-git-commit-and-push = 提交并推送
code-review-git-commit-and-publish = 提交并发布
code-review-git-commit-button = 提交
code-review-git-commit-and-create-pr = 提交并创建 PR
code-review-git-commit-tooltip-enter-message = 请输入提交信息
code-review-git-commit-toast-committed = 更改已成功提交。
code-review-git-commit-toast-committed-pushed = 更改已提交并推送。
code-review-git-commit-loading-label = 正在提交…

# git_dialog/push.rs + pr.rs（4 条）
code-review-git-push-toast-published = 分支已成功发布。
code-review-git-push-toast-pushed = 更改已成功推送。
code-review-git-pr-toast-open-link = 打开 PR
code-review-git-pr-toast-created = PR 已成功创建。

# git_dialog/mod.rs（文件数量；中文无单复数变化）
code-review-git-files-count = { $count } 个文件

# code_review_view.rs（主视图，约 50 条）
code-review-view-hide-file-navigation = 隐藏文件导航
code-review-view-show-file-navigation = 显示文件导航
code-review-view-tooltip-view-changes = 查看更改
code-review-view-disabled-remote = 差异仅适用于本地工作区。
code-review-view-disabled-non-git = 差异仅适用于 git 仓库。
code-review-view-disabled-wsl = 差异目前无法在 WSL 中使用。
code-review-view-discard-tooltip-git-busy = 当 git 操作（合并、变基等）进行中时无法丢弃更改
code-review-view-discard-tooltip-no-changes = 没有可丢弃的更改
code-review-view-discard-title-uncommitted = 丢弃未提交的更改？
code-review-view-discard-title-file-uncommitted = 丢弃此文件的所有未提交更改？
code-review-view-discard-title-all-against-branch = 丢弃所有更改？
code-review-view-discard-title-file-against-branch = 丢弃此文件的所有更改？
code-review-view-discard-body-all-uncommitted = 你即将丢弃所有尚未提交的本地更改。
code-review-view-discard-body-file-uncommitted = 这会将此文件恢复到最近一次提交的版本，并丢弃本地修改。
code-review-view-discard-body-all-against-branch = 你即将丢弃所有已提交和未提交的更改。
code-review-view-discard-body-file-against-branch = 这会将此文件恢复到主分支版本，并丢弃所有已提交和未提交的修改。
code-review-view-discard-body-file-against-named-branch = 这会将此文件重置为 { $branch } 分支版本，并丢弃所有已提交和未提交的修改。
code-review-view-tooltip-maximize = 最大化
code-review-view-tooltip-restore = 还原
code-review-view-button-commit = 提交
code-review-view-button-push = 推送
code-review-view-button-undo = 撤销
code-review-view-button-retry = 重试
code-review-view-button-discard-changes = 丢弃更改
code-review-view-button-cancel = 取消
code-review-view-title = 代码评审
code-review-view-button-initialize-codebase = 初始化代码库
code-review-view-tooltip-initialize-codebase = 启用代码库索引与 WARP.md
code-review-view-button-open-repository = 打开仓库
code-review-view-tooltip-open-repository = 导航到一个仓库并为其初始化编程功能
code-review-view-uncommitted-changes-row = 未提交的更改
code-review-view-discard-changes-button = 丢弃更改
code-review-view-tooltip-open-file = 打开文件
code-review-view-tooltip-add-file-diff-context = 将文件差异添加为上下文
code-review-view-tooltip-copy-file-path = 复制文件路径
code-review-view-loading-open-changes = 正在加载已打开的更改...
code-review-view-toast-comments-sent = 评论已发送给智能体
code-review-view-toast-comments-error = 无法将评论提交给智能体
code-review-view-stash-changes = 暂存更改
code-review-view-diff-removed = 差异已移除
code-review-view-cannot-attach-terminal-busy = 终端正在运行时无法附加上下文
code-review-view-cannot-attach-input-unavailable = 输入不可用时无法附加差异
code-review-view-button-create-pr = 创建 PR
code-review-view-button-publish = 发布
code-review-view-tooltip-no-changes-to-commit = 没有可提交的更改
code-review-view-tooltip-commit-changes = 在本地提交更改
code-review-view-tooltip-no-git-actions = 没有可用的 git 操作
code-review-view-menu-commit = 提交
code-review-view-menu-push = 推送
code-review-view-menu-publish = 发布
code-review-view-menu-create-pr = 创建 PR
code-review-view-menu-add-diff-context = 将差异集添加为上下文
code-review-view-menu-show-comment = 显示已保存的评论
code-review-view-menu-add-comment = 添加评论
code-review-view-menu-discard-all = 全部丢弃
code-review-view-header-reviewing = 正在评审代码更改
code-review-view-toast-repo-changed = 代码评审：仓库已变更。分支列表已清除。
code-review-view-repo-initialized-with-file = 仓库已使用 { $file_name } 文件初始化。
code-review-view-unsaved-file-tooltip = 此文件有未保存的更改。按 { $shortcut } 保存
