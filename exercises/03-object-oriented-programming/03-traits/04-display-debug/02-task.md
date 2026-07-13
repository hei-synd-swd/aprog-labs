---
id          = "trait_display"
name        = "Display and Debug"
language    = "rust"
difficulty  = 4
description = "Derive Debug and implement Display for a custom type."
topics      = ["Display", "Debug", "fmt", "derive"]
---

# Display and Debug

A `Book` struct is provided.

1. **Add `#[derive(Debug)]`** to `Book`.
2. **Implement `Display`** for `Book` so that `{}` prints:
   `"{title} by {author} ({year})"` — for example:
   `"1984 by George Orwell (1949)"`.

## Expected Result

- `println!("{:?}", book)` prints the debug representation.
- `println!("{}", book)` prints the user-friendly display format.
