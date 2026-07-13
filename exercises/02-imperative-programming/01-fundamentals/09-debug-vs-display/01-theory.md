# Debug vs Display

Rust has two main formatting traits:

- **`Display`** — for user-facing output, used with `{}`
- **`Debug`** — for developer-facing output, used with `{:?}`

Most built-in types implement both, but custom types only get `Debug`
when you derive it:

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 3, y: 4 };
println!("{p:?}"); // Point { x: 3, y: 4 }
```

To use `{}` on a custom type, you must implement `Display` manually:

```rust
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

println!("{p}"); // (3, 4)
```

| Trait | Format | Auto-derivable? | Typical output |
|-------|--------|----------------|----------------|
| `Display` | `{}` | No | User-friendly, no struct field names |
| `Debug` | `{:?}` | Yes (`#[derive(Debug)]`) | Struct field dump with names |
