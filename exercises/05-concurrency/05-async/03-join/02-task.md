---
id          = "conc_async_join"
name        = "Join Futures"
language    = "rust"
difficulty  = 3
description = "Await two futures concurrently with tokio::join! instead of one after the other."
topics      = ["async", "await", "tokio", "join", "concurrency"]
---

# Join Futures

A weather station has two slow sensors. The given function `read_sensor(id, delay_ms)` simulates one of them: it waits `delay_ms` milliseconds and then returns `id * 10`.

The given function `read_sequential(delay_a, delay_b)` reads sensor 1, **then** sensor 2, and returns the sum of both values. Its waiting times add up.

## Your Task

Replace the `todo!()` in `read_concurrent(delay_a: u64, delay_b: u64) -> u32`. Do not change `read_sensor`, `read_sequential`, `main`, the tests or `Cargo.toml`.

1. Create the two futures **without** awaiting them: `read_sensor(1, delay_a)` and `read_sensor(2, delay_b)`.
2. Pass both futures to `tokio::join!(...)`. It runs them concurrently and returns a tuple with both results.
3. Destructure that tuple: `let (a, b) = tokio::join!(...);`. Do not write `.await` after `join!` or on its arguments.
4. Return `a + b`.

```
read_sequential(300, 200).await → 30 after about 500 ms
read_concurrent(300, 200).await → 30 after about 300 ms
read_concurrent(100, 400).await → 30 after about 400 ms
```

## Expected Result

- `read_concurrent` returns `30` (`1 * 10 + 2 * 10`) for any delays.
- It takes as long as the **slower** sensor, not the sum of both delays.
- `main` prints both results with their elapsed time: about 500 ms sequential, about 300 ms concurrent.

> The first build downloads Tokio from crates.io, so a working internet connection is required.
