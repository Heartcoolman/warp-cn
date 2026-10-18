<div align="center">

# Warp 中文社区版

**客户端默认中文 · 自备 API Key · GitHub 自更新**

[中文](./README_zh.md) · [English](./README_en.md)

<br/>

[![GitHub stars](https://img.shields.io/github/stars/Heartcoolman/warp-cn?style=flat-square&logo=github)](https://github.com/Heartcoolman/warp-cn/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/Heartcoolman/warp-cn?style=flat-square&logo=github)](https://github.com/Heartcoolman/warp-cn/network/members)
[![GitHub release](https://img.shields.io/github/v/release/Heartcoolman/warp-cn?include_prereleases&style=flat-square&label=release)](https://github.com/Heartcoolman/warp-cn/releases)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-blue?style=flat-square)](#快速开始)
[![License](https://img.shields.io/badge/license-AGPL--3.0%20%2F%20MIT-green?style=flat-square)](#许可证)

<br/>

[**下载最新构建 →**](https://github.com/Heartcoolman/warp-cn/releases)
&nbsp;·&nbsp;
[提 Issue](https://github.com/Heartcoolman/warp-cn/issues)
&nbsp;·&nbsp;
[上游 Warp](https://github.com/warpdotdev/warp)

</div>

---

[Warp](https://www.warp.dev) 的中文社区 fork：界面默认中文，可一键切回英文或跟随系统；并附带 GitHub 自更新、本地加密凭据，以及可选的「自备 API Key 直连大模型」。

| 会汉化 | 不汉化 |
|:------:|:------:|
| 菜单 · 命令面板 · 设置 · 对话框 · Tooltip<br/>Block actions · Resource Center · Onboarding 等 **客户端 UI** | 终端命令输出 · 服务端 GraphQL 字段 · **AI 模型回复原文** |

---

## 目录

| | | |
|:---|:---|:---|
| [快速开始](#快速开始) | [语言切换](#语言切换) | [自备 API Key](#自备-api-keydirect-llm) |
| [与官方版差异](#与官方版的差异) | [平台说明](#平台说明) | [从源码构建](#从源码构建) |
| [贡献](#贡献) | [许可证](#许可证) | |

---

## 快速开始

### 下载安装

前往 **[Releases](https://github.com/Heartcoolman/warp-cn/releases)** 下载对应平台构建包。

| 平台 | 怎么装 |
|:----:|--------|
| **macOS** | 下载 `.app` / `.tar.gz`，首次启动前先 [解除隔离](#macos首次启动) |
| **Windows** | 优先使用安装包；便携分发请按 [运行时布局](#windows-运行时布局) 组织文件 |
| **Linux** | 目前以源码构建为主 → [从源码构建](#从源码构建) |

#### macOS：首次启动

本 fork 使用 ad-hoc 签名，**不依赖 Apple Developer ID**。第一次打开前二选一即可：

```bash
xattr -dr com.apple.quarantine /path/to/Warp-cn.app
```

或在 **系统设置 → 隐私与安全性** 中点击「仍要打开」。

> [!TIP]
> 完成首次授权后，应用内更新会静默生效，无需再处理隔离属性。

#### 应用内更新

**设置 → 账户 → 版本 → 下载并安装**

更新走 GitHub Releases + minisign 验签，**不经过任何第三方后端**。

---

## 语言切换

路径：**设置 → 通用 → 语言**

| 选项 | 行为 |
|:----:|------|
| **中文（简体）** | 强制中文（本 fork 默认） |
| **English** | 强制英文 |
| **跟随系统** | 系统 locale 为 `zh*` 时用中文，否则英文 |

切换后**即时生效**，无需重启。配置项：`~/.warp/settings.toml` → `language`。

---

## 自备 API Key（Direct LLM）

> [!IMPORTANT]
> **初版预览（v0.1）** — 主路径已通，仍在打磨。问题请到 [Issues](https://github.com/Heartcoolman/warp-cn/issues) 反馈。

用自己的 API Key 跑完整 Agent 循环：**无需登录 Warp 账号，不连 Warp 云端**。

### 支持的服务商

| 服务商 | 默认 Base URL | 备注 |
|:------:|---------------|------|
| **Anthropic** | `https://api.anthropic.com/v1` | 原生 SSE |
| **OpenAI 兼容** | `https://api.openai.com/v1` | 含 DeepSeek 等任意 OpenAI-compatible 接口 |
| **Google Gemini** | `https://generativelanguage.googleapis.com/v1beta` | `?alt=sse` 流式 |

每个服务商可单独配置 base URL、API Key、默认模型；模型列表从服务商 `/v1/models` 动态拉取。

### 启用步骤

```text
1. 设置 → AI → API Keys
   └─ 填写 Direct backend 的 key / URL / 默认模型

2. 设置 → 通用
   └─ 切换到对应 provider（首次切换需重启一次）

3. 打开 Agent Mode
   └─ 对话即走你自己的 Key
```

### 能力与边界

| 状态 | 说明 |
|:----:|------|
| **已支持** | 11 个工具：读文件、跑 shell、grep、glob、打 diff、向用户提问、长任务 shell 交互、MCP 资源与工具调用等 |
| **MCP** | 客户端侧执行，复用本机 `~/.warp/.mcp.json` |
| **暂不覆盖** | Computer Use / Workflow agent / Drive 等高阶云功能在 direct 模式下为空响应，不影响主对话 |
| **已知毛刺** | 多并行 `tool_call` 时偶发 result 时序竞态（日志可能出现 `stubbed N missing`，模型会被引导重试） |

源码入口：`app/src/server/direct_backend/` · `crates/ai/src/direct_backend/`

---

## 与官方版的差异

| | 官方 Warp | 本 fork（warp-cn） |
|:---:|:---------:|:------------------:|
| **默认语言** | 英文 | 中文（可切换） |
| **UI 字符串** | 英文 | ~7000+ 条 Fluent 汉化（`warp_i18n`） |
| **自动更新** | 官方通道 | GitHub Releases + minisign，无需开发者证书 |
| **macOS 凭据** | 钥匙串 | AES-256-GCM 本地加密文件（避免升级弹窗） |
| **Windows 凭据** | — | Windows DPAPI 加密落盘 |
| **AI 后端** | Warp 云端 | 可选 Direct LLM（自备 Key） |
| **Bundle ID** | 官方 ID | `dev.warp.WarpCn` / 应用名 `Warp-cn` |

上游通用说明、贡献流程与许可证原文 → [`README_en.md`](./README_en.md)

---

## 平台说明

<details>
<summary><strong>macOS 凭据（为什么不用钥匙串）</strong></summary>

<br/>

ad-hoc 签名的二进制每次发版 CDHash 会变，钥匙串 ACL 无法跨版本共享，升级时会反复弹「想要使用钥匙串中的机密信息」。因此本 fork **完全不用钥匙串**，登录 token、AI API Key、MCP OAuth 凭据统一写到：

```text
~/Library/Application Support/dev.warp.WarpCn/dev.warp.WarpCn-<KEY>
```

权限 `0600`，算法 AES-256-GCM（与 Linux fallback 同款）。

**从旧版（曾用钥匙串）升级时，需一次性完成：**

- 重新登录账号
- 重填 AI API Key
- 重新授权 MCP OAuth

旧钥匙串条目不会自动删除（避免再弹窗）。可在「钥匙串访问」中搜索 `dev.warp.WarpCn` 手动清理；不删也无功能影响。

</details>

<details>
<summary><strong>Windows 运行时布局</strong></summary>

<br/>

`warp-oss.exe` 依赖下列文件，**必须按此布局放置**，否则会报 `Failed to load ConPTY library module` 或缺 VC++ 运行时：

```text
warp-oss.exe
conpty.dll
dxcompiler.dll
dxil.dll
vcruntime140.dll
vcruntime140_1.dll
msvcp140.dll
x64\
  └─ OpenConsole.exe
resources\
```

以 `script/windows/windows-installer.iss` 为准。便携分发请用安装包产物，不要只拷 exe 同级几个 DLL。

Windows 上中文渲染会预加载微软雅黑 / 宋体 + Segoe UI Emoji，避免豆腐块。

安装包流水线：`script/windows/`（`bundle.ps1` + InnoSetup）。

</details>

---

## 从源码构建

### macOS / Linux

```bash
./script/bootstrap   # 平台依赖与工具链
./script/run         # 构建并启动
./script/presubmit   # 格式 / clippy / 测试
```

### Windows

默认需要 **Visual Studio 2022 Build Tools**（含 Windows SDK）与 PATH 上的 `protoc`：

```sh
cargo build --release --bin warp-oss --features gui
```

也可用免 VS 的便携工具链（`clang-cl` / `lld-link` / `llvm-rc` + xwin）：设置 `WARP_RC` 指向资源编译器即可。

详见 [`docs/building-windows-portable.md`](docs/building-windows-portable.md)。

<details>
<summary><strong>维护者：启用更新通道</strong></summary>

<br/>

```bash
script/generate_update_keys.sh
```

| 产物 | 放置位置 |
|------|----------|
| 公钥 | 提交到 `script/warp-update.pub` |
| 私钥 | GitHub Actions Secret `MINISIGN_SECRET_KEY` |

打 `v*` tag 后，CI 会产出带 minisign 签名的发布资产。

</details>

---

## 贡献

欢迎修翻译、补术语、报 bug、提 PR。

| 用途 | 入口 |
|:----:|------|
| 贡献者指南 | [`docs/i18n.md`](docs/i18n.md) |
| 术语锁定 | [`crates/warp_i18n/GLOSSARY.md`](crates/warp_i18n/GLOSSARY.md) |
| 与上游 merge 策略 | [`crates/warp_i18n/MERGE_NOTES.md`](crates/warp_i18n/MERGE_NOTES.md) |
| i18n lint（CI 同款） | `cargo xtask check-i18n --mode hard` |
| en / zh-CN 对齐 | `cargo xtask check-i18n --check-parity` |

含 UI 字符串的改动建议拆成两步：

```text
结构步  →  t!("...") + bundles/en/*.ftl
翻译步  →  填充 bundles/zh-CN/*.ftl
```

问题与建议 → [开 Issue](https://github.com/Heartcoolman/warp-cn/issues)

---

## 许可证

与上游一致：

| 范围 | 协议 |
|:----:|:----:|
| UI 框架（`warpui` / `warpui_core`） | [MIT](LICENSE-MIT) |
| 其余代码 | [AGPL v3](LICENSE-AGPL) |

本仓库基于 [warpdotdev/warp](https://github.com/warpdotdev/warp) 社区维护，**与 Warp 官方无隶属关系**。

上游 README、构建与贡献流程 → [`README_en.md`](./README_en.md)

<div align="center">

<br/>

**如果这个项目对你有帮助，欢迎点一颗 Star**

[![Star History Chart](https://api.star-history.com/svg?repos=Heartcoolman/warp-cn&type=Date)](https://star-history.com/#Heartcoolman/warp-cn&Date)

</div>
