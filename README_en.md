<p align="center">
  <a href="./README.md">中文</a> · <strong>English</strong>
</p>

<a href="https://www.warp.dev">
    <img width="1024" alt="Warp Agentic Development Environment product preview" src="https://github.com/user-attachments/assets/9976b2da-2edd-4604-a36c-8fd53719c6d4" />
</a>
&nbsp;
<p align="center">
  <a href="https://www.warp.dev"><img height="20" alt="Built with Warp" src="https://raw.githubusercontent.com/warpdotdev/brand-assets/main/Github/Built-With-Warp-Export@2x.png" /></a>
  &nbsp;
  <a href="https://oz.warp.dev"><img height="20" alt="Powered by Oz" src="https://raw.githubusercontent.com/warpdotdev/brand-assets/main/Github/Powered-By-Oz-Export@2x.png" /></a>
</p>

<p align="center">
  <a href="https://www.warp.dev">Website</a>
  ·
  <a href="https://www.warp.dev/code">Code</a>
  ·
  <a href="https://www.warp.dev/agents">Agents</a>
  ·
  <a href="https://www.warp.dev/terminal">Terminal</a>
  ·
  <a href="https://www.warp.dev/drive">Drive</a>
  ·
  <a href="https://docs.warp.dev">Docs</a>
  ·
  <a href="https://www.warp.dev/blog/how-warp-works">How Warp Works</a>
</p>

> [!NOTE]
> OpenAI is the founding sponsor of the new, open-source Warp repository, and the new agentic management workflows are powered by GPT models.

<h1></h1>

## About

[Warp](https://www.warp.dev) is an agentic development environment, born out of the terminal. Use Warp's built-in coding agent, or bring your own CLI agent (Claude Code, Codex, Gemini CLI, and others).

## Installation

You can [download Warp](https://www.warp.dev/download) and [read our docs](https://docs.warp.dev/) for platform-specific instructions.

## Warp Contributions Overview Dashboard

Explore [build.warp.dev](https://build.warp.dev) to:
- Watch thousands of Oz agents triage issues, write specs, implement changes, and review PRs
- View top contributors and in-flight features
- Track your own issues with GitHub sign-in
- Click into active agent sessions in a web-compiled Warp terminal

## Oz for OSS

Maintaining a popular open-source project? [Apply for Oz credits](https://tally.so/r/LZWxqG) to explore [Oz for OSS](https://github.com/warpdotdev/oz-for-oss).

Oz for OSS is our partner program for bringing the same agentic open-source management workflows used in this repository to select partner repositories. We work directly with maintainers to implement workflows for issue triage, PR review, community management, and contributor coordination in a way that fits each project.

## Licensing

Warp's UI framework (the `warpui_core` and `warpui` crates) are licensed under the [MIT license](LICENSE-MIT).

The rest of the code in this repository is licensed under the [AGPL v3](LICENSE-AGPL).

## Open Source & Contributing

Warp's client codebase is open source and lives in this repository. We welcome community contributions and have designed a lightweight workflow to help new contributors get started. For the full contribution flow, read our [CONTRIBUTING.md](CONTRIBUTING.md) guide.

> [!TIP]
> **Chat with contributors and the Warp team** in the [`#oss-contributors`](https://warpcommunity.slack.com/archives/C0B0LM8N4DB) Slack channel — a good place for ad-hoc questions, design discussion, and pairing with maintainers. New here? [Join the Warp Slack community](https://go.warp.dev/join-preview) first, then jump into `#oss-contributors`.

### Issue to PR

Before filing, [search existing issues](https://github.com/warpdotdev/warp/issues?q=is%3Aissue+is%3Aopen+sort%3Areactions-%2B1-desc) for your bug or feature request. If nothing exists, [file an issue](https://github.com/warpdotdev/warp/issues/new/choose) using our templates. Security vulnerabilities should be reported privately as described in [CONTRIBUTING.md](CONTRIBUTING.md#reporting-security-issues).

Once filed, a Warp maintainer reviews the issue and may apply a readiness label: [`ready-to-spec`](https://github.com/warpdotdev/warp/issues?q=is%3Aissue+is%3Aopen+label%3Aready-to-spec) signals the design is open for contributors to spec out, and [`ready-to-implement`](https://github.com/warpdotdev/warp/issues?q=is%3Aissue+is%3Aopen+label%3Aready-to-implement) signals the design is settled and code PRs are welcome. Anyone can pick up a labeled issue — mention **@oss-maintainers** on an issue if you'd like it considered for a readiness label.

### Building the Repo Locally

To build and run Warp from source:

```bash
./script/bootstrap   # platform-specific setup
./script/run         # build and run Warp
./script/presubmit   # fmt, clippy, and tests
```

See [AGENTS.md](AGENTS.md) for the full engineering guide, including coding style, testing, and platform-specific notes.

## Joining the Team

Interested in joining the team? See our [open roles](https://www.warp.dev/careers).

## Support and Questions

1. See our [docs](https://docs.warp.dev/) for a comprehensive guide to Warp's features.
2. Join our [Slack Community](https://go.warp.dev/join-preview) to connect with other users and get help from the Warp team — contributors hang out in [`#oss-contributors`](https://warpcommunity.slack.com/archives/C0B0LM8N4DB).
3. Try our [Preview build](https://www.warp.dev/download-preview) to test the latest experimental features.
4. Mention **@oss-maintainers** on any issue to escalate to the team — for example, if you encounter problems with the automated agents.

## Code of Conduct

We ask everyone to be respectful and empathetic. Warp follows the [Code of Conduct](CODE_OF_CONDUCT.md). To report violations, email warp-coc at warp.dev.

## Open Source Dependencies

We'd like to call out a few of the [open source dependencies](https://docs.warp.dev/help/licenses) that have helped Warp to get off the ground:

- [Tokio](https://github.com/tokio-rs/tokio)
- [NuShell](https://github.com/nushell/nushell)
- [Fig Completion Specs](https://github.com/withfig/autocomplete)
- [Warp Server Framework](https://github.com/seanmonstar/warp)
- [Alacritty](https://github.com/alacritty/alacritty)
- [Hyper HTTP library](https://github.com/hyperium/hyper)
- [FontKit](https://github.com/servo/font-kit)
- [Core-foundation](https://github.com/servo/core-foundation-rs)
- [Smol](https://github.com/smol-rs/smol)

---

## warp-cn fork additions

This repository is the [warp-cn community fork](https://github.com/Heartcoolman/warp-cn). Fork-only additions sit behind `#[cfg(feature = "direct_llm_backend")]` or in fork-only paths, so upstream-feature-off builds are unaffected. The primary (Chinese) README is [`README.md`](./README.md).

### Direct LLM Backend (BYOK) — initial preview

> ⚠️ **Initial release (v0.1).** Core paths work end-to-end (project evaluation, file reads, shell, MCP), but tool-result race conditions and a few rough edges are still being smoothed out. Please file issues in the fork repo.

Lets users point Warp at **their own LLM API key** (Anthropic / OpenAI-compatible incl. DeepSeek / Google Gemini) and run the full agent loop **without ever talking to Warp Cloud**. Landed via merge `84f9ef23` on `master` (9 commits squashed under `feat/direct-llm-backend`).

**Supported providers**

| Provider | Default base URL | Notes |
|---|---|---|
| Anthropic | `https://api.anthropic.com/v1` | Native SSE |
| OpenAI-compatible | `https://api.openai.com/v1` | Works with DeepSeek (`https://api.deepseek.com/v1`) and any `/v1/chat/completions` endpoint |
| Google Gemini | `https://generativelanguage.googleapis.com/v1beta` | `?alt=sse` streaming |

Each provider keeps its own base URL + key + default model; the model dropdown is populated dynamically from the provider's `/v1/models` endpoint.

**Enabling**

1. **Settings → AI → API Keys** has a new "Direct backend" section — fill in any provider's key, URL, and default model.
2. **Settings → General** switch the active provider (one-time restart on first switch).
3. From then on, Agent Mode talks to your own API key directly. **No Warp account login required.**

**Tool set (11)**

`read_files` · `run_shell_command` · `grep` · `file_glob` · `apply_file_diffs` · `ask_user_question` · `write_to_long_running_shell_command` · `read_shell_command_output` · `transfer_shell_command_control_to_user` · `read_mcp_resource` · `call_mcp_tool`

MCP runs client-side and reuses the local `~/.warp/.mcp.json`; no MCP server-side re-wiring needed.

**Known limitations**

- **Reasoning models**: DeepSeek-R1 / o1-style `reasoning_content` echo-back is supported; other reasoning models surface as errors if rejected.
- **Tool-result race**: client occasionally fires the next request before all parallel tool results return; the server stubs missing IDs with a transient-retry hint to keep the model from hallucinating. Look for `DirectBackend OpenAI: stubbed N missing` in `~/Library/Logs/warp-oss.log`.
- **Computer Use / Drive / Workflow agents** and other Warp-cloud-only auxiliaries return empty in direct mode (does not affect the main chat loop).
- **Cost / token accounting**: only input / output tokens are reported in `StreamFinished.token_usage`; cost is not computed.

**Security / permissions**

- Four permission gates (`read_files`, `mcp`, `file_write`, `pty`, `execute_commands`) coerce upstream `AlwaysAsk` to `AgentDecides` (or `AskOnFirstWrite` for PTY) when the feature is on, so own-LLM read-only inspection isn't trapped by a popup the model can never satisfy.
- All fork behavior is gated by `#[cfg(feature = "direct_llm_backend")]`; upstream-feature-off builds are unchanged.
- API keys are persisted to `~/Library/Application Support/dev.warp.WarpCn/` as AES-256-GCM encrypted files (no macOS Keychain).

**Source layout**

- Entry: `app/src/server/direct_backend/`
- Config schema: `crates/ai/src/direct_backend/config.rs`
- Multi-provider drivers / SSE: `app/src/server/direct_backend/multi_agent/`
- Cargo feature: `app/Cargo.toml` → `direct_llm_backend = [...]` (on by default in fork builds)
