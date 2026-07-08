# 键盘绑定命令名（命令面板 + 键盘快捷键设置页）。
# Key 前缀 `binding-`，按源模块分组：`binding-<scope>-<name>`。

# pane_group/mod.rs（14 条命令）
binding-pane-group-close-current-session = 关闭当前会话
binding-pane-group-add-left = 向左拆分窗格
binding-pane-group-add-right = 向右拆分窗格
binding-pane-group-add-up = 向上拆分窗格
binding-pane-group-add-down = 向下拆分窗格
binding-pane-group-navigate-left = 切换到左侧窗格
binding-pane-group-navigate-right = 切换到右侧窗格
binding-pane-group-navigate-up = 切换到上方窗格
binding-pane-group-navigate-down = 切换到下方窗格
binding-pane-group-resize-left = 调整窗格 > 向左移动分隔条
binding-pane-group-resize-right = 调整窗格 > 向右移动分隔条
binding-pane-group-resize-up = 调整窗格 > 向上移动分隔条
binding-pane-group-resize-down = 调整窗格 > 向下移动分隔条
binding-pane-group-toggle-maximize-pane = 切换最大化当前窗格

# terminal/input.rs（12 条命令；line 1782 edit-prompt 已使用 t!）
binding-input-show-history = 显示历史
binding-input-show-network-log = 显示 Warp 网络日志
binding-input-clear-screen = 清屏
binding-input-toggle-classic-completions = （实验）切换经典补全模式
binding-input-command-search = 命令搜索
binding-input-history-search = 历史搜索
binding-input-open-completions = 打开补全菜单
binding-input-toggle-workflows = 工作流
binding-input-ai-command-suggestions = 打开 AI 命令建议
binding-input-new-agent-conversation = 新建智能体对话
binding-input-trigger-auto-detection = 触发自动检测
binding-input-clear-ai-context-menu-query = 清除并重置 AI 上下文菜单查询

# code/editor/view/actions.rs（约 32 EditableBinding + 7 FixedBinding::custom）
# 剪贴板 / 撤销重做（FixedBinding::custom）
binding-code-editor-copy = 复制
binding-code-editor-paste = 粘贴
binding-code-editor-cut = 剪切
binding-code-editor-undo = 撤销
binding-code-editor-redo = 重做
# 按词移动
binding-code-editor-move-backward-one-word = 向左移动一个单词
binding-code-editor-move-forward-one-word = 向右移动一个单词
# 光标移动
binding-code-editor-up = 光标上移
binding-code-editor-down = 光标下移
binding-code-editor-left = 光标左移
binding-code-editor-right = 光标右移
binding-code-editor-move-to-line-start = 移到行首
binding-code-editor-home = 行首
binding-code-editor-move-to-line-end = 移到行尾
binding-code-editor-end = 行尾
binding-code-editor-cursor-at-buffer-start = 光标移到缓冲区开始
binding-code-editor-cursor-at-buffer-end = 光标移到缓冲区结尾
# 选择
binding-code-editor-select-left-by-word = 向左选择一个单词
binding-code-editor-select-right-by-word = 向右选择一个单词
binding-code-editor-select-left = 向左选择一个字符
binding-code-editor-select-right = 向右选择一个字符
binding-code-editor-select-up = 向上选择
binding-code-editor-select-down = 向下选择
binding-code-editor-select-all = 全选
binding-code-editor-select-to-line-start = 选择到行首
binding-code-editor-select-to-line-end = 选择到行尾
# 文本操作
binding-code-editor-backspace = 删除前一个字符
binding-code-editor-toggle-comment = 切换注释
binding-code-editor-delete = 删除
binding-code-editor-cut-word-left = 剪切左侧单词
binding-code-editor-delete-word-left = 删除左侧单词
binding-code-editor-cut-word-right = 剪切右侧单词
binding-code-editor-delete-word-right = 删除右侧单词
binding-code-editor-cut-all-left = 剪切光标左侧所有内容
binding-code-editor-delete-all-left = 删除光标左侧所有内容
binding-code-editor-cut-all-right = 剪切光标右侧所有内容
binding-code-editor-delete-all-right = 删除光标右侧所有内容
# Vim / 查找 / 跳转
binding-code-editor-vim-exit-insert-mode = 退出 Vim 插入模式
binding-code-editor-find = 在代码编辑器中查找
binding-code-editor-go-to-line = 跳转到行

# terminal/view/init.rs（72 条 EditableBinding 命令）
binding-terminal-view-alternate-terminal-paste = 备用终端粘贴
binding-terminal-view-toggle-cli-agent-rich-input = 切换 CLI Agent 富输入
binding-terminal-view-warpify-subshell = Warp 化子 Shell
binding-terminal-view-warpify-ssh-session = Warp 化 SSH 会话
binding-terminal-view-accept-prompt-suggestion = 接受提示建议
binding-terminal-view-cancel-active-process-mac-linux = 取消当前进程
binding-terminal-view-cancel-active-process-windows = 复制文本或取消当前进程
binding-terminal-view-focus-input = 聚焦终端输入
binding-terminal-view-paste = 粘贴
binding-terminal-view-copy = 复制
binding-terminal-view-reinput-commands = 重新输入所选命令
binding-terminal-view-reinput-commands-with-sudo = 以 root 重新输入所选命令
binding-terminal-view-find-in-terminal = 在终端中查找
binding-terminal-view-select-bookmark-up = 选择上方最近的书签
binding-terminal-view-select-bookmark-down = 选择下方最近的书签
binding-terminal-view-open-block-list-context-menu = 打开块上下文菜单
binding-terminal-view-toggle-teams-modal = 切换团队工作流弹窗
binding-terminal-view-copy-git-branch = 复制 Git 分支
binding-terminal-view-clear-blocks = 清空块
binding-terminal-view-executing-command-move-word-left = 在执行中的命令内向左移动一个单词
binding-terminal-view-executing-command-move-word-right = 在执行中的命令内向右移动一个单词
binding-terminal-view-executing-command-move-home = 在执行中的命令内移到行首
binding-terminal-view-executing-command-move-end = 在执行中的命令内移到行尾
binding-terminal-view-executing-command-delete-word-left = 在执行中的命令内删除左侧单词
binding-terminal-view-executing-command-delete-line-start = 在执行中的命令内删除到行首
binding-terminal-view-executing-command-delete-line-end = 在执行中的命令内删除到行尾
binding-terminal-view-backward-tabulation = 在执行中的命令内反向制表
binding-terminal-view-select-previous-block = 选择上一个块
binding-terminal-view-select-next-block = 选择下一个块
binding-terminal-view-share-selected-block = 分享所选块
binding-terminal-view-bookmark-selected-block = 收藏所选块
binding-terminal-view-find-within-selected-block = 在所选块中查找
binding-terminal-view-copy-command-and-output = 复制命令与输出
binding-terminal-view-copy-command-output = 复制命令输出
binding-terminal-view-copy-command = 复制命令
binding-terminal-view-scroll-up-one-line = 终端输出向上滚动一行
binding-terminal-view-scroll-down-one-line = 终端输出向下滚动一行
binding-terminal-view-scroll-to-top-of-selected-block = 滚动到所选块顶部
binding-terminal-view-scroll-to-bottom-of-selected-block = 滚动到所选块底部
binding-terminal-view-select-all-blocks = 全选块
binding-terminal-view-expand-block-selection-above = 向上扩展所选块
binding-terminal-view-expand-block-selection-below = 向下扩展所选块
binding-terminal-view-ask-ai-about-selection = 让 Warp AI 解读选区
binding-terminal-view-ask-ai-last-block = 让 Warp AI 解读最后一个块
binding-terminal-view-ask-ai = 询问 Warp AI
binding-terminal-view-insert-command-correction = 插入命令更正
binding-terminal-view-debug-onboarding-warpinput-project = [调试] 引导提示：WarpInput - 项目
binding-terminal-view-debug-onboarding-warpinput-no-project = [调试] 引导提示：WarpInput - 无项目
binding-terminal-view-debug-onboarding-modality-project = [调试] 引导提示：模式 - 项目
binding-terminal-view-debug-onboarding-modality-no-project = [调试] 引导提示：模式 - 无项目
binding-terminal-view-debug-onboarding-modality-terminal = [调试] 引导提示：模式 - 终端
binding-terminal-view-import-external-settings = 导入外部设置
binding-terminal-view-share-current-session = 分享当前会话
binding-terminal-view-stop-sharing-current-session = 停止分享当前会话
binding-terminal-view-toggle-block-filter = 切换所选或最后一个块的块过滤
binding-terminal-view-toggle-snackbar = 切换当前窗格的常驻命令头
binding-terminal-view-toggle-autoexecute-mode = 切换自动执行模式
binding-terminal-view-toggle-queue-next-prompt = 切换排队下一条提示
binding-terminal-view-debug-generate-codebase-index = [调试] 生成代码库索引
binding-terminal-view-load-agent-mode-conversation = 加载 Agent 模式对话（来自剪贴板中的调试链接）
binding-terminal-view-toggle-session-recording = 切换会话 PTY 录制
binding-terminal-view-set-input-mode-agent = 输入模式切换为 Agent 模式
binding-terminal-view-set-input-mode-terminal = 输入模式切换为终端模式
binding-terminal-view-toggle-hide-cli-responses = 切换隐藏 CLI 响应

# notebooks/editor/view.rs（约 51 条；笔记本/评论的富文本编辑器）
# 剪贴板 / 撤销重做（FixedBinding::custom，与代码编辑器语义一致）
binding-notebooks-editor-copy = 复制
binding-notebooks-editor-paste = 粘贴
binding-notebooks-editor-cut = 剪切
binding-notebooks-editor-undo = 撤销
binding-notebooks-editor-redo = 重做
# 命令选择
binding-notebooks-editor-deselect-command = 取消选择 Shell 命令
binding-notebooks-editor-select-command = 选择光标处的 Shell 命令
binding-notebooks-editor-select-previous-command = 选择上一个命令
binding-notebooks-editor-select-next-command = 选择下一个命令
binding-notebooks-editor-run-commands = 运行所选命令
# 调试
binding-notebooks-editor-toggle-debug-mode = 切换富文本调试模式
binding-notebooks-editor-debug-copy-buffer = 复制富文本缓冲区
binding-notebooks-editor-debug-copy-selection = 复制富文本选区
binding-notebooks-editor-log-state = 记录编辑器状态
# 按词移动
binding-notebooks-editor-move-backward-one-word = 向左移动一个单词
binding-notebooks-editor-move-forward-one-word = 向右移动一个单词
# 光标移动
binding-notebooks-editor-up = 光标上移
binding-notebooks-editor-down = 光标下移
binding-notebooks-editor-left = 光标左移
binding-notebooks-editor-right = 光标右移
binding-notebooks-editor-move-to-paragraph-start = 移到段落开始
binding-notebooks-editor-home = 行首
binding-notebooks-editor-move-to-paragraph-end = 移到段落结尾
binding-notebooks-editor-end = 行尾
# 选择
binding-notebooks-editor-select-left-by-word = 向左选择一个单词
binding-notebooks-editor-select-right-by-word = 向右选择一个单词
binding-notebooks-editor-select-left = 向左选择一个字符
binding-notebooks-editor-select-right = 向右选择一个字符
binding-notebooks-editor-select-up = 向上选择
binding-notebooks-editor-select-down = 向下选择
binding-notebooks-editor-select-all = 全选
binding-notebooks-editor-select-to-paragraph-start = 选择到段落开始
binding-notebooks-editor-select-to-paragraph-end = 选择到段落结尾
binding-notebooks-editor-select-to-line-end = 选择到行尾
binding-notebooks-editor-select-to-line-start = 选择到行首
# 文本操作
binding-notebooks-editor-backspace = 删除前一个字符
binding-notebooks-editor-delete = 删除
binding-notebooks-editor-cut-word-left = 剪切左侧单词
binding-notebooks-editor-delete-word-left = 删除左侧单词
binding-notebooks-editor-cut-word-right = 剪切右侧单词
binding-notebooks-editor-delete-word-right = 删除右侧单词
binding-notebooks-editor-cut-all-left = 剪切光标左侧所有内容
binding-notebooks-editor-delete-all-left = 删除光标左侧所有内容
binding-notebooks-editor-cut-all-right = 剪切光标右侧所有内容
binding-notebooks-editor-delete-all-right = 删除光标右侧所有内容
# 富文本样式
binding-notebooks-editor-edit-link = 创建或编辑链接
binding-notebooks-editor-inline-code = 切换内联代码样式
binding-notebooks-editor-strikethrough = 切换删除线样式
binding-notebooks-editor-underline = 切换下划线样式
# 查找栏
binding-notebooks-editor-find = 在笔记本中查找
binding-notebooks-editor-next-find-match = 聚焦下一个匹配
binding-notebooks-editor-previous-find-match = 聚焦上一个匹配
binding-notebooks-editor-toggle-regex-find = 切换正则表达式搜索
binding-notebooks-editor-toggle-case-sensitive-find = 切换区分大小写搜索

# editor/view/mod.rs（约 60 EditableBinding + 6 FixedBinding::custom；终端命令编辑器）
# 剪贴板 / 撤销重做（FixedBinding::custom）
binding-editor-view-copy = 复制
binding-editor-view-cut = 剪切
binding-editor-view-paste = 粘贴
binding-editor-view-undo = 撤销
binding-editor-view-redo = 重做
# 选择
binding-editor-view-select-left-by-word = 向左选择一个单词
binding-editor-view-select-right-by-word = 向右选择一个单词
binding-editor-view-select-left = 向左选择一个字符
binding-editor-view-select-right = 向右选择一个字符
binding-editor-view-select-up = 向上选择
binding-editor-view-select-down = 向下选择
binding-editor-view-select-all = 全选
binding-editor-view-select-to-line-start = 选择到行首
binding-editor-view-select-to-line-end = 选择到行尾
binding-editor-view-select-left-by-subword = 向左选择一个子词
binding-editor-view-select-right-by-subword = 向右选择一个子词
binding-editor-view-clear-and-copy-lines = 复制并清除所选行
binding-editor-view-add-next-occurrence = 选择下一处匹配
# 光标移动
binding-editor-view-up = 光标上移
binding-editor-view-down = 光标下移
binding-editor-view-left = 光标左移
binding-editor-view-right = 光标右移
binding-editor-view-move-to-line-start = 移到行首
binding-editor-view-move-to-line-end = 移到行尾
binding-editor-view-home = 行首
binding-editor-view-end = 行尾
binding-editor-view-cmd-down = 光标移到末尾
binding-editor-view-cmd-up = 光标移到开头
binding-editor-view-move-to-and-select-buffer-start = 选择并移到顶部
binding-editor-view-move-to-and-select-buffer-end = 选择并移到底部
binding-editor-view-move-forward-one-word = 向右移动一个单词
binding-editor-view-move-backward-one-word = 向左移动一个单词
binding-editor-view-move-forward-one-subword = 向右移动一个子词
binding-editor-view-move-backward-one-subword = 向左移动一个子词
binding-editor-view-move-to-paragraph-start = 移到段落开始
binding-editor-view-move-to-paragraph-end = 移到段落结尾
binding-editor-view-move-to-buffer-start = 移到缓冲区开始
binding-editor-view-move-to-buffer-end = 移到缓冲区结尾
# 缓冲区修改
binding-editor-view-backspace = 删除前一个字符
binding-editor-view-cut-word-left = 剪切左侧单词
binding-editor-view-delete-word-left = 删除左侧单词
binding-editor-view-cut-word-right = 剪切右侧单词
binding-editor-view-delete = 删除
binding-editor-view-delete-word-right = 删除右侧单词
binding-editor-view-clear-lines = 清除所选行
binding-editor-view-cut-all-right = 剪切光标右侧所有内容
binding-editor-view-delete-all-right = 删除光标右侧所有内容
binding-editor-view-delete-all-left = 删除光标左侧所有内容
binding-editor-view-insert-newline = 插入换行
# 折叠
binding-editor-view-fold = 折叠
binding-editor-view-unfold = 展开
binding-editor-view-fold-selected-ranges = 折叠所选范围
# 杂项
binding-editor-view-insert-last-word-previous-command = 插入上一条命令的最后一个单词
binding-editor-view-accept-autosuggestion = 接受自动建议
binding-editor-view-inspect-command = 检查命令
binding-editor-view-clear-buffer = 清空命令编辑器
binding-editor-view-add-cursor-above = 在上方添加光标
binding-editor-view-add-cursor-below = 在下方添加光标
binding-editor-view-insert-nonexpanding-space = 插入非展开空格
binding-editor-view-vim-exit-insert-mode = 退出 Vim 插入模式

# workspace/mod.rs（约 67 条字面量 description；其余 binding 已使用 t! 路径）
# 调试绑定（受 ChannelState::enable_debug_features 控制）
binding-workspace-crash-mac = 让应用崩溃（用于测试 sentry-cocoa）
binding-workspace-crash-other = 让应用崩溃（用于测试 sentry-native）
binding-workspace-log-review-comment-send-status = [调试] 记录当前标签的 review 评论发送状态
binding-workspace-panic = 触发 panic（用于测试 sentry-rust）
binding-workspace-open-view-tree-debugger = 打开视图树调试器
binding-workspace-debug-fte-view = [调试] 查看首次用户体验
binding-workspace-debug-build-plan-migration-modal = [调试] 打开 Build 方案迁移弹窗
binding-workspace-debug-reset-build-plan-migration = [调试] 重置 Build 方案迁移弹窗状态
binding-workspace-debug-reset-aws-bedrock-banner = [调试] 取消隐藏 AWS 登录横幅
binding-workspace-debug-open-oz-launch-modal = [调试] 打开 Oz 启动弹窗
binding-workspace-debug-reset-oz-launch-modal = [调试] 重置 Oz 启动弹窗状态
binding-workspace-debug-open-openwarp-launch-modal = [调试] 打开 OpenWarp 启动弹窗
binding-workspace-debug-reset-openwarp-launch-modal = [调试] 重置 OpenWarp 启动弹窗状态
binding-workspace-debug-install-opencode-warp-plugin = [调试] 安装 OpenCode Warp 插件
binding-workspace-debug-use-local-opencode-warp-plugin = [调试] 使用本地 OpenCode Warp 插件（仅测试）
binding-workspace-debug-open-session-config-modal = [调试] 打开会话配置弹窗
binding-workspace-debug-show-hoa-onboarding = [调试] 启动 HOA 引导流程
binding-workspace-sample-process = 采样进程
binding-workspace-dump-heap-profile = 转储堆分析（仅可执行一次）
# 标签导航（FixedBinding::custom）
binding-workspace-cycle-next-session = 切换到下一个标签
binding-workspace-cycle-prev-session = 切换到上一个标签
binding-workspace-add-window = 新建窗口
binding-workspace-new-file = 新建文件
# 缩放 / 字号
binding-workspace-zoom-in = 放大
binding-workspace-zoom-out = 缩小
binding-workspace-reset-zoom = 重置缩放
binding-workspace-increase-zoom-level = 增大缩放级别
binding-workspace-decrease-zoom-level = 减小缩放级别
binding-workspace-reset-zoom-level = 重置缩放级别为默认
binding-workspace-increase-font-size = 增大字号
binding-workspace-decrease-font-size = 减小字号
binding-workspace-reset-font-size = 重置字号为默认
# 主题 / 标签
binding-workspace-open-theme-picker = 打开主题选择器
binding-workspace-open-tab-configs-menu = 打开标签配置菜单
# 标签切换
binding-workspace-activate-tab-1 = 切换到第 1 个标签
binding-workspace-activate-tab-2 = 切换到第 2 个标签
binding-workspace-activate-tab-3 = 切换到第 3 个标签
binding-workspace-activate-tab-4 = 切换到第 4 个标签
binding-workspace-activate-tab-5 = 切换到第 5 个标签
binding-workspace-activate-tab-6 = 切换到第 6 个标签
binding-workspace-activate-tab-7 = 切换到第 7 个标签
binding-workspace-activate-tab-8 = 切换到第 8 个标签
binding-workspace-activate-tab-last = 切换到最后一个标签
binding-workspace-activate-prev-tab = 激活上一个标签
binding-workspace-activate-next-tab = 激活下一个标签
binding-workspace-activate-prev-pane = 激活上一个窗格
binding-workspace-activate-next-pane = 激活下一个窗格
# 布局 / 命令面板
binding-workspace-toggle-keybindings-page = 切换键盘快捷键页
binding-workspace-open-keybindings-editor = 打开键盘快捷键编辑器
binding-workspace-toggle-block-snackbar = 切换常驻命令头
# 无障碍
binding-workspace-a11y-concise = [a11y] 设置简洁的无障碍朗读
binding-workspace-a11y-verbose = [a11y] 设置详细的无障碍朗读
# 标签管理
binding-workspace-rename-active-tab = 重命名当前标签
binding-workspace-quit-warp = 退出 Warp
binding-workspace-close-active-tab = 关闭当前标签
binding-workspace-close-other-tabs = 关闭其他标签
# 通知
binding-workspace-toggle-notifications-on = 开启通知
binding-workspace-toggle-notifications-off = 关闭通知
# 命令面板
binding-workspace-launch-config-palette = 启动配置命令面板
binding-workspace-toggle-files-palette = 切换文件面板
binding-workspace-save-launch-configuration = 另存为新启动配置
binding-workspace-search-warp-drive = 搜索 Warp 云盘
# 更新
binding-workspace-update-and-relaunch = 安装更新并重启
binding-workspace-check-for-updates = 检查更新
# 杂项
binding-workspace-log-out = 退出登录
binding-workspace-toggle-resource-center = 切换资源中心
binding-workspace-export-warp-drive-objects = 导出全部 Warp 云盘对象
binding-workspace-install-cli = 全局安装 Oz CLI（可在 Warp 之外使用）
binding-workspace-uninstall-cli = 撤销 Oz CLI 全局安装（oz 在 Warp 内仍可使用）
binding-workspace-view-changelog = 查看最新更新日志
binding-workspace-toggle-warp-ai = 切换 Warp AI
binding-workspace-shift-focus-left = 切换焦点到左侧面板
binding-workspace-shift-focus-right = 切换焦点到右侧面板
binding-workspace-import-to-personal-drive = 导入到个人云盘
binding-workspace-import-to-team-drive = 导入到团队云盘
binding-workspace-copy-access-token = 复制访问令牌到剪贴板
binding-workspace-jump-to-latest-toast = 跳转到最近的智能体任务
binding-workspace-toggle-notification-mailbox = 切换通知收件箱
binding-workspace-toggle-agent-management-view = 切换智能体管理视图
# 设置页打开（其余使用 t! 路径）
binding-workspace-open-settings-account = 打开设置：账户
binding-workspace-open-settings-features = 打开设置：功能
binding-workspace-open-settings-file = 打开设置文件
# 溢出菜单
binding-workspace-invite-people = 邀请他人...
binding-workspace-link-to-slack = 加入我们的 Slack 社区（打开外部链接）
binding-workspace-link-to-user-docs = 查看用户文档（打开外部链接）
binding-workspace-view-warp-logs = 查看 Warp 日志
binding-workspace-link-to-privacy-policy = 查看隐私政策（打开外部链接）
binding-workspace-dump-debug-info = 转储调试信息

# 分散文件（任务 #20）
# notebooks/notebook.rs（4 EditableBinding + 2 FixedBinding::custom）
binding-notebooks-increase-font-size = 增大笔记本字号
binding-notebooks-decrease-font-size = 减小笔记本字号
binding-notebooks-reset-font-size = 重置笔记本字号
binding-notebooks-focus-terminal-input = 从笔记本聚焦终端输入
binding-notebooks-fixed-increase-font-size = 增大字号
binding-notebooks-fixed-decrease-font-size = 减小字号

# code/view.rs（4 EditableBinding）— SAVE_FILE_BINDING 由 ai_document_view 复用
binding-code-view-save-file = 保存文件
binding-code-view-save-file-as = 文件另存为
binding-code-view-close-all-tabs = 关闭所有标签页
binding-code-view-close-saved-tabs = 关闭已保存的标签页

# ai_assistant/panel.rs（1 FixedBinding::custom + 3 EditableBinding）
binding-ai-assistant-close-warp-ai = 关闭 Warp AI
binding-ai-assistant-focus-terminal-input = 从 Warp AI 聚焦终端输入
binding-ai-assistant-restart-warp-ai = 重启 Warp AI

# workspace/sync_inputs.rs（3 EditableBinding）
binding-workspace-disable-input-syncing = 停止同步所有窗格
binding-workspace-toggle-sync-inputs-tab = 切换同步当前标签页所有窗格
binding-workspace-toggle-sync-inputs-all-tabs = 切换同步所有标签页所有窗格

# code_review/mod.rs（2 EditableBinding + 1 FixedBinding::custom）
binding-code-review-save-all = 保存代码审查中所有未保存文件
binding-code-review-show-find-bar = 在代码审查中显示查找栏
binding-code-review-undo-revert = 撤销
binding-code-review-submit-review-comments = 将代码审查评论发送给智能体

# workflows/workflow_view.rs（2 EditableBinding）
binding-workflow-save = 保存工作流
binding-workflow-close = 关闭

# view_components/find.rs + code/editor/find/view.rs（共享 key）
binding-find-next-occurrence = 查找下一个匹配
binding-find-prev-occurrence = 查找上一个匹配

# root_view.rs（2 EditableBinding）
binding-root-view-toggle-fullscreen = 切换全屏
binding-root-view-debug-enter-onboarding = [调试] 进入引导状态

# pane_group/pane/welcome_view.rs（2 EditableBinding）
binding-welcome-view-terminal-session = 终端会话
binding-welcome-view-add-repository = 添加仓库

# notebooks/file/mod.rs（2 EditableBinding）
binding-notebooks-file-focus-terminal-input = 从文件聚焦终端输入
binding-notebooks-file-reload = 重新加载文件

# coding_entrypoints/project_buttons.rs（2 EditableBinding）
binding-project-buttons-open-repository = 打开仓库
binding-project-buttons-create-new-project = 新建项目

# workspace/view/right_panel.rs（1 EditableBinding）
binding-right-panel-toggle-maximize-code-review = 切换最大化代码审查面板

# undo_close/mod.rs（1 EditableBinding）
binding-undo-close-reopen = 重新打开已关闭的会话

# pane_group/pane/view/mod.rs（1 EditableBinding）
binding-pane-share-contents = 分享窗格

# pane_group/pane/get_started_view.rs（1 EditableBinding）
binding-get-started-terminal-session = 终端会话

# env_vars/view/env_var_collection.rs（1 EditableBinding）
binding-env-var-collection-close = 关闭

# ai/blocklist/inline_action/requested_command.rs（1 EditableBinding）
binding-requested-command-edit = 编辑请求的命令

# ai/blocklist/inline_action/code_diff_view.rs（1 EditableBinding）
binding-code-diff-edit = 编辑代码 diff

# ai/blocklist/block/cli.rs（1 EditableBinding）
binding-cli-take-control-of-running-command = 接管正在运行的命令
