# Named Constructors & Computed Fields

## Multiple Constructors

A type can have several **named constructors** — associated functions that
return `Self`. This gives callers ergonomic ways to create instances:

```rust
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    fn black() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    fn white() -> Self {
        Self { r: 255, g: 255, b: 255 }
    }
}
```
