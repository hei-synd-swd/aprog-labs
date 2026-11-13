---
id          = "conc_par_iter"
name        = "Parallel Iterator"
language    = "rust"
difficulty  = 2
description = "Turn a sequential iterator chain into a parallel one with Rayon."
topics      = ["rayon", "par_iter", "data parallelism", "map", "sum"]
---

# Parallel Iterator

The Collatz rule turns a number `n` into `n / 2` if it is even and into `3 * n + 1` if it is odd, and repeats this until it reaches 1. The given function `collatz_steps(n)` counts the steps this takes, for example `collatz_steps(3)` is 7.

The given function `total_steps_seq(numbers)` adds up the step counts of all numbers **sequentially**. This is CPU-bound and every number is independent, so it is a good fit for Rayon.

## Your Task

Replace the `todo!()` in `total_steps_par(numbers: &[u64]) -> u64`. Do not change `collatz_steps`, `total_steps_seq`, `main`, the tests or `Cargo.toml`.

1. Read `total_steps_seq`. Your function computes exactly the same value.
2. Write the same iterator chain, but start it with `numbers.par_iter()` instead of `numbers.iter()`.
3. Keep the rest of the chain unchanged: `.map(|&n| collatz_steps(n))` followed by `.sum()`.

The import `use rayon::prelude::*;` is already at the top of the file. It is what makes `par_iter()` available.

```
total_steps_par(&[1, 2, 3]) → 8      (0 + 1 + 7)
total_steps_par(&[27])      → 111
total_steps_par(&[])        → 0
```

## Expected Result

- `total_steps_par` returns the same total as `total_steps_seq` for every input, and `0` for an empty slice.
- `main` prints the total and the time of both versions and the speedup. On a multi-core machine the parallel version is clearly faster.

> The first build downloads Rayon from crates.io, so a working internet connection is required.
