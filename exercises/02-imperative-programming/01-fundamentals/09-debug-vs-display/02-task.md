---
id          = "impp_debug_display"
name        = "Debug vs Display"
language    = "rust"
difficulty  = 3
description = "Derive Debug and implement Display on a custom struct."
topics      = ["formatting", "debug", "display", "traits"]
---

# Debug vs Display

A `Person` struct is provided. Do the following:

1. Add `#[derive(Debug)]` above the struct so it works with `{:?}`.
2. Implement `fmt::Display` so `{}` prints as `"Name (Age)"`.
3. Write `format_debug` and `format_display`.

Expected Result:
```
format_debug(&p)   → contains "Person { name: \"Alice\", age: ... }"
format_display(&p) → "Alice (30)"
```
