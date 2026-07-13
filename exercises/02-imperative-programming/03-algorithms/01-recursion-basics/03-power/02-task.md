---
id          = "impp_power"
name        = "Power"
language    = "rust"
difficulty  = 2
description = "Compute base^exp using recursion."
topics      = ["recursion", "math"]
---

# Power

Write a recursive function `power(base: u64, exp: u32) -> u64` that
returns `base` raised to `exp`.

- Base case: `power(base, 0) = 1`
- Recursive case: `power(base, exp) = base × power(base, exp - 1)`

Expected Result:
```
power(2, 10) → 1024
power(5, 3)  → 125
power(3, 4)  → 81
```
