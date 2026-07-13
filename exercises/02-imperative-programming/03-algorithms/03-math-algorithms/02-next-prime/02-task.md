---
id          = "impp_next_prime"
name        = "Next Prime"
language    = "rust"
difficulty  = 2
description = "Find the smallest prime >= n."
topics      = ["loops", "math"]
---

# Next Prime

Write `next_prime(n: u64) -> u64` that returns the smallest prime that
is **greater than or equal to** `n`.

Start from `n` and call `is_prime` in a loop — keep incrementing until
you find one.

Expected Result:
```
next_prime(10) → 11
next_prime(13) → 13
next_prime(1)  → 2
```
