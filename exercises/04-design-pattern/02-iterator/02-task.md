---
id          = "iterator_trait"
name        = "Iterator Trait"
language    = "rust"
difficulty  = 3
description = "Implement Iterator for your own type and use the standard adaptors on it."
topics      = ["traits", "iterator", "next", "option", "filter", "collect"]
---

# Iterator Trait

`Squares` yields the square numbers 1, 4, 9, ... up to `max * max`. The struct and its constructor `Squares::up_to(max)` are provided: `n` is the next number to square and starts at 1.

## Your Task

1. Implement the trait `Iterator` for `Squares`:
   - set `type Item = u32;`
   - write `fn next(&mut self) -> Option<u32>`: if `self.n` is greater than `self.max`, return `None`. Otherwise compute the square `self.n * self.n`, add 1 to `self.n`, and return the square in `Some(...)`.
2. Replace the `todo!()` in `squares_above(max: u32, min: u32) -> Vec<u32>`: take `Squares::up_to(max)`, keep the squares greater than `min` with `.filter(|&square| square > min)`, and `.collect()` them.

```
Squares::up_to(4)     yields 1, 4, 9, 16
Squares::up_to(0)     yields nothing
squares_above(6, 10)  → [16, 25, 36]
```

## Expected Result

- `Squares::up_to(max)` yields the squares of `1..=max` in ascending order, then `None`.
- It works in a `for` loop and with `sum`, `map`, `take`, `filter` and `collect`, without any extra code.
- `main` prints the five squares 1, 4, 9, 16, 25, then `sum = 14` and `[16, 25, 36]`.
