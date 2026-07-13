---
id          = "impp_shape_area"
name        = "Shape Area Calculator"
language    = "rust"
difficulty  = 3
description = "Use enums with data and match to calculate shape areas."
topics      = ["enum", "match", "destructuring", "f64"]
---

# Shape Area Calculator

Model geometric shapes as an enum with data and calculate their area.

1. Define an enum `Shape` with three variants:
   - `Circle { radius: f64 }`
   - `Rect { width: f64, height: f64 }`
   - `Triangle { base: f64, height: f64 }`
2. Write a function `area(shape: &Shape) -> f64` using `match` and pattern
   destructuring.
3. Use `std::f64::consts::PI` for the circle area.

## Expected Result

```
area(&Shape::Circle { radius: 1.0 })    → 3.141592653589793
area(&Shape::Rect { width: 3.0, height: 4.0 }) → 12.0
area(&Shape::Triangle { base: 4.0, height: 3.0 }) → 6.0
```
