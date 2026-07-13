---
id          = "impp_string_formatting"
name        = "String Formatting"
language    = "rust"
difficulty  = 2
description = "Use format! with width, precision, zero-padding, and number bases."
topics      = ["formatting", "format", "strings"]
---

# String Formatting

Implement four functions that each return a formatted `String`:

1. `zero_pad(n: u32) -> String` — `n` right-aligned in width 8, padded with
   leading zeros. `42 → "00000042"`.
2. `as_binary(n: u8) -> String` — `n` as 8-bit binary with leading zeros.
   `5 → "00000101"`.
3. `two_decimals(x: f64) -> String` — `x` with exactly 2 decimal places.
   `3.14159 → "3.14"`.
4. `as_hex(n: u16) -> String` — `n` as lowercase hex with a `0x` prefix, padded
   to 4 hex digits. `255 → "0x00ff"`.

## Expected Result

```
zero_pad(42)        → "00000042"
as_binary(5)        → "00000101"
two_decimals(3.14159) → "3.14"
as_hex(255)         → "0x00ff"
```
