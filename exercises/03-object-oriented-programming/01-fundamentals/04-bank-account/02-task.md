---
id          = "oop_circle"
name        = "Circle Struct"
language    = "rust"
difficulty  = 2
description = "Practice struct patterns by building a Circle type with &self and &mut self methods."
topics      = ["struct", "methods", "&self", "&mut self"]
---

# Circle Struct

Define a `Circle` struct with a field `radius: f64`. Then implement the
following methods:

1. **`area(&self) -> f64`** — returns `π × r²` (use `std::f64::consts::PI`).
2. **`circumference(&self) -> f64`** — returns `2 × π × r`.
3. **`grow(&mut self, factor: f64)`** — multiplies the radius by `factor`.

Create instances directly: `Circle { radius: 5.0 }`.

## Expected Result

`area` and `circumference` return correct values; `grow` modifies the radius.
