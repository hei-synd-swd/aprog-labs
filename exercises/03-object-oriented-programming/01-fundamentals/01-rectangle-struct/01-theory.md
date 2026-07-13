# Structs — Custom Data Types

A **struct** (short for *structure*) is Rust's primary way to define a custom
data type by grouping related values together.

## Defining a Struct

Use the `struct` keyword, a name in `PascalCase`, and named fields:

```rust
struct Point {
    x: i32,
    y: i32,
}
```

## Creating an Instance

Use **struct literal syntax** — specify the struct name and provide values for
every field:

```rust
let p = Point {
    x: 10,
    y: 20,
};
```

## Accessing Fields

Use dot notation to read or mutate fields:

```rust
println!("x: {}", p.x);
```

A mutable binding lets you change fields:

```rust
let mut p = Point { x: 10, y: 20 };
p.x = 15;
```
