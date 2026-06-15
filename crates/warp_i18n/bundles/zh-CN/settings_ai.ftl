# AI / 智能体设置页字符串（Warp 智能体、配置文件、知识库、第三方 CLI 智能体）。
# Keys MUST start with `settings-ai-`.

settings-ai-title = AI
settings-ai-remote-session-disallowed = 您的组织不允许在活动窗格包含远程会话内容时使用 AI
settings-ai-next-command-description = 让 AI 根据您的命令历史、输出和常见工作流建议下一条要运行的命令。
settings-ai-prompt-suggestions-description = 让 AI 根据最近的命令及其输出，在输入框中以内联横幅形式建议自然语言提示词。
settings-ai-code-banners-description = 让 AI 根据最近的命令及其输出，在区块列表中以内联横幅形式建议代码差异和查询。
settings-ai-autosuggestions-description = 让 AI 根据最近的命令及其输出建议自然语言自动补全。
settings-ai-shared-block-title-description = 让 AI 根据命令和输出为您的共享区块生成标题。
settings-ai-git-autogen-description = 让 AI 生成提交消息和 Pull Request 标题与描述。
settings-ai-commands-placeholder = 命令，以逗号分隔
settings-ai-agent-decides = 智能体决定
settings-ai-always-allow = 始终允许
settings-ai-always-ask = 始终询问
settings-ai-allow-specific-dirs = 允许在特定目录中
settings-ai-show-oz-changelog-agent = 在新建智能体对话视图中显示 Oz 更新日志
settings-ai-hide-oz-changelog-agent = 在新建智能体对话视图中隐藏 Oz 更新日志
settings-ai-show-oz-changelog = 在新建对话视图中显示 Oz 更新日志
settings-ai-show-use-agent-footer = 显示"使用智能体"页脚
settings-ai-hide-use-agent-footer = 隐藏"使用智能体"页脚
settings-ai-voice-input-toggle-key-error = 在下拉选项列表中找不到当前的 VoiceInputToggleKey 值
settings-ai-edit = 编辑
settings-ai-models = 模型
settings-ai-permissions = 权限
settings-ai-base-model = 基础模型：
settings-ai-full-terminal-use = 完整终端使用：
settings-ai-computer-use = 计算机使用：
settings-ai-apply-code-diffs = 应用代码差异：
settings-ai-read-files = 读取文件：
settings-ai-execute-commands = 执行命令：
settings-ai-interact-running-commands = 与运行中的命令交互：
settings-ai-ask-questions = 提问：
settings-ai-call-mcp-servers = 调用 MCP 服务器：
settings-ai-call-web-tools = 调用网页工具：
settings-ai-auto-sync-plans = 自动同步计划到 Warp 云盘：
settings-ai-ask-on-first-write = 首次写入时询问
settings-ai-never = 从不
settings-ai-never-ask = 从不询问
settings-ai-ask-unless-auto-approve = 除非自动批准否则询问
settings-ai-on = 开启
settings-ai-off = 关闭
settings-ai-none = 无
settings-ai-directory-allowlist = 目录允许列表：
settings-ai-command-allowlist = 命令允许列表：
settings-ai-command-denylist = 命令拒绝列表：
settings-ai-mcp-allowlist = MCP 允许列表：
settings-ai-mcp-denylist = MCP 拒绝列表：
settings-ai-auto = 自动
settings-ai-unknown = 未知

# 设置区块 / 切换项标题
settings-ai-section-permissions = 权限
settings-ai-perm-apply-diffs = 应用代码差异
settings-ai-perm-read-files = 读取文件
settings-ai-perm-execute = 执行命令
settings-ai-perm-interact = 与运行中的命令交互
settings-ai-perm-managed-by-workspace = 你的部分权限由所在工作区管理。
settings-ai-cmd-denylist = 命令拒绝列表
settings-ai-cmd-denylist-desc = 用于匹配 Warp 智能体执行前必须征得许可的命令的正则表达式。
settings-ai-cmd-allowlist = 命令允许列表
settings-ai-cmd-allowlist-desc = 用于匹配 Warp 智能体可自动执行的命令的正则表达式。
settings-ai-dir-allowlist = 目录允许列表
settings-ai-dir-allowlist-desc = 授予智能体对特定目录的文件访问权限。

settings-ai-base-model-title = 基础模型
settings-ai-base-model-desc = 该模型是 Warp 智能体的核心引擎，承担大多数交互；必要时会调用其他模型完成规划、代码生成等任务。Warp 可能根据可用性或会话摘要等辅助任务自动切换备用模型。

settings-ai-codebase-context = 代码库上下文
settings-ai-codebase-context-desc = 允许 Warp 智能体生成代码库的概要以作为上下文。任何代码都不会存储在我们的服务器上。
settings-ai-learn-more = 了解更多

settings-ai-mcp-call-servers = 调用 MCP 服务器
settings-ai-mcp-allowlist-desc = 允许 Warp 智能体调用以下 MCP 服务器。
settings-ai-mcp-denylist-desc = Warp 智能体在调用此列表中的任何 MCP 服务器之前都会先征得许可。
settings-ai-mcp-add-server = 添加服务器
settings-ai-mcp-manage = 管理 MCP 服务器
settings-ai-mcp-autospawn = 自动启动来自第三方智能体的服务器
settings-ai-mcp-autospawn-desc = 自动检测并启动来自全局范围（如主目录）第三方 AI 智能体配置文件的 MCP 服务器。仓库内检测到的服务器永不自动启动，必须在 MCP 设置页中单独启用。
settings-ai-mcp-supported-providers = 查看受支持的提供方。

settings-ai-input-hint-text = 显示输入提示文本
settings-ai-show-agent-tips = 显示智能体提示
settings-ai-include-agent-cmds-history = 在历史中包含智能体执行的命令

settings-ai-natural-lang-detection = 自然语言检测
settings-ai-natural-lang-detection-desc = 启用自然语言检测后，将识别终端输入中的自然语言并自动切换到智能体模式以执行 AI 查询。
settings-ai-natural-lang-denylist = 自然语言拒绝列表
settings-ai-natural-lang-denylist-desc = 此列表中的命令永不触发自然语言检测。
settings-ai-let-us-know = 告诉我们
settings-ai-autodetect-prompts-terminal = 在终端输入中自动检测智能体提示词
settings-ai-autodetect-cmds-agent = 在智能体输入中自动检测终端命令

settings-ai-rules-help-desc = 规则可帮助 Warp 智能体遵循你的约定，无论是面向代码库还是特定工作流。
settings-ai-suggested-rules = 推荐规则
settings-ai-suggested-rules-desc = 让 AI 根据你的交互建议要保存的规则。
settings-ai-warp-drive-context = 将 Warp 云盘作为智能体上下文
settings-ai-warp-drive-context-desc = Warp 智能体可利用你的 Warp 云盘内容，依据你和团队的开发者工作流和环境定制响应；包括所有工作流、笔记本与环境变量。
settings-ai-knowledge = 知识库
settings-ai-manage-rules = 管理规则
settings-ai-voice-input = 语音输入
settings-ai-voice-input-desc = 语音输入允许你直接对终端说话来控制 Warp（由
settings-ai-voice-input-key = 语音输入激活键
settings-ai-voice-input-key-tip = 按住以激活。

settings-ai-show-conv-history = 在工具面板中显示会话历史
settings-ai-thinking-display = 智能体思考显示
settings-ai-thinking-display-desc = 控制推理 / 思考轨迹的显示方式。
settings-ai-existing-conv-layout = 打开已有智能体会话时的首选布局
settings-ai-show-coding-toolbar = 显示编码智能体工具栏
settings-ai-show-coding-toolbar-desc = 在运行编码智能体时显示带快捷操作的工具栏，例如
settings-ai-third-party-cli-agents = 第三方 CLI 智能体
settings-ai-rich-input-auto-show = 根据智能体状态自动显示 / 隐藏 Rich Input
settings-ai-rich-input-requires-plugin = 需要为你的编码智能体安装 Warp 插件
settings-ai-rich-input-auto-open = 编码智能体会话开始时自动打开 Rich Input
settings-ai-rich-input-auto-dismiss = 提交提示词后自动关闭 Rich Input
settings-ai-toolbar-regex-desc = 添加正则表达式以为匹配的命令显示编码智能体工具栏。

settings-ai-attribution-toggle = 启用智能体署名
settings-ai-attribution-section = 智能体署名
settings-ai-attribution-desc = Oz 可在它创建的提交消息和 Pull Request 中添加署名信息

settings-ai-cloud-computer-use = 在云端智能体中使用计算机
settings-ai-experimental = 实验性
settings-ai-cloud-computer-use-desc = 在 Warp 应用启动的云端智能体会话中启用计算机使用。
settings-ai-orchestration = 编排
settings-ai-orchestration-desc = 启用多智能体编排，允许智能体生成并协调并行的子智能体。

settings-ai-byok-desc = 使用你来自模型提供方的 API 密钥供 Warp 智能体调用。API 密钥仅本地存储，不会同步到云端。使用自动模型或未提供 API 密钥的提供方的模型将消耗 Warp 额度。
settings-ai-openai-key = OpenAI API 密钥
settings-ai-anthropic-key = Anthropic API 密钥
settings-ai-google-key = Google API 密钥
settings-ai-upgrade-build = 升级到 Build 套餐
settings-ai-credit-fallback = Warp 额度回退

settings-ai-bedrock-managed-by-org = Warp 会加载并发送本地 AWS CLI 凭据以供支持 Bedrock 的模型使用。该设置由你所在的组织管理。
settings-ai-bedrock-toggle = 使用 AWS Bedrock 凭据
settings-ai-bedrock-login-cmd = 登录命令
settings-ai-bedrock-auto-run-login = 自动运行登录命令
settings-ai-bedrock-auto-run-login-desc = 启用后，AWS Bedrock 凭据过期时将自动运行登录命令。

settings-ai-toolbar-layout = 工具栏布局
settings-ai-show-model-picker = 在提示词中显示模型选择器
settings-ai-toolbar-cmds-enable-span = 启用工具栏的命令
settings-ai-active-ai = 活动 AI
settings-ai-next-command = 下一条命令
settings-ai-split-pane = 拆分窗格
settings-ai-read-only-permission = 只读
settings-ai-supervised-permission = 受监督
settings-ai-restricted-billing = 因计费问题已受限
settings-ai-unlimited = 无限
settings-ai-prompt-suggestions = 提示词建议
settings-ai-suggested-code-banners = 代码建议横幅
settings-ai-natural-lang-autosuggestions = 自然语言自动建议
settings-ai-shared-block-title-gen = 共享区块标题生成
settings-ai-warp-agent = Warp 智能体
settings-ai-create-account-prompt = 要使用 AI 功能，请创建账户。
settings-ai-profiles = 配置文件
settings-ai-set-boundaries-desc = 为智能体的运行设置边界。选择它能访问什么、拥有多少自主权、何时必须征得你的批准；你还可以微调与自然语言输入、代码库感知等相关的行为。
settings-ai-mcp-select-servers-header = 选择 MCP 服务器
settings-ai-other = 其他
settings-ai-select-coding-agent = 选择编码智能体
settings-ai-resets-after = 重置于 {$time}

# 命令面板 suffix
settings-ai-cmd-suffix-ai = AI
settings-ai-cmd-suffix-terminal-cmd-autodetect-agent = 智能体输入框中的终端命令自动识别
settings-ai-cmd-suffix-natural-language-detection = 自然语言检测
settings-ai-cmd-suffix-agent-prompt-autodetect-terminal = 终端输入中的智能体提示自动识别
settings-ai-cmd-suffix-prompt-suggestions = 提示词建议
settings-ai-cmd-suffix-code-suggestions = 代码建议
settings-ai-cmd-suffix-natural-language-autosuggestions = 自然语言自动建议
settings-ai-cmd-suffix-shared-block-title-generation = 共享区块标题生成
settings-ai-cmd-suffix-voice-input = 语音输入
settings-ai-cmd-suffix-codebase-index = 代码库索引
settings-ai-cmd-suffix-git-operations-autogen = 提交与拉取请求生成
settings-ai-cmd-suffix-include-agent-commands-in-history = 在历史记录中包含智能体执行的命令
settings-ai-cmd-suffix-conversation-history-in-tools-panel = 工具面板中的对话历史
settings-ai-cmd-suffix-model-picker-in-prompt = 提示框中的模型选择器
settings-ai-cmd-suffix-coding-agent-toolbar = 编码智能体工具栏
settings-ai-cmd-suffix-rules = 规则
settings-ai-cmd-suffix-suggested-rules = 建议规则
settings-ai-cmd-suffix-warp-drive-as-agent-context = 将 Warp Drive 作为智能体上下文
settings-ai-cmd-suffix-auto-spawn-third-party-mcp = 自动启动第三方智能体的服务器
settings-ai-cmd-suffix-warp-credit-fallback = Warp 额度回退
settings-ai-cmd-suffix-auto-toggle-rich-input = 根据智能体状态自动显示或隐藏富输入
settings-ai-cmd-suffix-auto-open-rich-input-on-cli-agent-start = 编码智能体会话开始时自动打开富输入
settings-ai-cmd-suffix-auto-dismiss-rich-input-after-submit = 提交提示后自动关闭富输入
settings-ai-supergrok-connect-description = 连接你的 SuperGrok 订阅，即可通过你的 xAI 账户在 Warp Agent 中使用 Grok 模型。
