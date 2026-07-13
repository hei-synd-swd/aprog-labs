# Arrays

An **array** is a fixed-length list of values of the same type:

```rust
let a: [u8; 3] = [10, 20, 30];
```

The type `[T; N]` means "N elements of type T". The length is part of the
type and cannot change.

## 2D arrays

An array of arrays creates a grid:

```rust
let board: [[u8; 3]; 3] = [[0; 3]; 3];
//       columns ^            rows ^   
```

`[[0; 3]; 3]` creates a 3×3 grid filled with zeros.

Access elements with double bracket notation:

```rust
board[row][col] = 5;       // write
let value = board[r][c];   // read
```

## Iterating over an array

Use `for` with a range to loop over indices:

```rust
for row in 0..3 {
    for col in 0..3 {
        // do something with board[row][col]
    }
}
```

Arrays are fixed-size and stack-allocated — no heap allocation needed.
