---
id          = "impp_reverse"
name        = "Reverse"
language    = "rust"
difficulty  = 2
description = "Reverse a slice in place."
topics      = ["slice", "iteration"]
---

# Reverse

Write `reverse(arr: &mut [i32])` that reverses the slice **in place**.

Swap the first and last elements, then the second and second-last, and
so on. Use `arr.swap(i, j)` to swap two positions.

Expected Result:
```
let mut v = vec![1, 2, 3, 4];
reverse(&mut v);
// v == [4, 3, 2, 1]
```
