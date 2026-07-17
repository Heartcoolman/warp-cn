# Warp 中文社区版

**中文** | [English](./README_en.md)

本仓库是 [warpdotdev/warp](https://github.com/warpdotdev/warp) 的**中文社区 fork**：客户端 UI 全量汉化，默认中文，可在设置中切换为英文或跟随系统。

> 本 fork 仅汉化客户端 UI（菜单、命令面板、设置、对话框、Tooltip、Block actions、Resource Center、Onboarding 等）；终端命令输出、服务端 GraphQL 字段、AI 模型回复保持原样不变。

## 语言切换

打开 **设置 → 通用 → 语言**：

- `中文（简体）` — 强制中文（fork 默认）
- `English` — 强制英文
- `跟随系统` — 系统 locale 为 `zh*` 时使用中文，否则英文

切换不需要重启 Warp。设置项位于 `~/.warp/settings.toml` 的 `language` 字段。

## 自动更新（无需 Apple Developer ID）

本 fork 通过 GitHub Releases 实现客户端自更新，**不依赖任何后端服务器**。机制：

- CI 在打 `v*` tag 时产出 `.tar.gz` + `.minisig` 资产，使用 minisign 私钥签名（私钥仅存在于 GitHub Actions Secret）。
- 客户端启动后到 **设置 → 账户 → 版本**，点击 **下载并安装**，进程内通过 `reqwest` 拉取并验签，`tar` 解压后原地替换、重启。
- 进程内下载不会附加 `com.apple.quarantine`，因此 ad-hoc 签名的 `.app` 替换后 Gatekeeper 不会重新评估，更新对用户静默。

**首次安装** 是唯一需要手动操作的一次（macOS 不允许应用绕过 Gatekeeper 自我授权）：

```bash
xattr -dr com.apple.quarantine /path/to/Warp-cn.app
```

或在 系统设置 → 隐私与安全性 中点击「仍要打开」。**之后所有自动更新永久静默生效。**

> 维护者 fork 后首次启用更新通道：执行 `script/generate_update_keys.sh` 生成 minisign 密钥对，将公钥提交到仓库（`script/warp-update.pub`），私钥放入 GitHub Actions Secret `MINISIGN_SECRET_KEY`。

## macOS 凭据存储（不使用钥匙串）

ad-hoc 签名的 `.app` 每次发版二进制 CDHash 都会变，而 macOS 钥匙串 ACL 对 ad-hoc 二进制是按 CDHash 信任的，没有 designated requirement 可以跨版本共享，因此每次升级都会弹「Warp-cn 想要使用 dev.warp.WarpCn 中的机密信息」。本 fork 在 macOS 上**不使用钥匙串**，登录 token / AI API key / MCP OAuth 凭据全部以 AES-256-GCM 加密文件形式落在：

```
~/Library/Application Support/dev.warp.WarpCn/dev.warp.WarpCn-<KEY>
```

文件权限 `0600`，加密方案与本仓库 Linux fallback 同款（见 `crates/warpui_extras/src/secure_storage/mac.rs`）。

**从旧版本（曾使用钥匙串）升级**会触发一次性代价：

- 需要重新登录账号
- 需要重填 AI API key（设置 → AI）
- MCP OAuth server 需要重新授权

旧钥匙串条目不会被自动清理（避免再次弹窗）。如要手动清理，打开「钥匙串访问.app」搜索 `dev.warp.WarpCn`，删除 `USER_STORAGE_KEY`、`SECURE_STORAGE_KEY`、`FileBasedMcpCredentials` 三条；不删除也无副作用，仅占用零碎空间。

## Windows 支持

本 fork 可在 Windows（x86_64）上原生构建运行，UI 同样默认中文。

### 运行所需文件

`warp-oss.exe` 依赖一组运行库文件，**必须按下述布局放在一起**，否则启动会报 `Failed to load ConPTY library module` 或缺失 VC++ 运行时：

```
warp-oss.exe
conpty.dll
dxcompiler.dll
dxil.dll
vcruntime140.dll
vcruntime140_1.dll
msvcp140.dll
x64\
  └─ OpenConsole.exe      ← 注意在 x64 子目录下
resources\                ← 打包资源（由安装流程生成）
```

> 以官方 `script/windows/windows-installer.iss` 的安装布局为准。其中 `conpty.dll` / DXC / VC++ 运行时与 exe 同级，而 `OpenConsole.exe` 位于 `x64\` 子目录。`app/build.rs` 在构建时会把 ConPTY 与 DXC 这几个 DLL 自动拷到 `target/<profile>/` 下对应位置；VC++ 运行时三件套（`vcruntime140.dll` / `vcruntime140_1.dll` / `msvcp140.dll`）由安装包从 `app/assets/windows/<arch>/` 一并打入。便携分发请直接用安装包，或参照 iss 布局手动组织，不要只复制 exe 同级的几个文件。

### 从源码构建

在 Windows 上构建有两条资源编译路径。默认（未设置 `WARP_RC` 时）需要 MSVC 工具链（Visual Studio 2022 Build Tools，含 Windows SDK）：`app/build.rs` 经注册表定位 `cl.exe` 以装配 MSVC 环境（让资源编译器能找到头文件），再由 embed-resource 调用 `rc.exe` 编译资源：

```sh
cargo build --release --bin warp-oss --features gui
```

还需 `protoc`（protobuf 编译器）在 PATH 上。

> **免 Visual Studio 的便携工具链**（LLVM `clang-cl` / `lld-link` / `llvm-rc` + xwin）已支持：设置 `WARP_RC` 环境变量指向独立资源编译器（如 `llvm-rc`）后，`app/build.rs` 会直接用它编译资源、跳过上面的 `cl.exe` 注册表查找。详细配置见 [`docs/building-windows-portable.md`](docs/building-windows-portable.md)。

### 中文渲染

Windows 上 UI 文本经 cosmic-text 排版，启动时会预加载系统的中文字体（微软雅黑、宋体）+ Segoe UI Emoji 供字形回退；若系统缺这些字体则回退到全量系统字体扫描，确保中文不出现豆腐块。

### 凭据存储

Windows 上登录 token / AI API key / MCP 凭据通过 **Windows DPAPI**（`CryptProtectData`，按当前用户加密）保护后落盘，不使用系统钥匙串，见 `crates/warpui_extras/src/secure_storage/windows.rs`。（注：macOS 用 AES-256-GCM 加密文件，二者机制不同。）

> Windows 安装包（InnoSetup）流水线参见 `script/windows/`（`bundle.ps1` + `windows-installer.iss`），相关进展见 issue #10。

## 与上游同步

本 fork 维护者会定期 merge upstream。每个含 UI 字符串的 PR 拆为两步：

1. **结构步**：`t!("...")` 替换 inline 字面量 + 同步 `bundles/en/*.ftl`
2. **翻译步**：填充 `bundles/zh-CN/*.ftl`

详见 `crates/warp_i18n/MERGE_NOTES.md`。

## 贡献翻译 / 修订

- 新增字符串 / 修订术语：见 `docs/i18n.md`
- 术语锁定：`crates/warp_i18n/GLOSSARY.md`
- Lint 与 CI：`cargo xtask check-i18n --mode hard`
- Bundle 对齐校验：`cargo xtask check-i18n --check-parity`

## Direct LLM Backend（BYOK 直连）— 初版预览

> ⚠️ **初版（v0.1）**：核心路径已通，但仍在打磨。基本评测、读盘、shell、MCP 均可跑；偶尔会遇到 tool result 时序竞态（v4-flash 出现 "tool result unavailable" 文本时模型已被指引重试）。请把遇到的问题在 issue 区反馈，我们会持续迭代。

允许用户**直接用自己的 API Key 调用第三方 LLM**，跑完整 agent 循环时**完全不连 Warp 云端**。9 个 commit 一次合入 master（merge `84f9ef23`）。

### 已支持的 Provider

| Provider | 默认入口 | 备注 |
|---|---|---|
| Anthropic | `https://api.anthropic.com/v1` | 原生 SSE |
| OpenAI 兼容 | `https://api.openai.com/v1` | 含 DeepSeek（`https://api.deepseek.com/v1`）等任何 OpenAI-compatible endpoint |
| Google Gemini | `https://generativelanguage.googleapis.com/v1beta` | `?alt=sse` 流式 |

每个 Provider 可独立配置 base url + API key + 默认 model；模型列表通过 provider 的 `/v1/models` 动态拉取。

### 启用方法

1. **设置 → AI → API Keys** 面板新增的 "Direct backend" 部分填入任一 provider 的 key + url + 默认 model
2. **设置 → 通用** 切换到对应 provider（首次启用需重启一次）
3. 之后所有 Agent Mode 对话直接走你自己的 API key，**无需登录 Warp 账号**

### 已实现的工具集（11 个）

`read_files` · `run_shell_command` · `grep` · `file_glob` · `apply_file_diffs` · `ask_user_question` · `write_to_long_running_shell_command` · `read_shell_command_output` · `transfer_shell_command_control_to_user` · `read_mcp_resource` · `call_mcp_tool`

MCP 走客户端侧执行，复用本机已配的 `~/.warp/.mcp.json`，无需在 server 侧重新对接 MCP server。

### 已知限制 / 待办

- **Reasoning 模型回灌**：DeepSeek-R1 / o1 系列已支持 `reasoning_content` 必须 echo back 的协议；其他 reasoning 模型若拒收按错误暴露
- **Tool result 时序**：客户端在多 parallel tool_call 场景下偶尔在 result 全部回流前就发下一轮请求，缺失的 result 由 server 侧 stub 引导 LLM 重试。复现请抓 `~/Library/Logs/warp-oss.log` 里的 `DirectBackend OpenAI: stubbed N missing` 警告
- **Computer Use / Workflow agent / Drive 等高阶云功能**：在 direct mode 下走默认空响应，不影响主对话路径
- **Cost / token 统计**：`StreamFinished.token_usage` 仅记 input/output，不算成本

### 安全 / 权限

- 4 个权限 gate（read_files / mcp / file_write / pty / execute_commands）在启用 feature 时把上游默认 `AlwaysAsk` 强制 coerce 到 `AgentDecides`，确保 own-LLM 的 read-only 探查不会被 UI 弹窗陷阱拒死
- 所有 fork 行为都在 `#[cfg(feature = "direct_llm_backend")]` 之后；上游 build（关 feature）行为完全不受影响
- API Key 走 `~/Library/Application Support/dev.warp.WarpCn/` 的 AES-256-GCM 加密文件，不入钥匙串

### 详细信息

- 入口源码：`app/src/server/direct_backend/`
- 配置 schema：`crates/ai/src/direct_backend/config.rs`
- 工具调度 / SSE：`app/src/server/direct_backend/multi_agent/`
- Cargo feature：`app/Cargo.toml` 的 `direct_llm_backend = [...]`（默认开启，编译 fork build 时无需额外 flag）

## 与上游差异

- 客户端 UI ~7000+ 字符串汉化（不影响功能 / 性能）
- 新增 crate：`warp_i18n`、`warp_i18n_macros`
- 新增 CI workflow：`.github/workflows/i18n-lint.yml`（`hard` 模式 + allowlist 冻结）
- `crates/warp_server_client` 增加 `RemoteString` marker，标记不可本地化的服务端字段
- 新增 `direct_llm_backend` cargo feature（见上）

## 上游 / 英文 README

通用项目说明、构建、贡献流程、许可证：见 [`README_en.md`](./README_en.md)。
