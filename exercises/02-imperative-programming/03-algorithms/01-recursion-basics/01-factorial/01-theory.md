# Recursion

A function that calls itself is called **recursive**. Recursion lets you
break a problem into smaller instances of the same problem.

Every recursive function needs two parts:

1. **Base case** — a simple input that can be solved directly without
   recursing (e.g. `n == 0`).
2. **Recursive case** — a step that reduces the problem toward the base
   case (e.g. `n * factorial(n - 1)`).

## Example: Countdown

```rust
fn countdown(n: u32) {
    if n == 0 {
        println!("Go!");
    } else {
        println!("{n}");
        countdown(n - 1);
    }
}
```

When `countdown(3)` is called, it prints `3` then calls `countdown(2)`,
which prints `2` then calls `countdown(1)`, and so on until the base case
`countdown(0)` prints `Go!`.
