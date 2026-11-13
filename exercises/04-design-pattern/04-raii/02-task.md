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

The function `report(message: String)` is provided. It prints the message like `println!` and remembers it, so that the tests can check what was printed. Use it for every line you print.

## Your Task

1. Give `TempDir` a `String` field holding the directory name.
2. Complete `TempDir::new(name: &str) -> Self`: call `report(format!("Created temp dir: {name}"))`, then return a `TempDir` that stores the name.
3. Complete `drop` in the `Drop` implementation: report `Removed temp dir: <name>`.
4. In `main`, create a `TempDir` named `tmp_data` inside an inner `{ }` block, **before** the line that reports `Main continues`.

## Expected Result

Running the program prints, in order:

```
Created temp dir: tmp_data
Removed temp dir: tmp_data
Main continues
```

The `Removed` line appears **before** `Main continues`, because the `TempDir` is dropped at the end of its block. Two directories created in the same block are removed in reverse order.
