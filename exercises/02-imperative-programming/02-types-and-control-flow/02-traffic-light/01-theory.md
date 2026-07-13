# Enums

An **enum** (enumeration) lists possible variants of a type:

```rust
enum Direction {
    North,
    East,
    South,
    West,
}
```

Each variant is a value of the enum type: `Direction::North`.

## match

Use `match` to handle each variant:

```rust
match dir {
    Direction::North => println!("going up"),
    Direction::East  => println!("going right"),
    Direction::South => println!("going down"),
    Direction::West  => println!("going left"),
}
```

The compiler checks that every variant is covered — unmatched variants
cause a compilation error. This is called **exhaustiveness**.

## matches!

The `matches!` macro checks if a value matches a pattern and returns a `bool`:

```rust
matches!(dir, Direction::North)  // true if dir is North
```

This is shorter than writing a full `match` when you only need a `true`/`false`
answer.
