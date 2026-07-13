---
id          = "impp_game_of_life"
name        = "Game of Life"
language    = "rust"
difficulty  = 4
description = "Implement one generation of Conway's Game of Life."
topics      = ["grid", "array", "loops", "boundary"]
---

# Game of Life

Implement one generation of Conway's Game of Life on a 2D grid.

1. `count_neighbors(grid: &Vec<Vec<bool>>, row: usize, col: usize) -> u8`
   — counts live neighbors (8 directions). Cells outside the grid
   are **ignored** (edges/corners have fewer than 8 neighbors).

2. `next_generation(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>>`
   — applies the rules to compute the next generation:
   - Live cell with < 2 neighbors → **dies**
   - Live cell with 2 or 3 neighbors → **survives**
   - Live cell with > 3 neighbors → **dies**
   - Dead cell with exactly 3 neighbors → **becomes alive**

## Expected Result

```
let blinker = vec![
    vec![false, true, false],
    vec![false, true, false],
    vec![false, true, false],
];
let next = next_generation(&blinker);
// blinker oscillates:
//       col 0    col 1    col 2
// row 0: false    false    false
// row 1: true     true     true
// row 2: false    false    false
```
