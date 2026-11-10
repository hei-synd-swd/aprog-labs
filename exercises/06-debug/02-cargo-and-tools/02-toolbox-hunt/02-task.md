---
id          = "debug_toolbox"
name        = "Toolbox Hunt"
language    = "rust"
difficulty  = 3
description = "Use cargo test, clippy and fmt to find and fix failing tests, clippy lints and style violations."
topics      = ["cargo", "clippy", "fmt", "testing", "debugging"]
---

# Toolbox Hunt

A robot-dashboard utility that compiles, but fails in three ways:

- **two unit tests fail**,
- **`cargo clippy` reports lints**,
- **`cargo fmt --check` reports style violations**.

This directory contains a Cargo project so you're able to use following commands (and more!):

```bash
cargo test            # see which tests fail and why
cargo clippy          # let the lints point at suspicious lines
cargo fmt --check     # check formatting
```

This exercise uses all of the above commands to verify your solution.

1. Run `cargo test`. Two tests will fail. Read the `left` vs `right` values, find the bug and fix it. If the failing lines are not obvious, `cargo clippy` might help you find them.
2. Fix the remaining clippy lints.
3. Run `cargo fmt` to fix remaining style violations, then `cargo fmt --check` to confirm the file is formatted correctly.

## Expected Result

`cargo test` passes all unit tests, `cargo clippy -- -D warnings` passes silently, `cargo fmt --check` reports nothing, and LangQuest shows all checks green.

