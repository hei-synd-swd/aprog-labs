---
id          = "oop_rectangle_resize"
name        = "Rectangle Resize"
language    = "rust"
difficulty  = 2
description = "Add a mutable method that modifies the rectangle."
topics      = ["methods", "&mut self", "mutable"]
---

# Rectangle Resize

The `Rectangle` struct and its methods (`area`, `perimeter`) are provided. Add a **mutable method** that modifies the instance.

Implement:

1. **`scale(&mut self, factor: u32)`** — multiplies both `width` and `height`
   by `factor`.
2. **`shrink(&mut self, factor: u32)`** — divides both `width` and `height`
   by `factor`. (Integer division is fine — truncation is acceptable.)

## Expected Result

After `rect.scale(2)` the dimensions double; after `rect.shrink(2)` they halve.
