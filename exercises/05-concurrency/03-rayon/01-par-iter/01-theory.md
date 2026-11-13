# Parallel Iterators with Rayon

Rayon is a crate for **data parallelism**: the same operation applied to many independent items. It keeps a **thread pool** with about one thread per core and splits the work over it for you.

To use it, replace `iter()` with `par_iter()`. The adaptors you already know (`map`, `filter`, `sum`, `count`, `min`, `max`, ...) stay the same.

```rust
use rayon::prelude::*;

fn main() {
    let words = vec!["concurrency", "is", "not", "parallelism"];

    // sequential: one item after the other, on one core
    let seq: usize = words.iter().map(|w| w.len()).sum();

    // parallel: Rayon splits the vector and uses all cores
    let par: usize = words.par_iter().map(|w| w.len()).sum();

    assert_eq!(seq, par);
}
```

## How it works

- `use rayon::prelude::*;` adds the `par_iter()` method to vectors and slices.
- Rayon cuts the data into pieces, runs the closure on several threads and combines the partial results.
- The items are processed in no particular order, but the result is the same as the sequential one.

## When it pays off

- The work per item is **CPU-bound** and the items are **independent** of each other.
- The closure must not mutate shared state. The compiler checks this, so a `par_iter` that compiles has no data races.
- Splitting and combining costs time. For tiny workloads the sequential version is faster.

Rayon is an external crate. It is listed under `[dependencies]` in `Cargo.toml`.
