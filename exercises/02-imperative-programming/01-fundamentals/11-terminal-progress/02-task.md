---
id          = "impp_progress"
name        = "Terminal Progress Bar"
language    = "rust"
difficulty  = 2
description = "Build a terminal-style progress bar using format alignment."
topics      = ["formatting", "padding", "alignment"]
---

# Terminal Progress Bar

Write `progress_bar(current: u32, total: u32, width: u32) -> String` that
returns a progress bar like `"[###.......]  30%"`.

- `width` is the number of positions **inside** the brackets.
- Filled positions: `current × width / total`
- Empty positions: `width − filled`
- Percentage: `current × 100 / total`, right-aligned to 3 characters.

Expected Result:
```
progress_bar(3, 10, 10) → "[###.......]  30%"
progress_bar(0, 10, 10) → "[..........]   0%"
progress_bar(10, 10, 10) → "[##########] 100%"
```
