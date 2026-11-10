---
id          = "debug_off_by_one"
name        = "Off by One"
language    = "rust"
difficulty  = 2
description = "Fix three pagination helpers that compile cleanly but fail their tests: rounding, clamping and index arithmetic."
topics      = ["debugging", "off-by-one", "integer division", "panics", "clamping"]
---

# Off by One

A blog engine's pagination module compiles, but fails its tests. All three functions in `main.rs` contain integer-related bugs.

1. Go to the output tab. You'll see **2 pages** for 12 items at 5 per page. 12 items should actually need 3 pages though.
2. Fix `total_pages` to **round up** with the help of `usize::div_ceil`.
3. Run the demo again: now it **panics** while slicing the last page. Fix `page_bounds` so that the returned `(start, end)` is always in following form: `start <= end <= total` (clamp both with `.min(..)`).
4. Fix `last_page_index` so it actually returns the **index** of the last page and doesn't underflow when there are no pages (`saturating_sub`). 

## Specification

```text
total_pages(total, per_page)       -> ceil(total / per_page), 0 for total = 0
page_bounds(page, per_page, total) -> (min(page * per_page, total),
                                     min(start + per_page, total))
last_page_index(total, per_page)   -> total_pages(total, per_page) - 1,
                                     clamped at 0
```

`per_page` is always >= 1, there's no need to guard against 0.

## Expected Result

All five test functions pass and the demo run in `main` prints every page without panicking.
