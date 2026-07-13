---
id          = "impp_linear_search"
name        = "Linear Search"
language    = "rust"
difficulty  = 1
description = "Find the index of a value in a slice."
topics      = ["slice", "iteration"]
---

# Linear Search

Write `linear_search(arr: &[i32], target: i32) -> Option<usize>` that
returns the **index** of the first occurrence of `target`, or `None`
if not found.

Expected Result:
```
linear_search(&[5, 3, 8, 1], 8) → Some(2)
linear_search(&[5, 3, 8, 1], 9) → None
```
