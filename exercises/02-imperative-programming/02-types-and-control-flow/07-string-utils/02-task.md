---
id          = "impp_string_utils"
name        = "String Utils"
language    = "rust"
difficulty  = 3
description = "Use immutable and mutable references to work with strings."
topics      = ["borrowing", "references", "mutability", "string"]
---

# String Utils

Practice how Rust's borrowing rules work with immutable (`&`) and
mutable (`&mut`) references.

1. Write `count_chars(text: &str) -> usize` — returns the number of
   characters (immutable borrow).
2. Write `first_word(text: &str) -> &str` — returns the first word
   (immutable borrow, returns a slice).
3. Write `add_exclamation(text: &mut String)` — appends `"!"` to the
   string (mutable borrow).
4. Write `clear(text: &mut String)` — empties the string, sets it to
   `String::new()` (mutable borrow).

Key rules demonstrated:
- Multiple immutable borrows (`count_chars`, `first_word`) can coexist.
- A mutable borrow (`add_exclamation`, `clear`) is exclusive — no other
  borrows can exist at the same time.

## Expected Result

```
let mut s = String::from("hello world");
count_chars(&s)      → 11
first_word(&s)       → "hello"
add_exclamation(&mut s);
count_chars(&s)      → 12
clear(&mut s);
count_chars(&s)      → 0
```
