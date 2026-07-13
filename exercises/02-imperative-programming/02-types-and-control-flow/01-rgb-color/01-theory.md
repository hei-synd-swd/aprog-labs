# Structs

A **struct** bundles related values into one type:

```rust
struct Point {
    x: i32,
    y: i32,
}
```

Fields are accessed with dot notation: `point.x`.

## Constructing a struct

Create an instance by naming each field:

```rust
let p = Point { x: 10, y: 20 };
```

## Formatting values

Use `format!` to produce a string. Format specifiers control the output:

```rust
format!("{:02X}", 255)     // "FF" — uppercase hex, padded to 2 digits
format!("{:08b}", 5)       // "00000101" — binary, padded
format!("{:>10}", "hi")    // "        hi" — right-aligned
```

Combine multiple values into one string:

```rust
format!("({}, {})", p.x, p.y)  // "(10, 20)"
```

## Top-level constants

Constants defined at the top level (outside any function) are available
everywhere in the file:

```rust
const ORIGIN: Point = Point { x: 0, y: 0 };
```
