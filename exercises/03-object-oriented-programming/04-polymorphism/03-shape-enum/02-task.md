---
id          = "shape_enum"
name        = "Shape Enum"
language    = "rust"
difficulty  = 4
description = "Define an enum wrapping different shapes and implement Shape on it."
topics      = ["enum", "static dispatch", "match", "polymorphism"]
---

# Shape Enum

The `Shape` trait, `Rectangle`, and `Circle` are provided.

1. **Define** an enum `ShapeEnum` with two variants:
   - `Rectangle(Rectangle)`
   - `Circle(Circle)`
2. **Implement `Shape`** for `ShapeEnum` using `match`.
3. **Write** a function `fn total_area_list(shapes: &[ShapeEnum]) -> f64` that
   sums the areas (similar to the previous exercise, but with a `Vec<ShapeEnum>`
   instead of `Vec<Box<dyn Shape>>`).

## Expected Result

Same functionality as the trait object version, but using static dispatch.
