# Memoization

Memoization stores the results of expensive function calls so they can
be reused instead of recomputed.

For Fibonacci, the naive recursion recomputes F(2) many times.
With memoization, each value is computed once and then cached.

```rust
let mut memo: Vec<Option<u64>> = vec![None; size];
memo[0] = Some(0);
memo[1] = Some(1);
// Before computing, check if memo[n] is Some
// After computing, store in memo[n]
```

This reduces time from O(2ⁿ) to O(n).
