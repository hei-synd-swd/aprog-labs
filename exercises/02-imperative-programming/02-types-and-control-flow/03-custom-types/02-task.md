---
id          = "impp_custom_types"
name        = "Custom Types"
language    = "rust"
difficulty  = 2
description = "Use derive macros and type aliases with a custom enum."
topics      = ["enum", "derive", "debug", "clone", "partialeq", "type-alias"]
---

# Custom Types

Practice using `#[derive]` on a custom enum and creating a type alias.

1. Define an enum `Level` with variants `Low`, `Medium`, `High`.
   Derive `Debug`, `Clone`, `Copy`, and `PartialEq`.
2. Define a type alias `type Priority = u8`.
3. Write a function `level_priority(level: &Level) -> Priority` that maps:
   - `Low` → `1`
   - `Medium` → `2`
   - `High` → `3`
4. Write a function `can_skip(level: &Level) -> bool` that returns `true`
   only for `Low`.
5. In `main()`, use `{:?}` to print a level value (this requires `Debug`).

## Expected Result

```
level_priority(&Level::Low)    → 1
level_priority(&Level::High)   → 3
can_skip(&Level::Low)          → true
can_skip(&Level::High)         → false
```
