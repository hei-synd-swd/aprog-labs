# Padding and Alignment

`format!` and `println!` support alignment specifiers inside `{}`:

```rust
format!("{:8}", 42);    // "      42"  right-aligned (default)
format!("{:<8}", 42);   // "42      "  left-aligned
format!("{:^8}", 42);   // "   42   "  center-aligned
```

The width can be **dynamic** — use `$` to reference a variable or argument:

```rust
let width = 8;
format!("{:^1$}", 42, 8);            // "   42   "  (positional arg, width = 8)
```

Combine alignment with repeats to build structured output:

```rust
format!("{:^1$}", "*".repeat(5), 15); // "   *****    "
```
