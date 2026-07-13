---
id          = "shape_collection"
name        = "Shape Collection"
language    = "rust"
difficulty  = 4
description = "Store mixed shapes in a Vec<Box<dyn Shape>> and compute total area."
topics      = ["collection", "Vec", "trait object", "iteration", "Box"]
---

# Shape Collection

The `Shape` trait, `Rectangle`, and `Circle` are provided.

1. **Create** a `Vec<Box<dyn Shape>>` containing at least one `Rectangle`
   and one `Circle`.
2. Write a function **`total_area(shapes: &[Box<dyn Shape>]) -> f64`** that
   sums the areas of all shapes in the slice.

## Expected Result

`total_area` computes the sum regardless of which shapes are in the list.
