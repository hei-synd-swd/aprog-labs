# Alignment and Dynamic Widths

You can align text left, right, or center with `<`, `>`, and `^`:

```rust
format!("|{:<10}|", "left");    // |left      |
format!("|{:>10}|", "right");   // |     right|
format!("|{:^10}|", "center");  // |  center  |
```

Dynamic width via `$` works with alignment too:

```rust
let w = 10;
format!("|{:>1$}|", 42, w);     // "|        42|"
```

Combine string repetition with alignment for visual elements:

```rust
let filled = 7;
let empty = 3;
format!("[{}{}]", "#".repeat(filled), ".".repeat(empty)); // "[#######...]"
```
