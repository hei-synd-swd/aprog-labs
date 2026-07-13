---
id          = "trait_object"
name        = "Trait Object"
language    = "rust"
difficulty  = 4
description = "Use Box<dyn Shape> to call methods through dynamic dispatch."
topics      = ["trait object", "dyn", "Box", "dynamic dispatch"]
---

# Trait Object

The `Shape` trait, `Rectangle`, and `Circle` are provided.

1. **Create** a `Box<dyn Shape>` from a `Rectangle` and another from a
   `Circle`.
2. **Call** `.area()` on each trait object and print the result.
3. **Write** a function `fn print_area_dyn(shape: &dyn Shape)` that takes a
   trait object reference and prints the area.

## Expected Result

Same `.area()` call works on both shapes through dynamic dispatch.
