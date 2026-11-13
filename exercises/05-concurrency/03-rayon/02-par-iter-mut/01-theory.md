# Mutating and Reducing in Parallel

Rayon can also **change** items in place and **combine** all items into one value.

```rust
use rayon::prelude::*;

fn main() {
    let mut prices = vec![10, 20, 30, 40];

    // change every item in place
    prices.par_iter_mut().for_each(|price| *price *= 2);

    // combine all items into one value
    let total = prices.par_iter().copied().reduce(|| 0, |a, b| a + b);

    println!("{prices:?} -> {total}"); // [20, 40, 60, 80] -> 200
}
```

## `par_iter_mut` and `for_each`

- `par_iter_mut()` yields a `&mut` reference to every item. `for_each` runs a closure on each of them.
- Inside the closure, `*item` reads or writes the value behind the reference.
- Every thread gets **different** items, so no two threads ever write to the same one. That is why this compiles without a lock.

## `reduce`

`reduce(identity, op)` combines the items pairwise, in any grouping and on any thread:

- `op` takes two values and returns one. It must give the same result for any grouping, like `+`, `min` or `max`.
- `identity` is a **closure** that returns the neutral value of `op`: the value that changes nothing. It is `0` for `+` and negative infinity for `max`. Each piece of the work starts from it, and it is the result for an empty input.
- `copied()` turns the `&f64` items of `par_iter()` into plain `f64` values, so that `op` works on values.

A function with a matching signature can be passed directly as `op`: `f64::max` takes two `f64` and returns the larger one.
