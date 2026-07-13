---
id          = "impp_shadowing"
name        = "Shadowing"
language    = "rust"
difficulty  = 2
description = "Re-bind one name across types to trim then parse a string."
topics      = ["shadowing", "let", "parse", "types"]
---

# Shadowing

Implement `parse_padded`, which turns a possibly padded numeric string into an
`i32`. Use **shadowing** to reuse the single name `input` as you transform it:
first trim the surrounding whitespace (still a `&str`), then parse it into an
`i32` (a new type, same name).

You may assume the trimmed input is always a valid integer.

## Expected Result

```
parse_padded("  42  ") → 42
parse_padded("7")      → 7
parse_padded("  -5")   → -5
```
