# Methods on Structs

Methods are functions attached to a struct via an `impl` block. They give your
types *behaviour* — like methods on a class in OOP.

## The `&self` Parameter

The first parameter of a method is always `self` (or `&self`, `&mut self`).
It represents the instance the method is called on.

- **`&self`** — borrows the instance **immutably** (read-only). Most common.
- **`&mut self`** — borrows **mutably** (can change fields).
- **`self`** — takes **ownership** (consumes the instance).

```rust
struct Counter {
    value: i32,
}

impl Counter {
    fn get(&self) -> i32 {
        self.value
    }
}
```

## Calling a Method

Use **dot notation**:

```rust
let c = Counter { value: 42 };
println!("{}", c.get()); // 42
```

Rust automatically borrows the receiver — `c.get()` is the same as
`(&c).get()`.

## Multiple Methods

You can add as many methods as you need in the same `impl` block:

```rust
impl Counter {
    fn get(&self) -> i32 {
        self.value
    }

    fn increment(&mut self) {
        self.value += 1;
    }
}
```
