---
id          = "oop_rectangle_new"
name        = "Rectangle Constructor"
language    = "rust"
difficulty  = 1
description = "Add a new constructor to the Rectangle struct."
topics      = ["constructor", "associated function", "impl", "new"]
---

# Rectangle Constructor

The `Rectangle` struct is provided. Add an `impl` block with a **constructor**
function `new` that takes `width` and `height` and returns a new `Rectangle`.

```rust
fn new(width: u32, height: u32) -> Rectangle
```

## Expected Result

`Rectangle::new(10, 20)` creates a rectangle with width 10 and height 20.
