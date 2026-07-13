# Integer Literals

The same integer value can be written in several **numeric bases**:

```rust
let decimal = 98_222;      // base 10
let hex     = 0xFF;        // base 16, prefix 0x  → 255
let octal   = 0o77;        // base 8,  prefix 0o  → 63
let binary  = 0b1111_0000; // base 2,  prefix 0b  → 240
let byte    = b'A';        // ASCII byte literal  → 65
```

The underscore `_` is a **digit separator** — it is ignored by the compiler
and only improves readability:

```rust
let million = 1_000_000;
let mask    = 0b0000_1111;
```

All of these are just different *notations* for the same numbers. For example
`0xFF`, `0b1111_1111`, `0o377`, and `255` all represent the value 255.
