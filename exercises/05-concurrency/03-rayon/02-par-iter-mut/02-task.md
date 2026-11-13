---
id          = "conc_par_iter_mut"
name        = "Mutate and Reduce in Parallel"
language    = "rust"
difficulty  = 3
description = "Update a slice in place with par_iter_mut and find its maximum with a parallel reduce."
topics      = ["rayon", "par_iter_mut", "for_each", "reduce", "identity"]
---

# Mutate and Reduce in Parallel

A weather station stores a million temperature readings in °C. All sensors are off by the same amount, so every reading must be corrected. Afterwards the station wants to know the highest temperature.

## Your Task

Replace the two `todo!()`. Do not change the signatures, `main`, the tests or `Cargo.toml`. The import `use rayon::prelude::*;` is already there.

**`calibrate(readings: &mut [f64], offset: f64)`** adds `offset` to every reading, in place.

1. Start with `readings.par_iter_mut()`.
2. Call `.for_each(...)` with a closure that takes one `reading` and adds the offset to the value behind it: `*reading += offset`.

**`hottest(readings: &[f64]) -> f64`** returns the largest reading.

1. Start with `readings.par_iter()` and add `.copied()` to get `f64` values instead of references.
2. Call `.reduce(identity, op)`:
   - `identity` is the closure `|| f64::NEG_INFINITY`. Do not use `0.0`: it would be wrong for readings below zero.
   - `op` is `f64::max`.
3. Return the result of `reduce`.

```
calibrate(&mut [20.0, 21.5, 19.25], 0.5) → the slice becomes [20.5, 22.0, 19.75]

hottest(&[20.5, 22.0, 19.75]) → 22.0
hottest(&[-3.0, -1.5, -7.0])  → -1.5
hottest(&[])                  → f64::NEG_INFINITY
```

## Expected Result

- `calibrate` changes the slice in place: every reading is larger by `offset`, the length and the order stay the same.
- `hottest` returns the maximum, also when all readings are negative, and negative infinity for an empty slice.
- `main` calibrates a million readings by 0.5 °C, then prints the first four readings `[0.5, 0.6, 0.7, 0.8]` and the hottest one, `40.4`.

> The first build downloads Rayon from crates.io, so a working internet connection is required.
