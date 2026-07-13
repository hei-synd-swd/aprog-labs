---
id          = "impp_max"
name        = "Maximum"
language    = "rust"
difficulty  = 1
description = "Find the largest value in a slice."
topics      = ["slice", "iteration"]
---

# Maximum

Write `max(arr: &[i32]) -> Option<i32>` that returns the largest value
in the slice, or `None` if the slice is empty.

Expected Result:
```
max(&[])           → None
max(&[5])          → Some(5)
max(&[3, 7, 2, 9]) → Some(9)
```
