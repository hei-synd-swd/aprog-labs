---
id          = "inheritance_composed_shape"
name        = "Composed Shape"
language    = "rust"
difficulty  = 4
description = "Combine composition with trait objects to build a Drawing that holds mixed shapes."
topics      = ["composition", "trait object", "dyn", "polymorphism", "delegation"]
---

# Composed Shape

A `Shape` trait, `Circle`, and `Rectangle` are provided, each with `area()`
and `name()`. A `Drawing` struct is partially implemented — it has a vector
of `Box<dyn Shape>` and a label.

1. **Implement `add_shape(&mut self, shape: Box<dyn Shape>)`** — push the
   shape into the drawing's shape vector.

2. **Implement `total_area(&self) -> f64`** — return the sum of all shape
   areas.

3. **Implement `describe(&self) -> String`** — return a formatted string:
   `"Drawing '{label}' has {n} shapes, total area: {total:.2}"`.

## Expected Result

- An empty drawing has a total area of 0.0.
- After adding shapes, `total_area` returns the correct sum.
- `describe` includes the label, shape count, and total area.
