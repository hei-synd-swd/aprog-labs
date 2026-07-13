# Putting It All Together

Now that you know how to define a struct, add read-only methods (`&self`),
and add mutable methods (`&mut self`), you can build more interesting types.

## Struct with Multiple Methods

A struct can have many methods in the same `impl` block — mix read-only
methods and mutable methods:

```rust
impl Rectangle {
    // Read-only method
    fn area(&self) -> u32 { self.width * self.height }

    // Mutable method
    fn scale(&mut self, factor: u32) { self.width *= factor; }
}
```

## Key Points

- Methods with `&self` read the fields without modifying them.
- Methods with `&mut self` can modify the fields.
- You create instances directly with struct literal syntax:
  ```rust
  let rect = Rectangle { width: 10, height: 20 };
  ```
