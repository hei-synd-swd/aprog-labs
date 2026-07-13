---
id          = "impp_magic_square_verify"
name        = "Magic Square Verify"
language    = "rust"
difficulty  = 3
description = "Check if a 3×3 grid is a valid magic square."
topics      = ["array", "iteration", "validation"]
---

# Magic Square Verify

A **magic square** of size `n` is an `n × n` grid where every row, column,
and diagonal sums to the same value (the **magic constant**). For a 3×3
square using numbers 1..=9, the magic constant is 15.

Write `is_magic(grid: &[Vec<u32>]) -> bool` that returns `true` if `grid`
is a valid 3×3 magic square:

- The grid is exactly 3 rows and 3 columns.
- Uses each number from 1 to 9 exactly once.
- Every row, column, and both diagonals sum to 15.

## Expected Result

```
let valid = vec![vec![8, 1, 6], vec![3, 5, 7], vec![4, 9, 2]];
is_magic(&valid) → true

let invalid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
is_magic(&invalid) → false
```
