##################################################
# Variables
#
open := if os() == "linux" {
  "xdg-open"
} else if os() == "macos" {
  "open"
} else {
  "start \"\" /max"
}

project_dir   := justfile_directory()
project_name  := file_stem(justfile_directory())

rust_version  := "rustc --version"
rustup_url    := "https://sh.rustup.rs"

gh_version    := "gh --version | head -n1"

lq_version    := "lq --version"
lq_github     := "https://github.com/tschinz/langquest"

exercises_dir := "exercises"
labs_dir      := "labs"

set shell := ["bash", "-uc"]
set windows-shell := ["cmd.exe", "/c"]

##################################################
# COMMANDS
#
# List all commands
@default:
  just --list

# Information about the environment
@info:
  echo "Environment Informations\n------------------------\n"
  echo "    OS          : {{os()}}({{arch()}})"
  echo "    Open        : {{open}}"
  echo "    Rust        : `{{rust_version}}`"
  echo "    GitHub CLI  : `{{gh_version}}`"
  echo "    LangQuest   : `{{lq_version}}`"
  echo "    Projectdir  : {{project_dir}}"
  echo "    Projectname : {{project_name}}"
  echo "    Exercises   : {{exercises_dir}}"
  echo "    Labs        : {{labs_dir}}"

# install required sw (Rust toolchain + GitHub CLI + LangQuest)
[linux]
[macos]
@install:
  echo "Install Rust (rustup)"
  command -v cargo >/dev/null 2>&1 || curl --proto '=https' --tlsv1.2 -sSf {{rustup_url}} | sh -s -- -y
  echo "Install GitHub CLI (gh)"
  command -v gh >/dev/null 2>&1 || { if command -v brew >/dev/null 2>&1; then brew install gh; elif command -v apt-get >/dev/null 2>&1; then sudo apt-get update && sudo apt-get install -y gh; elif command -v dnf >/dev/null 2>&1; then sudo dnf install -y gh; elif command -v pacman >/dev/null 2>&1; then sudo pacman -S --noconfirm github-cli; else echo "Please install gh manually: https://github.com/cli/cli#installation"; fi; }
  echo "Authenticate GitHub CLI (gh) - required by LangQuest"
  gh auth status >/dev/null 2>&1 || gh auth login
  echo "Install LangQuest"
  . "$HOME/.cargo/env" && cargo install --git {{lq_github}}

# install required sw (Rust toolchain + GitHub CLI + LangQuest)
[windows]
@install:
  echo "Install Rust (rustup)"
  where cargo >nul 2>nul || winget install --id Rustlang.Rustup -e --accept-source-agreements --accept-package-agreements
  echo "Install GitHub CLI (gh)"
  where gh >nul 2>nul || winget install --id GitHub.cli -e --accept-source-agreements --accept-package-agreements
  echo "Authenticate GitHub CLI (gh) - required by LangQuest"
  gh auth status >nul 2>nul || gh auth login
  echo "Install LangQuest"
  "%USERPROFILE%\.cargo\bin\cargo" install --git {{lq_github}}

# start LangQuest on the exercises
@ex-start dir=exercises_dir:
  lq --repo {{dir}}

# print current exercise and overall progress
@ex-status dir=exercises_dir:
  lq --repo {{dir}} status

# wipe all progress and start fresh
@ex-reset dir=exercises_dir:
  lq --repo {{dir}} --reset
