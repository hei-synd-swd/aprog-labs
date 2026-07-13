# Offset-based Neighbor Iteration

Many grid algorithms need to inspect the cells adjacent to a given position.
A clean way to do this is to define the **relative offsets** of the neighbors,
then loop over them and **add bounds checks**.

## The offset table

For the 8 cardinal and diagonal directions:

```rust
let offsets = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];
```

Each `(dr, dc)` pair is the row/col delta from the current cell.

## Applying offsets with bounds

Convert to `isize` for the offset arithmetic, then verify the result is
inside the grid before accessing it:

```rust
let rows = grid.len();
let cols = grid[0].len();

for (dr, dc) in offsets {
    let r = row as isize + dr;
    let c = col as isize + dc;
    if r >= 0 && r < rows as isize && c >= 0 && c < cols as isize {
        // grid[r as usize][c as usize] is a valid neighbor
    }
}
```

This pattern works for any grid-based algorithm — Conway's Game of Life,
pathfinding, image filters, maze solving — without hard-coding each
neighbor position.
