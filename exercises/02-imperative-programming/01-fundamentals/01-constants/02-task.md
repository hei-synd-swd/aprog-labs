---
id          = "impp_constants"
name        = "Constants"
language    = "rust"
difficulty  = 1
description = "Declare typed constants and a computed constant expression."
topics      = ["constants", "const", "types"]
---

# Constants

Define two constants used by a small scheduling tool.

1. `MAX_LOGIN_ATTEMPTS` — a `u8` constant equal to `5`.
2. `SECONDS_PER_DAY` — a `u32` constant **computed** from the three
   building-block constants already provided (`SECONDS_PER_MINUTE`,
   `MINUTES_PER_HOUR`, `HOURS_PER_DAY`).

## Expected Result

`MAX_LOGIN_ATTEMPTS` holds `5`, and `SECONDS_PER_DAY` evaluates to the number
of seconds in one day (`86_400`), derived only from the other constants — no
magic number.
