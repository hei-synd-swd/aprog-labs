---
id          = "impp_fib_memo"
name        = "Fibonacci Memoization"
language    = "rust"
difficulty  = 3
description = "Compute Fibonacci numbers using recursion with memoization."
topics      = ["recursion", "memoization"]
---

# Fibonacci — Memoization

Write `fib_memo(n: u32) -> u64` that returns the nth Fibonacci number
using **recursion with memoization**.

Use a `Vec<Option<u64>>` to cache computed values. The memoization
technique is covered in the theory file.

Expected Result:
```
fib_memo(0)  → 0
fib_memo(1)  → 1
fib_memo(10) → 55
```
