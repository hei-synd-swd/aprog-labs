---
id          = "impp_factorial"
name        = "Factorial"
language    = "rust"
difficulty  = 1
description = "Compute n! using recursion."
topics      = ["recursion", "math"]
---

# Factorial

Write a recursive function `factorial(n: u64) -> u64` that returns `n!`
(the product `1 × 2 × ... × n`).

- Base case: `0! = 1`
- Recursive case: `n! = n × (n - 1)!`

Expected Result:
```
factorial(0) → 1
factorial(5) → 120
factorial(10) → 3_628_800
```
