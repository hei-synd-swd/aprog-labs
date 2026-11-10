---
id          = "debug_missing_dep"
name        = "The Missing Dependency"
language    = "rust"
source      = "Cargo.toml"
difficulty  = 1
description = "Read a Cargo build error and repair the manifest by adding a missing crates.io dependency."
topics      = ["cargo", "Cargo.toml", "dependencies", "crates.io", "E0432"]
---

# The Missing Dependency

Someone cloned a tiny dice-roller project and but it refuses to build. The source code (`main.rs`) is finished and correct. `Cargo.toml` is the only thing that's broken.

1. Switch to the debug tab and read the error.
2. Identify the crate the code depends on and add it to `Cargo.toml` under `[dependencies]`, pinned to version `0.10`.
3. Save the file. LangQuest runs `cargo test` in this folder, so the saved `Cargo.toml` is verified automatically (optionally run `cargo run` yourself to see `rolled: N` for a die value 1..=6).

## Expected Result

`cargo build` succeeds, `cargo run` prints `rolled: N` with N in 1..=6, and LangQuest shows all tests green.

> The first build downloads the crate from crates.io, so a working internet connection is required.

