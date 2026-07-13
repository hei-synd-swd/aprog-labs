# Variables and Mutability

In Rust, bindings created with `let` are **immutable by default**:

```rust
let x = 5;
x = 6; // ERROR: cannot assign twice to immutable variable `x`
```

To allow reassignment, add `mut`:

```rust
let mut count = 0;
count += 1; // OK
count += 1; // count is now 2
```

A common use of `mut` is an **accumulator** that is updated inside a loop:

```rust
let mut total = 0;
for n in [1, 2, 3] {
    total += n;
}
// total == 6
```

Start immutable; add `mut` only when you actually need to change the value.
This makes intent explicit and lets the compiler catch accidental mutations.
