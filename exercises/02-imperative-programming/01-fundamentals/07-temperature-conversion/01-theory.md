# `From` and `Into`

The `From` trait defines an **infallible conversion** from one type to another.
Implementing `From` automatically gives you `Into` for free.

```rust
struct Meters(f64);
struct Feet(f64);

impl From<Meters> for Feet {
    fn from(m: Meters) -> Self {
        Feet(m.0 * 3.281)
    }
}
```

You can then convert in two equivalent ways:

```rust
let f = Feet::from(Meters(10.0)); // using From
let f: Feet = Meters(10.0).into(); // using Into (auto-derived)
```

Note: `Meters(10.0)` is a **tuple struct**; its single field is accessed with
`.0`.

Prefer `From`/`Into` for conversions that can never fail. Use `TryFrom` when a
conversion might fail.
