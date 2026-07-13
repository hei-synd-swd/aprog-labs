---
id          = "impp_type_conversion"
name        = "Type Conversion"
language    = "rust"
difficulty  = 3
description = "Contrast a lossy `as` cast with a safe `TryFrom` conversion."
topics      = ["conversion", "as", "try_from", "option"]
---

# Type Conversion

Implement two conversions from `i32` to `u8`:

1. `truncate_to_u8(n: i32) -> u8` — use an `as` cast. This is fast but **lossy**
   when `n` does not fit in a `u8`.
2. `safe_to_u8(n: i32) -> Option<u8>` — use a checked conversion that returns
   `Some(v)` when `n` fits in a `u8` and `None` otherwise.

## Expected Result

```
truncate_to_u8(42)  → 42
truncate_to_u8(300) → 44     (300 wraps around)

safe_to_u8(200) → Some(200)
safe_to_u8(300) → None
safe_to_u8(-1)  → None
```
