# Shadowing

**Shadowing** reuses a name by introducing a *new binding* with `let`. The new
binding can even have a different type:

```rust
let spaces = "   ";        // &str
let spaces = spaces.len(); // usize — a brand-new binding
```

This is different from `mut`:

| `mut` | shadowing |
|-------|-----------|
| same binding, reassigned | new binding each time |
| type cannot change | type **can** change |

Shadowing is handy when transforming a value through several steps while
keeping one meaningful name:

```rust
let input = "  42  ";       // &str
let input = input.trim();   // &str, trimmed
let input: i32 = input.parse().unwrap(); // i32
```

Each `let` creates a fresh binding that shadows the previous one.
