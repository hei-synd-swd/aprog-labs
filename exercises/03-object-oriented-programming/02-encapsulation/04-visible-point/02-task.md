---
id          = "encapsulation_point"
name        = "Visible Point"
language    = "rust"
difficulty  = 3
description = "Add a named constructor, a computation method, and a mutable method to an encapsulated Point."
topics      = ["encapsulation", "constructor", "method", "getter", "mutable"]
---

# Visible Point

A `Point` struct with **private** fields `x: f64` and `y: f64` is provided.
Getters `x()` and `y()` are already implemented; `new(x, y)` is the primary
constructor.

1. **Add a named constructor** `pub fn origin() -> Self` that returns a
   `Point` at `(0.0, 0.0)`.

2. **Add a computation method**
   `pub fn distance(&self, other: &Point) -> f64` that returns the Euclidean
   distance: √((x₂ − x₁)² + (y₂ − y₁)²).

3. **Add a mutable method**
   `pub fn translate(&mut self, dx: f64, dy: f64)` that adds `dx` to `x` and
   `dy` to `y`.

## Expected Result

- `Point::origin()` creates a point at (0, 0).
- `distance` correctly computes the distance between two points.
- `translate` shifts the point by the given offsets.
- The fields `x` and `y` remain **private** — access is only through getters.
