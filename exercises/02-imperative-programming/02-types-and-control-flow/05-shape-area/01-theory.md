# Enums with Data and f64

Enum variants can hold data — each variant can have different fields
with different types:

```rust
enum Shape {
    Circle { radius: f64 },
    Rect { width: f64, height: f64 },
}
```

## Destructuring with match

Use `match` to extract the data:

```rust
match shape {
    Shape::Circle { radius } => /* use radius */,
    Shape::Rect { width, height } => /* use width, height */,
}
```

## f64 constants (no math library needed)

Rust's standard library provides math constants in `std::f64::consts`:

```rust
std::f64::consts::PI       // 3.141592653589793
std::f64::consts::E        // 2.718281828459045
```

Import them with `use std::f64::consts::PI;` then use as `PI`.
