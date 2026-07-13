---
id          = "impp_tree"
name        = "Tree Creator"
language    = "rust"
difficulty  = 2
description = "Build an ASCII Christmas tree using format padding."
topics      = ["formatting", "padding", "loops"]
---

# Tree Creator

Write `build_tree(height: u32) -> String` that returns an ASCII Christmas
tree. Each row `i` (0-indexed) has `2×i + 1` stars, **center-aligned** to a
width of `2×height − 1`.

Expected Result:
```
build_tree(3) →
  *
 ***
*****
```
