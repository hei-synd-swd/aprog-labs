# 2D Arrays with `Vec`

A 2D grid stored as `Vec<Vec<T>>` has a row count and a column count:

```rust
let grid: Vec<Vec<u32>> = vec![
    vec![1, 2, 3],
    vec![4, 5, 6],
    vec![7, 8, 9],
];

let rows = grid.len();       // 3
let cols = grid[0].len();    // 3 (assumes at least one row)
```

## Iterating over all cells

A nested `for` loop visits every cell:

```rust
for r in 0..rows {
    for c in 0..cols {
        // grid[r][c] is the cell at row r, column c
    }
}
```
