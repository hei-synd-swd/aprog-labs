---
id          = "oop_rectangle_area"
name        = "Rectangle Area"
language    = "rust"
difficulty  = 1
description = "Add area and perimeter methods to Rectangle."
topics      = ["methods", "&self", "impl"]
---

# Rectangle Area

The `Rectangle` struct is provided. Add an `impl` block with two **methods**
that use `&self` to read the fields:

1. **`area(&self) -> u32`** — returns `width × height`.
2. **`perimeter(&self) -> u32`** — returns `2 × (width + height)`.

Create instances directly with struct literal syntax, e.g.:
`Rectangle { width: 10, height: 20 }`.

## Expected Result

`rect.area()` returns the area and `rect.perimeter()` returns the perimeter.
