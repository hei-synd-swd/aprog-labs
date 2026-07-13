---
id          = "trait_impl"
name        = "Implement a Trait"
language    = "rust"
difficulty  = 2
description = "Implement the same trait on multiple different types."
topics      = ["trait", "impl", "polymorphism"]
---

# Implement a Trait

The `Describe` trait is provided, along with two structs `Rectangle` and
`Circle`.

Implement `Describe` for both types:

- **`Rectangle`**: `describe()` returns `"Rectangle: {w} x {h}"`
- **`Circle`**: `describe()` returns `"Circle: radius {r}"`

## Expected Result

Both `rect.describe()` and `circle.describe()` work and return the correct
format.
