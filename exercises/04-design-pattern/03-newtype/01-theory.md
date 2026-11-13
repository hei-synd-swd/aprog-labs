# Newtype Pattern

The **newtype** pattern wraps a single value in a one-field tuple struct. It adds **no runtime cost**, but lets the compiler tell otherwise-identical values apart.

```rust
struct Seconds(f64);
struct Meters(f64);

fn speed(d: Meters, t: Seconds) -> f64 {
    d.0 / t.0 // access the wrapped value with `.0`
}
```

Because `Meters` and `Seconds` are distinct types, you can never pass one where the other is expected - a mix-up a plain `f64` would happily allow.

Key points:

- Declared as a tuple struct: `struct Name(InnerType);`.
- Access the inner value with `.0`.
- Implement `From<A> for B` once, then convert anywhere with `.into()`.
