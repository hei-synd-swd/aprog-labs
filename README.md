<p align="center">
  <img src="https://github.com/hei-templates/hei-synd-logos/blob/a47918f96647efdc10d30127b5e194b1f1005cff/hei-en.svg" alt="HEI-Vs Logo" width="350">
</p>

# HEI-Vs Engineering School - AProg Advanced Programming Labs

<p align="center">
  <img src="https://github.com/hei-templates/hei-synd-logos/blob/a47918f96647efdc10d30127b5e194b1f1005cff/orientations/it-logo.svg" alt="Systems Engineering Infotronics Logo" width="350">
</p>

<p align="center">
  <img src="https://github.com/hei-templates/hei-synd-logos/blob/a47918f96647efdc10d30127b5e194b1f1005cff/courses/aprog.svg" alt="Advanced Programming Logo" width="150">
</p>

This repository holds the **hands-on part** of the **AProg Advanced Programming** course of the
[HEI-Vs Engineering School](https://synd.hevs.io) in Sion, Switzerland - the interactive
[LangQuest](https://github.com/tschinz/langquest) exercises and the laboratory starter code.
The lecture slides, exercise statements and lab documents live in the companion
[aprog-docs](https://github.com/hei-synd-swd/aprog-docs) repository.

## What's Inside

| Folder        | Content                                                                                          |
|---------------|--------------------------------------------------------------------------------------------------|
| `exercises/`  | Interactive [LangQuest](https://github.com/tschinz/langquest) exercises, grouped by chapter. Solutions are encrypted and revealed by LangQuest once you pass. |
| `labs/`       | Laboratory starter projects (Cargo projects, source templates) you build on during the labs.     |
| `.zed/`       | Shared [Zed](https://zed.dev) editor settings for a consistent setup.                            |
| `AGENTS.md`   | The rules any AI assistant **must** follow when you use it for this course (see below).           |

## Prerequisites

You need the [Rust](https://www.rust-lang.org/) toolchain, the [`just`](https://github.com/casey/just)
command runner, the [GitHub CLI](https://cli.github.com/) (`gh`), and the
[LangQuest](https://github.com/tschinz/langquest) exercise runner. LangQuest uses `gh` to verify your
identity, so it must be **installed and authenticated** before you start the exercises.

### Linux / macOS

```bash
# 1. Install Rust (includes cargo, rustc, rustup)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Add the formatter, linter and the just command runner
rustup component add rustfmt clippy
cargo install just

# 3. Install the GitHub CLI
#    macOS:  brew install gh
#    Debian/Ubuntu:  sudo apt install gh
#    Fedora:  sudo dnf install gh
#    Arch:    sudo pacman -S github-cli
#    (see https://github.com/cli/cli#installation for other systems)

# 4. Authenticate the GitHub CLI (opens a browser, follow the prompts)
gh auth login

# 5. Install everything else the labs need (Rust toolchain check + LangQuest)
just install
```

### Windows

Install Rust from [rustup.rs](https://rustup.rs) and the [GitHub CLI](https://cli.github.com/) (e.g.
`winget install --id GitHub.cli`), then from PowerShell:

```powershell
rustup component add rustfmt clippy
cargo install just
gh auth login   # authenticate the GitHub CLI (opens a browser)
just install
```

> LangQuest relies on `gh` being logged in. If `gh auth login` was skipped, the exercises and commands
> like `just ex-reset` will fail with an authentication error - run `gh auth status` to check.

## Usage - `just` Commands

All common tasks are wrapped in the [`justfile`](justfile). Run `just` on its own to list them:

| Command                  | Description                                                             |
|--------------------------|-------------------------------------------------------------------------|
| `just`                   | List all available commands.                                            |
| `just info`              | Show your environment (OS, Rust version, LangQuest version, paths).     |
| `just install`           | Install the required software (Rust toolchain + GitHub CLI + LangQuest) and authenticate `gh`. |
| `just ex-start`          | Start LangQuest on the `exercises/` and work through them interactively.|
| `just ex-status`         | Print the current exercise and your overall progress.                   |
| `just ex-reset`          | Wipe your local progress and start fresh.                               |

The exercise commands accept an optional directory argument, e.g. `just ex-start exercises`.

### Working Through the Exercises

```bash
just ex-start
```

LangQuest walks you through the exercises one by one: it opens the task, watches the file you edit,
runs the tests, and only unlocks the next exercise once yours passes. Use `just ex-status` at any time
to see where you are. Your progress is stored locally in `exercises/lq.toml` - it is personal to you
and does not need to be committed.

## Using AI Assistants - Read This First

You are **allowed and encouraged** to use AI assistants (Claude, ChatGPT, Copilot, …) while working
through this course - but in **reference mode**. The AI is permitted to explain, define, summarize and translate. This repository ships an [`AGENTS.md`](AGENTS.md)
file that configures assistants to teach and guide you instead of handing you the answer.

**On every AI interaction for this course, you must use the [`AGENTS.md`](AGENTS.md) rules.**

- With tools that read `AGENTS.md` automatically (e.g. agentic CLIs), keep the file at the repository
  root and run the assistant from there.
- With chat tools that do not, paste the contents of [`AGENTS.md`](AGENTS.md) at the start of your
  session as system/context instructions.

In short, the assistant should help you *understand* - explain concepts, ask guiding questions, point
you toward what to investigate - and must **not write or complete your solutions for you**. Getting a
copy-pasted answer defeats the purpose of the labs and will not help you in the exam.

## Issues and Support

If you hit a problem with the exercises or labs, open an issue in this repository and the teaching team
will help you.

## Find us on

[Systems Engineering](https://synd.hevs.io) &nbsp;&middot;&nbsp;
[Smart Process Lab](https://spl.hevs.io) &nbsp;&middot;&nbsp;
LinkedIn [HEI-Vs](https://www.linkedin.com/showcase/school-of-engineering-valais-wallis/) &nbsp;&middot;&nbsp;
Youtube [HES-SO Valais-Wallis](https://www.youtube.com/user/HESSOVS) &nbsp;&middot;&nbsp;
Twitter [@hessovalais](https://twitter.com/hessovalais) &nbsp;&middot;&nbsp;
Facebook [@hessovalais](https://www.facebook.com/hessovalais) &nbsp;&middot;&nbsp;
