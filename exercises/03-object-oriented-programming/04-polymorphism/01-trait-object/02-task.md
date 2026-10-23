---
id          = "trait_object"
name        = "Trait Object"
language    = "rust"
difficulty  = 2
description = "Call methods through dynamic dispatch with &dyn Shape and Box<dyn Shape>."
topics      = ["trait object", "dyn", "Box", "dynamic dispatch"]
---

# Trait Object

The `Shape` trait, `Rectangle`, and `Circle` are provided. Your job is to use them through trait objects.

## Your Task

1. Write `fn describe(shape: &dyn Shape) -> String` that returns the shape's area formatted as `"Area: X.XX"` (two decimals), using the `&dyn Shape` borrow form.
2. Write `fn total_area(shapes: &[Box<dyn Shape>]) -> f64` that sums the area of every shape in the slice, calling `.area()` on each owned `Box<dyn Shape>` through dynamic dispatch.
3. In `main`, put a `Rectangle` and a `Circle` into a `Vec<Box<dyn Shape>>`, print each with `describe`, and print the sum with `total_area`.

## Expected Result

`describe(&Rectangle::new(3, 4))` returns `"Area: 12.00"`, and `total_area` of a `5×8` rectangle plus a unit circle is about `43.14` - the same `.area()` call works on every `Shape` through dynamic dispatch.
