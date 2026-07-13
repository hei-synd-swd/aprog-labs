---
id          = "shape_trait"
name        = "Shape Trait"
language    = "rust"
difficulty  = 3
description = "Define a Shape trait with area, implement for two types, write a polymorphic function."
topics      = ["trait", "polymorphism", "shape", "area", "impl Trait"]
---

# Shape Trait

You are given `Rectangle` and `Circle` structs (with constructors).

1. **Define** a trait `Shape` with one method:
   - `fn area(&self) -> f64`
2. **Implement `Shape`** for both `Rectangle` and `Circle`.
3. **Write a function** `fn compare_areas(a: &impl Shape, b: &impl Shape) -> String`
   that returns which shape has a larger area: `"First is larger"`,
   `"Second is larger"`, or `"Equal areas"`.

## Expected Result

`compare_areas(&rect, &circle)` compares areas of different shapes using a
single function.
