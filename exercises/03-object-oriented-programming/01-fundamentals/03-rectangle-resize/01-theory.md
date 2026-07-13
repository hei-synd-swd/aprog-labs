# Mutable Methods

When a method needs to change the instance's fields, use **`&mut self`** as the
first parameter. This borrows the instance mutably.

```rust
struct Counter {
    value: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.value += 1;
    }
}
```

## Calling a Mutable Method

The binding must be `mut`:

```rust
let mut c = Counter { value: 0 };
c.increment();                // c.value is now 1
println!("{}", c.value);      // 1
```

## `&self` vs `&mut self`

| Signature | Can read fields | Can write fields |
|---|---|---|
| `&self` | ✅ | ❌ |
| `&mut self` | ✅ | ✅ |

The Rust compiler enforces these rules at compile time — no data races.
