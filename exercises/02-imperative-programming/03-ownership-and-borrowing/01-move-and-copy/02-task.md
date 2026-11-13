---
id          = "impp_move_and_copy"
name        = "Move and Copy"
language    = "rust"
difficulty  = 2
description = "See which values move and which are copied, and use clone to get a second owner."
topics      = ["ownership", "move", "copy", "clone", "drop"]
---

# Move and Copy

Three small functions show how ownership behaves: a number is copied, a `String` moves, and a value that is not returned is dropped.

## Your Task

Replace the three `todo!()`. Do not change the signatures, `main` or the tests.

**`pair_of(n: u32) -> (u32, u32)`** returns the number twice.

1. Return the tuple `(n, n)`. This compiles because `u32` is a `Copy` type: `n` can be used twice.

**`two_copies(text: String) -> (String, String)`** returns the text twice.

1. First try the same idea: return `(text, text)`. Read the compiler error `E0382: use of moved value`. A `String` has one owner, so it cannot be put into the tuple twice.
2. Create a second, independent `String`: `let copy = text.clone();`.
3. Return `(text, copy)`.

**`keep_longer(a: String, b: String) -> String`** returns the longer of the two texts. If both have the same length, it returns `a`.

1. Write an `if a.len() >= b.len()` with an `else`.
2. Return `a` in the `if` branch and `b` in the `else` branch. No `clone()` is needed: the chosen `String` moves out to the caller, and the other one is dropped when the function ends.

```
pair_of(7)                                               → (7, 7)
two_copies(String::from("ab"))                           → ("ab", "ab")
keep_longer(String::from("Rust"), String::from("Ferris")) → "Ferris"
keep_longer(String::from("abc"), String::from("xyz"))     → "abc"
```

## Expected Result

- `pair_of` returns the number twice, and the caller can still use its own variable afterwards.
- `two_copies` returns two independent strings: changing one does not change the other.
- `keep_longer` returns the longer text, and the first one if the lengths are equal.
- `main` prints `(21, 21)`, `n is still usable: 21`, `Ferris and Ferris` and `Ferris`.
