# Display & Debug

Rust provides two built-in traits for converting values to strings.

## `Debug` — Developer Output (`{:?}`)

Usually **derived** — the compiler generates a field-by-field representation:

```rust
#[derive(Debug)]
struct Point { x: i32, y: i32 }

println!("{:?}", Point { x: 3, y: 4 }); // Point { x: 3, y: 4 }
```

## `Display` — User Output (`{}`)

Must be **implemented manually** — only you know what "pretty" looks like:

```rust
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

println!("{}", Point { x: 3, y: 4 }); // (3, 4)
```
