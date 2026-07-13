---
id          = "impp_fib_iter"
name        = "Fibonacci Iterative"
language    = "rust"
difficulty  = 2
description = "Compute Fibonacci numbers using a bottom-up loop."
topics      = ["iteration"]
---

# Fibonacci — Iterative

Write `fib_iter(n: u32) -> u64` that returns the nth Fibonacci number
using **iteration** (bottom-up).

Start with `a = 0` and `b = 1`, then loop `n` times updating both:
- `(a, b) = (b, a + b)`

No recursion, no extra vector.

Expected Result:
```
fib_iter(0)  → 0
fib_iter(1)  → 1
fib_iter(10) → 55
```
