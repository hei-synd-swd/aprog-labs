---
id          = "trait_definition"
name        = "Trait Definition"
language    = "rust"
difficulty  = 2
description = "Define a trait and implement it on a struct."
topics      = ["trait", "impl", "method"]
---

# Trait Definition

The `Rectangle` struct is provided.

1. **Define** a trait `Describe` with one method:
   - `fn describe(&self) -> String`

2. **Implement** `Describe` for `Rectangle` so that `describe()` returns
   a string like `"Rectangle: 5 x 8"`.

## Expected Result

`rect.describe()` returns a description of the rectangle's dimensions.
