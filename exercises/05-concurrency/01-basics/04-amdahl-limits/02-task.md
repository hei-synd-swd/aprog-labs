---
id          = "conc_amdahl_limits"
name        = "Limits of Amdahl's Law"
language    = "rust"
difficulty  = 3
description = "Compute the maximum speedup and the number of processors needed to reach a target speedup."
topics      = ["amdahl", "speedup", "limit", "option", "loop"]
---

# Limits of Amdahl's Law

Adding processors has diminishing returns. Find the limit, and find out how many processors a given speedup costs.

The function `speedup(parallel_fraction, processors)` is given. It returns the speedup according to Amdahl's Law.

## Your Task

Replace the two `todo!()`. Do not change `speedup`, the signatures, `main` or the tests.

**`max_speedup(parallel_fraction: f64) -> f64`** returns the speedup with infinitely many processors.

1. Return `1.0 / (1.0 - parallel_fraction)`.

**`cores_needed(parallel_fraction: f64, target: f64) -> Option<u32>`** returns the smallest number of processors whose speedup is at least `target`.

1. If `target` is greater than or equal to `max_speedup(parallel_fraction)`, the target can never be reached: return `None`.
2. Otherwise start with `let mut processors = 1;`.
3. In a `while` loop, add 1 to `processors` as long as `speedup(parallel_fraction, processors)` is smaller than `target`.
4. Return `Some(processors)`.

```
max_speedup(0.5)       → 2.0
max_speedup(0.9)       → 10.0

cores_needed(0.9, 3.0)  → Some(4)     (3 processors give 2.5x, 4 give 3.08x)
cores_needed(0.9, 6.0)  → Some(14)
cores_needed(0.9, 0.8)  → Some(1)     (one processor is already enough)
cores_needed(0.9, 12.0) → None        (the limit is 10x)
cores_needed(0.5, 2.0)  → None        (the limit itself is never reached)
```

## Expected Result

- `max_speedup` returns the limit `1 / (1 - f)`.
- `cores_needed` returns `Some(p)` with the smallest sufficient `p`, and `None` when the target is at or above the limit.
- `main` prints the limit for a program that is 95 % parallel and the processors needed for a speedup of 4, 8, 12, 17 and 25.
