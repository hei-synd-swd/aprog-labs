---
id          = "impp_digit_sum"
name        = "Digit Sum"
language    = "rust"
difficulty  = 1
description = "Sum the digits of a number."
topics      = ["loops", "math"]
---

# Digit Sum

Write `digit_sum(n: u64) -> u64` that returns the sum of the decimal
digits of `n`.

Use `n % 10` to get the last digit, `n / 10` to remove it.
`digit_sum(0)` should return `0`.

Expected Result:
```
digit_sum(123)  → 6
digit_sum(9999) → 36
digit_sum(0)    → 0
```
