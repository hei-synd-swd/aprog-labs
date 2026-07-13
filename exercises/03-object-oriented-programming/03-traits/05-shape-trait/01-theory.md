# Traits for Computation

Traits shine when used for **computation** — a function that works with any
type implementing a trait can compute different results for each type.

```rust
struct Square { side: f64 }
struct Circle { radius: f64 }

trait Area {
    fn area(&self) -> f64;
}

impl Area for Square {
    fn area(&self) -> f64 { self.side * self.side }
}

impl Area for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
}
```

A function can accept **any type that implements the trait**:

```rust
fn print_area(shape: &impl Area) {
    println!("Area: {:.2}", shape.area());
}
```

`print_area` works for both `Square` and `Circle` — each computes its own
area through the same function. Adding a new type later (e.g. `Triangle`)
doesn't require changing the function, only implementing `Area` for it.
