---
id          = "debug_panics"
name        = "Fix the Panics"
language    = "rust"
difficulty  = 3
description = "Three functions panic on edge cases. Make them crash-proof."
topics      = ["debugging", "panics", "Option", "overflow", "bytes vs chars"]
---

# Fix the Panics

A small utility program compiles and works with expected input. It does crash when used with edge cases though. All three functions already have a **safe signature** (they return `Option`), but the bodies still allow the program to panic.

The failing tests show which inputs kill which functions.

1. `first_and_last` panics on an **empty slice**. Make sure the function actually returns `None` for these. `slice.first()` and `slice.last()` might end up being helpful.
2. `average` panics on the empty slice  *and* if the slice contains entries whose **sum overflows `u32::MAX`**.
3. `nth_char` doesn't handle **multi-byte characters** correctly and panics if the index is out-of-bounds. Remember, Bytes are not equal to chars as some chars take up multiple bytes (e.g. emojis). `str::chars().nth(i)` might be helpful.

## Expected Result

All three tests pass.
