<div align="center">

<h1>ASIMOV</h1>
<h2><em>Agentic Coding for Builders who Ship</em></h2>
<img src="public/Ship.png" alt="Rustle on the ship" width="350" />

<p>
    <a href="https://github.com/kuberwastaken/asimov"><img src="https://img.shields.io/badge/Built_with-Rust-CE4D2B?style=for-the-badge&logo=rust&logoColor=white" alt="Built with Rust"></a>
    <a href="https://github.com/kuberwastaken/asimov"><img src="https://img.shields.io/badge/Version-0.0.9-2E8B57?style=for-the-badge" alt="Version 0.0.9"></a>
    <a href="https://github.com/kuberwastaken/asimov/blob/main/LICENSE.md"><img src="https://img.shields.io/badge/License-GPL--3.0-blue?style=for-the-badge" alt="GPL-3.0 License"></a>
</p>

<br />

<img src="public/screenshot.png" alt="ASIMOV in action" width="1080" />
</div>

---

Asimov is an **open-source, multi-provider terminal coding agent** built from the ground up in Rust. It started as a clean-room reimplementation of Claude Code's behavior (from [spec](https://github.com/kuberwastaken/asimov/tree/main/spec)) and has since evolved into an amazing TUI pair programmer with multi-provider support, a rich UI, plugin system, a companion named Rustle, chat forking, memory consolidation, and much more.

It's fast, it's memory-efficient, it's yours to run however you want, and there's no tracking or telemetry.

---

> [!NOTE]
> **Recent Updates:**
> - **/goal support:** Try out `/goal <objective>` to see asimov keep working an objective, spanning multiple turns instead of stopping after one normal turn. `[EXPERIMENTAL]`
>
> - **Managed Agents Preview:** Run `/managed-agents` to create a better agentic loop with a Manager-Executor relation and dramatically improved performance for fractions of the cost from running a larger model. Choose from 6 pre-built templates or build your own.`[EXPERIMENTAL]`
>
> - Speech modes: Try `/rocky` and `/caveman` to hear the difference! `/normal` to go back.

---

# Getting Started

## Quick install (one-liner)

**Linux / macOS:**

```bash
curl -fsSL https://github.com/kuberwastaken/asimov/releases/latest/download/install.sh | bash
```

**Windows (PowerShell):**

```powershell
irm https://github.com/kuberwastaken/asimov/releases/latest/download/install.ps1 | iex
```

This drops `asimov` into `~/.asimov/bin` (or `%USERPROFILE%\.asimov\bin` on Windows) and adds it to your `PATH` automatically. Open a new terminal and run `asimov`.

To upgrade later, run:

```bash
asimov upgrade
```

> Pin a specific version with `--version 0.0.9` on either installer, or `asimov upgrade --version 0.0.9`.

## Manual download

If you'd rather grab the binary yourself, the latest archives are on [**GitHub Releases**](https://github.com/kuberwastaken/asimov/releases):

| Platform | Archive |
|----------|---------|
| **Windows** x86_64 | `asimov-windows-x86_64.zip` |
| **Linux** x86_64 | `asimov-linux-x86_64.tar.gz` |
| **Linux** aarch64 | `asimov-linux-aarch64.tar.gz` |
| **macOS** Intel | `asimov-macos-x86_64.tar.gz` |
| **macOS** Apple Silicon | `asimov-macos-aarch64.tar.gz` |

Each archive contains a single `asimov` (or `asimov.exe`) binary. Extract it and put it on your `PATH`.

## Build from source

```bash
git clone https://github.com/kuberwastaken/asimov.git
cd asimov/src-rust
cargo build --release --package asimov

# Binary is at target/release/asimov
```

**Raspberry Pi / systems without ALSA** (e.g. Debian Trixie, headless servers):

```bash
# Build without voice/microphone support — no libasound2-dev required
cargo build --release --package asimov --no-default-features
```

## First run

```bash
# Set your API key (or use /connect inside Asimov to configure)
export ANTHROPIC_API_KEY=sk-ant-...

# Start Asimov
asimov

# Or run a one-shot headless query
asimov -p "explain this codebase"
```

## Devcontainer setup

After cloning this repository, open it in VS Code and use Reopen in Container to start the development environment.

Prerequisites:
- Docker installed on your host machine: https://www.docker.com/products/docker-desktop/

GPG and SSH forwarding is enabled in the devcontainer, given you have it set up on your host machine. Follow [this guide](https://code.visualstudio.com/remote/advancedcontainers/sharing-git-credentials) if you need help with that.

### Devcontainer features

- Base image: `rust:1-bullseye`.
- Preinstalled build dependencies: `gnupg2`, `libasound2-dev`, `libxdo-dev`, and `pkg-config`.
- Devcontainer features enabled: `common-utils` (with `vscode` user `uid/gid 1000` and Zsh install disabled), `git`, and `docker-outside-of-docker` (`moby: false`).
- Runs as `vscode` user by default.
- Persistent Cargo caches via named volumes for `/usr/local/cargo/registry` and `/usr/local/cargo/git`.
- Binds local `.asimov` into `/home/vscode/.asimov` for local settings/session history access.
- Sets `GNUPGHOME=/home/vscode/.gnupg` and prepends `src-rust/target/debug` and `src-rust/target/release` to `PATH`.
- Post-create setup creates and permissions `.gnupg`, and fixes ownership for `/usr/local/cargo`.
- VS Code setting `terminal.integrated.inheritEnv` is enabled.

## Documentation

For more info on how to configure Asimov, [head over to our docs](https://asimov.kuber.studio/docs).

>**PS:** The original breakdown of the findings from Claude Code's source that started this project is on [my blog](https://kuber.studio/blog/AI/Claude-Code's-Entire-Source-Code-Got-Leaked-via-a-Sourcemap-in-npm,-Let's-Talk-About-it) - the full technical writeup of what was found, how the leak happened, and what it revealed.

---

## Contributing

Asimov is built for the community, by the community and we'd love your help making it better.

[Open an issue](https://github.com/Kuberwastaken/asimov/issues/new) for bugs, ideas, or questions, or [Raise a PR](https://github.com/Kuberwastaken/asimov/pulls/new) to fix bugs, add features, or improve documentation.

---

## Important Notice

This repository does not hold a copy of the proprietary Claude Code TypeScript source code.
This is a **clean-room Rust reimplementation** of Claude Code's behavior.

The process was explicitly two-phase:

**Specification** [`spec/`](https://github.com/kuberwastaken/asimov/tree/main/spec) — An AI agent analyzed the source and produced exhaustive behavioral specifications and improvements, deviated from the original: architecture, data flows, tool contracts, system designs. No source code was carried forward.

**Implementation** [`src-rust/`](https://github.com/kuberwastaken/asimov/tree/main/src-rust) — A separate AI agent implemented from the spec alone, never referencing the original TypeScript. The output is idiomatic Rust that reproduces the behavior, not the expression.

This mirrors the legal precedent established by Phoenix Technologies v. IBM (1984) — clean-room engineering of the BIOS — and the principle from Baker v. Selden (1879) that copyright protects expression, not ideas or behavior.

---

