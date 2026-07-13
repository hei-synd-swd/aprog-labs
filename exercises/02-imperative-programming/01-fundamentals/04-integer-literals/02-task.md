---
id          = "impp_integer_literals"
name        = "Integer Literals"
language    = "rust"
difficulty  = 2
description = "Write the same values using hex, octal, binary, and byte literals."
topics      = ["literals", "integer", "bases"]
---

# Integer Literals

Define four constants, each written in the requested **notation** (not as a
plain decimal number):

1. `COLOR_MAGENTA: u32` — `16711935`, written as a **hex** literal with
   `_` digit separators (`0x...`).
2. `UNIX_PERMS: u32` — the file-permission value `493`, written as an **octal**
   literal (`0o...`).
3. `ALL_FLAGS: u8` — all eight bits set, written as a **binary** literal
   (`0b...`).
4. `LETTER_A: u8` — the ASCII code of `A`, written as a **byte literal**
   (`b'...'`).

## Expected Result

Each constant equals its decimal value: `16711935`, `493`, `255`, and `65`
respectively — but is written using the notation named above.
