---
id          = "trait_default"
name        = "Default Methods"
language    = "rust"
difficulty  = 3
description = "Add a default method to a trait and override it on one type."
topics      = ["trait", "default", "method"]
---

# Default Methods

The `Describe` trait is provided, along with `Rectangle` and `Circle`.

1. **Add** a method `fn introduction(&self) -> String` to the `Describe` trait
   with a **default implementation** that returns `"I am a {describe()}"`
   (i.e. it calls `self.describe()` inside the default).

2. **Override** `introduction` for `Circle` to return `"I am a circle with
   radius {radius}"` instead.

`Rectangle` should use the default; `Circle` should override it.

## Expected Result

- `rect.introduction()` → `"I am a Rectangle: 10 x 20"`
- `circle.introduction()` → `"I am a circle with radius 5"`
