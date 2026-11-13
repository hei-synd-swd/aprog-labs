# Amdahl's Law

Every program has a **serial part** that cannot be parallelised. Only the **parallel part** gets faster with more processors. Amdahl's Law gives the overall speedup:

```
S = 1 / ((1 - f) + f / p)
```

- `S` is the **speedup**: how many times faster the program runs.
- `f` is the **parallel fraction** of the execution time, between 0 and 1.
- `1 - f` is the **serial fraction**.
- `p` is the number of **processors**.

## Example

90 % of a program can run in parallel (`f = 0.9`) and it gets 4 processors (`p = 4`):

```
S = 1 / ((1 - 0.9) + 0.9 / 4) = 1 / (0.1 + 0.225) = 3.08
```

Four processors make it about three times faster, not four times.

## In Rust

- The formula works on `f64` values. Write constants as `1.0`, not `1`.
- An integer is converted to `f64` without loss using `f64::from(...)`:

```rust
let processors: u32 = 4;
let share = 0.9 / f64::from(processors); // 0.225
```
