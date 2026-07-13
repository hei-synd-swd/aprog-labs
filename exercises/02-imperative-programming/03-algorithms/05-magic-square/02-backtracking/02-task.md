---
id          = "impp_magic_backtracking"
name        = "Magic Square Solver"
language    = "rust"
difficulty  = 5
description = "Count all 3×3 magic squares using backtracking."
topics      = ["recursion", "backtracking", "enumeration"]
---

# Magic Square Solver

The function `is_magic` is already provided (from the verify exercise).
Your job is to write the **recursive backtracking** that counts all 3×3
magic squares.

Write two functions:

1. `solve(grid: &mut Vec<Vec<u32>>, used: &mut [bool; 10], pos: usize, count: &mut u32)`
   — `pos` goes from 0 to 8 (the 9 cells). Convert to `(row, col)` with
   `pos / 3` and `pos % 3`. When `pos == 9`, call `is_magic` and
   increment `count` if valid. Otherwise, try each number 1..=9 that
   isn't marked in `used`.

2. `count_magic_squares() -> u32` — creates a 3×3 grid of zeros, a
   `[false; 10]` for used numbers, a counter at 0, calls `solve`, and
   returns the count.

Expected Result:
```
count_magic_squares() → 8
```
