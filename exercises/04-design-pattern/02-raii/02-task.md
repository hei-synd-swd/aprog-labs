---
id          = "raii"
name        = "RAII Pattern"
language    = "rust"
difficulty  = 2
description = "Tie resource cleanup to an object's lifetime with Drop."
topics      = ["struct", "drop", "raii"]
---

# RAII Pattern

Model a temporary directory that announces when it is created and, thanks to `Drop`, when it is automatically removed.

## Your Task

1. Give `TempDir` a `String` field holding the directory name.
2. Add a constructor `TempDir::new(name: &str) -> Self` that prints `Created temp dir: <name>` and stores the name.
3. Implement the `Drop` trait for `TempDir` so `drop` prints `Removed temp dir: <name>`.
4. In `main`, create a `TempDir` inside an inner `{ }` block, then print `Main continues` **after** the block.

## Expected Result

Running the program prints, in order:

```
Created temp dir: tmp_data
Removed temp dir: tmp_data
Main continues
```

The `Removed` line appears **before** `Main continues`, because the `TempDir` is dropped at the end of its block.
