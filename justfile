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
set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile", "-Command"]

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

# generate rust-project.json so rust-analyzer (Zed) works on the single-file exercises
[unix]
@ide-setup:
  sysroot="$(rustc --print sysroot)/lib/rustlib/src/rust/library"; \
  [ -d "$sysroot" ] || rustup component add rust-src; \
  { printf '{"sysroot_src":"%s","crates":[' "$sysroot"; \
    find {{exercises_dir}} -name '*.rs' -not -path '*/solution/*' -not -path '*/src/*' | sort \
    | awk 'BEGIN{s=""} {printf "%s{\"root_module\":\"%s\",\"edition\":\"2024\",\"deps\":[]}", s, $0; s=","}'; \
    printf ']}\n'; } > rust-project.json
  echo "rust-project.json written"

# generate rust-project.json so rust-analyzer (Zed) works on the single-file exercises
[windows]
[script('powershell', '-NoProfile', '-ExecutionPolicy', 'Bypass', '-File')]
[extension('.ps1')]
ide-setup:
  $sysroot = (rustc --print sysroot).Trim() + "/lib/rustlib/src/rust/library"
  if (-not (Test-Path $sysroot)) { rustup component add rust-src }
  $root = "{{project_dir}}"
  $crates = Get-ChildItem "$root/{{exercises_dir}}" -Recurse -Filter *.rs |
    Where-Object { $_.FullName -notmatch '[\\/](solution|src)[\\/]' } |
    Sort-Object FullName |
    ForEach-Object {
      @{
        root_module = $_.FullName.Substring($root.Length + 1).Replace('\', '/')
        edition     = "2024"
        deps        = @()
      }
    }
  @{ sysroot_src = $sysroot.Replace('\', '/'); crates = @($crates) } |
    ConvertTo-Json -Depth 5 -Compress |
    Set-Content "$root/rust-project.json" -Encoding ascii
  Write-Host "rust-project.json written"

# remove build artifacts and generated files (keeps LangQuest progress)
[unix]
@clean:
  find . -type d -name target -not -path './.git/*' -prune -exec rm -rf {} +
  find . \( -name .DS_Store -o -name '*.rs.bk' -o -name '*.pdb' -o -name '.rust-project.json' -o -name rust-project.json \) -not -path './.git/*' -delete
  find {{exercises_dir}} -name Cargo.lock -delete
  # lq's rustc output and temp dirs left behind by interrupted compiles
  find {{exercises_dir}} \( -name '.lq_*' -o -type d \( -name 'rmeta??????' -o -name 'rustc??????' \) \) -prune -exec rm -rf {} +
  echo "cleaned (run 'just ide-setup' to restore rust-analyzer config)"

# remove build artifacts and generated files (keeps LangQuest progress)
[windows]
[script('powershell', '-NoProfile', '-ExecutionPolicy', 'Bypass', '-File')]
[extension('.ps1')]
clean:
  Set-Location "{{project_dir}}"
  Get-ChildItem -Recurse -Force -Directory -Filter target |
    Where-Object { $_.FullName -notmatch '[\\/]\.git[\\/]' } |
    Remove-Item -Recurse -Force
  Get-ChildItem -Recurse -Force -File |
    Where-Object {
      $_.FullName -notmatch '[\\/]\.git[\\/]' -and
      ($_.Name -in '.DS_Store', '.rust-project.json', 'rust-project.json' -or $_.Name -like '*.rs.bk' -or $_.Name -like '*.pdb')
    } |
    Remove-Item -Force
  Get-ChildItem {{exercises_dir}} -Recurse -Filter Cargo.lock | Remove-Item -Force
  # lq's rustc output and temp dirs left behind by interrupted compiles
  @(Get-ChildItem {{exercises_dir}} -Recurse -Force) |
    Where-Object { $_.Name -like '.lq_*' -or ($_.PSIsContainer -and $_.Name -match '^(rmeta|rustc)[A-Za-z0-9]{6}$') } |
    Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
  Write-Host "cleaned (run 'just ide-setup' to restore rust-analyzer config)"
