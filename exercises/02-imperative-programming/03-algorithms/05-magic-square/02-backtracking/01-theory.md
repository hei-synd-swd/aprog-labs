# Backtracking

Backtracking explores all candidates and abandons dead ends:

```rust
fn explore(candidates: ...) {
    if all_placed() {
        // check solution and return
    }
    for option in available_options {
        // 1. place
        // 2. recurse
        // 3. un-place (backtrack)
    }
}
```

For the magic square, candidates are empty cells and options are numbers
1..=9 not yet placed.
