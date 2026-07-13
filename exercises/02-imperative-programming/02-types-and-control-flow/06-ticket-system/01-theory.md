# Ownership and Move Semantics

Each value in Rust has exactly one **owner**. When a variable is assigned
to another or passed to a function, ownership is **moved**:

```rust
let a = String::from("hello");
let b = a;          // a is MOVED to b — a can no longer be used
```

The same happens when passing to a function:

```rust
fn take(s: String) { /* s owns the data now */ }

let s = String::from("hi");
take(s);            // ownership moved into take
// println!("{s}"); // compile error — s was moved
```

## Borrowing

Instead of transferring ownership, you can **borrow** with `&`:

```rust
fn peek(s: &String) -> usize {
    s.len()   // just reads, doesn't take ownership
}
```

- `&T` — immutable reference (read-only, many can coexist)
- `&mut T` — mutable reference (exclusive write access)

## Clone

If you need a copy, use `.clone()`:

```rust
let b = a.clone();  // deep copy — both a and b stay valid
```
