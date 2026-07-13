# Custom Types and Derive Macros

Types you define yourself (enums, structs) don't automatically know how to
be printed, copied, or compared. The `#[derive]` attribute asks the compiler
to generate those capabilities automatically:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum Status {
    Active,
    Inactive,
}
```

- **`Debug`** → `{:?}` formatting — print the value for debugging
- **`Clone`** → `.clone()` — make an explicit copy
- **`Copy`** → Implicit copy on assignment (no move) — only for simple types
- **`PartialEq`** → `==` and `!=` comparisons

## Type aliases

A **type alias** gives a shorter or more meaningful name to an existing type:

```rust
type Age = u8;
```

Now `Age` and `u8` are interchangeable, but the name documents intent.
