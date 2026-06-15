# 命令面板：命令显示名与简介。
# Key 前缀 MUST 为 `command-`，与 en bundle 一一对应。

# 标签页 / 窗口生命周期。
command-new-tab = 新建标签页
command-new-terminal-tab = 新建终端标签页
command-new-agent-tab = 新建 Agent 标签页
command-new-cloud-agent-tab = 新建云 Agent 标签页
command-close-window = 关闭窗口
command-move-tab-left = 标签页左移
command-move-tab-right = 标签页右移
command-close-tabs-right = 关闭右侧标签页

# 侧边面板与面板切换。
command-open-left-panel = 打开左侧面板
command-close-focused-panel = 关闭聚焦面板
command-toggle-project-explorer = 切换项目浏览器
command-toggle-vertical-tabs-panel = 切换垂直标签栏
command-toggle-code-review = 切换代码评审
command-toggle-warp-drive = 切换 Warp Drive
command-toggle-agent-conversation-list = 切换 Agent 会话列表视图
command-toggle-command-palette = 切换命令面板
command-toggle-navigation-palette = 切换导航面板
command-open-global-search = 打开全局搜索
command-left-panel-agent-conversations = 左侧面板：Agent 会话
command-left-panel-project-explorer = 左侧面板：项目浏览器
command-left-panel-global-search = 左侧面板：全局搜索
command-left-panel-warp-drive = 左侧面板：Warp Drive

# Drive：笔记本 / 工作流 / 文件夹 / 提示词 / 环境变量。
command-new-team-notebook = 新建团队笔记本
command-new-personal-notebook = 新建个人笔记本
command-new-team-workflow = 新建团队工作流
command-new-personal-workflow = 新建个人工作流
command-new-team-folder = 新建团队文件夹
command-new-personal-folder = 新建个人文件夹
command-new-team-env-vars = 新建团队环境变量
command-new-personal-env-vars = 新建个人环境变量
command-new-personal-prompt = 新建个人提示词
command-new-team-prompt = 新建团队提示词

# 仓库 / 配置入口。
command-open-repository = 打开仓库
command-open-ai-rules = 打开 AI 规则
command-open-mcp-servers = 打开 MCP 服务器

# 设置入口。
command-open-settings = 打开设置
command-open-settings-appearance = 打开设置：外观
command-open-settings-shared-blocks = 打开设置：共享块
command-open-settings-keyboard-shortcuts = 打开设置：键盘快捷键
command-open-settings-about = 打开设置：关于
command-open-settings-teams = 打开设置：团队
command-open-settings-privacy = 打开设置：隐私
command-open-settings-warpify = 打开设置：Warpify
command-open-settings-ai = 打开设置：AI
command-open-settings-billing = 打开设置：账单与用量
command-open-settings-code = 打开设置：代码
command-open-settings-referrals = 打开设置：推荐
command-open-settings-environments = 打开设置：环境
command-open-settings-mcp-servers = 打开设置：MCP 服务器

# 反馈 / 外部链接。
command-send-feedback = 发送反馈（打开外部链接）
command-send-feedback-with-oz = 用 Oz 发送反馈

# 终端侧命令。
command-edit-prompt = 编辑提示符
command-attach-block-as-agent-context = 将所选块附加为 Agent 上下文
command-attach-text-as-agent-context = 将所选文本附加为 Agent 上下文
command-write-codebase-index-snapshot = 写入当前代码库索引快照
command-initiate-project = 初始化 Warp 项目
command-add-folder-as-project = 添加当前文件夹为项目

# macOS 顶部菜单短形式（MAC_MENUS_CONTEXT）。与上方命令面板长描述并列存在。
command-project-explorer-mac = 项目浏览器
command-new-team-notebook-mac = 新建团队笔记本
command-new-personal-notebook-mac = 新建个人笔记本
command-new-team-workflow-mac = 新建团队工作流
command-new-personal-workflow-mac = 新建个人工作流
command-new-team-folder-mac = 新建团队文件夹
command-new-personal-folder-mac = 新建个人文件夹
command-toggle-code-review-mac = 切换代码评审
command-toggle-vertical-tabs-panel-mac = 切换垂直标签栏
command-global-search-mac = 全局搜索
command-warp-drive-mac = Warp Drive
command-agent-conversation-list-mac = Agent 会话列表视图
command-close-focused-panel-mac = 关闭聚焦面板
command-command-palette-mac = 命令面板
command-close-window-mac = 关闭窗口
command-navigation-palette-mac = 导航面板
command-new-team-env-vars-mac = 新建团队环境变量
command-new-personal-env-vars-mac = 新建个人环境变量
command-new-personal-prompt-mac = 新建个人提示词
command-new-team-prompt-mac = 新建团队提示词
command-open-repository-mac = 打开仓库
command-open-ai-rules-mac = 打开 AI 规则
command-open-mcp-servers-mac = 打开 MCP 服务器
command-settings-mac = 设置
command-appearance-mac = 外观…
command-view-shared-blocks-mac = 查看共享块…
command-configure-keyboard-shortcuts-mac = 配置键盘快捷键…
command-about-warp-mac = 关于 Warp
command-open-team-settings-mac = 打开团队设置
command-configure-warpify-mac = 配置 Warpify…
command-attach-selection-as-agent-context-mac = 将所选内容附加为 Agent 上下文

# 静态斜杠命令描述。
command-slash-agent-desc = 开始新会话
command-slash-cloud-agent-desc = 开始新的云智能体会话
command-slash-add-mcp-desc = 通过 MCP 设置页添加新的 MCP 服务器
command-slash-pr-comments-desc = 拉取 GitHub PR 评审评论
command-slash-create-environment-desc = 通过向导创建 Oz 环境（Docker 镜像 + 代码仓库）
command-slash-docker-sandbox-desc = 创建新的 Docker 沙箱终端会话
command-slash-create-new-project-desc = 让 Oz 引导你创建新的编码项目
command-slash-open-skill-desc = 在 Warp 内置编辑器中打开技能的 Markdown 文件
command-slash-skills-desc = 调用技能
command-slash-add-prompt-desc = 添加新的智能体提示词
command-slash-add-rule-desc = 为智能体添加新的全局规则
command-slash-open-file-desc = 在 Warp 代码编辑器中打开文件
command-slash-rename-tab-desc = 重命名当前标签页
command-slash-rename-conversation-desc = 重命名当前会话
command-slash-set-tab-color-desc = 设置当前标签页颜色
command-slash-fork-desc = 在新窗格或新标签页中 fork 当前会话
command-slash-open-code-review-desc = 打开代码审查
command-slash-index-desc = 索引此代码库
command-slash-init-desc = 索引此代码库并生成 AGENTS.md 文件
command-slash-open-project-rules-desc = 打开项目规则文件（AGENTS.md）
command-slash-open-mcp-servers-desc = 打开 MCP 服务器
command-slash-open-settings-file-desc = 打开设置文件（TOML）
command-slash-changelog-desc = 打开最新更新日志
command-slash-feedback-desc = 发送反馈
command-slash-open-repo-desc = 切换到另一个已索引代码仓库
command-slash-open-rules-desc = 查看你的全局规则和项目规则
command-slash-new-desc = 开始新会话（/agent 的别名）
command-slash-model-desc = 切换基础智能体模型
command-slash-host-desc = 切换云智能体执行主机
command-slash-harness-desc = 切换云智能体执行框架
command-slash-environment-desc = 切换云智能体环境
command-slash-profile-desc = 切换当前执行配置
command-slash-plan-desc = 让智能体先调研并为任务创建计划
command-slash-orchestrate-desc = 将任务拆成子任务，并用多个智能体并行运行
command-slash-compact-desc = 总结会话历史，释放上下文空间
command-slash-compact-and-desc = 压缩会话后发送后续提示词
command-slash-queue-desc = 将提示词排队，在智能体回复完成后发送
command-slash-fork-and-compact-desc = fork 当前会话，并在副本中压缩上下文
command-slash-fork-from-desc = 从指定查询 fork 会话
command-slash-continue-locally-desc = 在本地继续此云端会话
command-slash-usage-desc = 打开账单与用量设置
command-slash-remote-control-desc = 为此会话启动远程控制
command-slash-cost-desc = 切换额度用量详情
command-slash-conversations-desc = 打开会话历史
command-slash-prompts-desc = 搜索已保存的提示词
command-slash-rewind-desc = 回退到会话中的先前位置
command-slash-export-to-clipboard-desc = 以 Markdown 格式将当前会话导出到剪贴板
command-slash-export-to-file-desc = 将当前会话导出为 Markdown 文件
command-slash-handoff-desc = 把当前对话移交给云端智能体
