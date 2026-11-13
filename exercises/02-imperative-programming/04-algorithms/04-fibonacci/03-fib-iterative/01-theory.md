# Fibonacci - Iterative

The Fibonacci sequence starts `0, 1, 1, 2, 3, 5, 8, 13, ...`, where each number is the **sum of the two before it**: `F(n) = F(n-1) + F(n-2)`.

Instead of recursing, you can build it **bottom-up**: keep only the last two values in a rolling pair and step forward `n` times.

```rust
let (mut a, mut b) = (0, 1);
// each step: a becomes b, b becomes a + b
(a, b) = (b, a + b);
```

This runs in O(n) time using O(1) memory - no recursion and no cache, unlike the recursive and memoized versions.
