# The Constructor Pattern

Rust does not have a built-in `new` keyword. Instead, the convention is to
write an **associated function** called `new` that builds and returns a new
instance.

## Associated Functions

An **associated function** is a function inside an `impl` block that does *not*
take `self` as a parameter. It is called on the **type itself**, not on an
instance.

```rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
}

fn main() {
    let p = Point::origin();
    println!("({}, {})", p.x, p.y);
}
```

## Key Points

- `new` is just a convention — any name works (`from`, `with_defaults`, ...).
- The `::` syntax calls associated functions (like `Point::origin`).
- Associated functions return `Self` (an alias for the type).
- A constructor is a natural place to add validation or defaults later.
