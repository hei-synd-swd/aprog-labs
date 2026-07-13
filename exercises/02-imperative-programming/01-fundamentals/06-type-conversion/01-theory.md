# Type Conversion

## The `as` cast

`as` performs a fast numeric cast, but it can be **lossy**:

```rust
let big: u32 = 300;
let small = big as u8; // 300 wraps to 44 — silent data loss!
```

Widening (`u8` → `u32`) is always safe. Narrowing (`u32` → `u8`) truncates bits
without warning.

## Safe conversion with `TryFrom` / `try_into`

When a value might not fit, use the fallible conversions. They return a
`Result`, so you can detect failure instead of silently corrupting data:

```rust
use std::convert::TryFrom;

let ok  = u8::try_from(200_i32);  // Ok(200)
let bad = u8::try_from(300_i32);  // Err(...)
```

`Result::ok()` turns a `Result<T, E>` into an `Option<T>`, discarding the error:

```rust
let value: Option<u8> = u8::try_from(n).ok();
```
