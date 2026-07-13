---
id          = "impp_hanoi"
name        = "Tower of Hanoi"
language    = "rust"
difficulty  = 3
description = "Solve the Tower of Hanoi puzzle with recursion."
topics      = ["recursion", "hanoi"]
---

# Tower of Hanoi

Solve the classic Tower of Hanoi puzzle recursively.

1. `hanoi(n: u32, from: &str, to: &str, aux: &str)` — prints each move
   in the format `"Move disk from A to C"`. Use string literals like
   `"A"`, `"B"`, `"C"` for the pegs.

2. `hanoi_count(n: u32) -> u32` — returns the number of moves needed
   (which is 2ⁿ − 1). Implement it recursively or with a formula.

## Expected Result

```
hanoi(1, "A", "C", "B") prints:
  Move disk from A to C

hanoi_count(3) → 7
```
