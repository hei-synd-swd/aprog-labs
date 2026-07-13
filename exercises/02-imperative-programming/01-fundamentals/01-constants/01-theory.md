# Constants

A **constant** is a name permanently bound to a *compile-time value*.

```rust
const MAX_SPEED: u32 = 120;
```

Key rules:

- Declared with the `const` keyword (not `let`).
- The **type annotation is mandatory** — the compiler will not infer it.
- The name uses `SCREAMING_SNAKE_CASE`.
- Constants are always immutable and valid for the whole program.

Constants can be built from other constants using a **constant expression** —
any computation the compiler can evaluate ahead of time:

```rust
const WIDTH: u32 = 4;
const HEIGHT: u32 = 3;
const AREA: u32 = WIDTH * HEIGHT; // 12, computed at compile time
```

Unlike `let` bindings, a `const` has no fixed memory address — the compiler
*inlines* its value wherever it is used, so there is zero runtime cost.
