---
id          = "conc_box_list"
name        = "Box List"
language    = "rust"
difficulty  = 2
description = "Build a recursive linked list with Box<T>."
topics      = ["box", "heap", "recursive type", "enum", "recursion"]
---

# Box List

Build a linked list of `i32` values. A list is either empty (`Nil`), or it is one value followed by the rest of the list (`Cons`). The rest is a list again, so the type is recursive.

```
1 -> 2 -> 3 -> Nil      is      Cons(1, Cons(2, Cons(3, Nil)))
```

## Your Task

Complete the three `TODO`s. Do not change the signatures, `main` or the tests.

1. **The enum.** Add a variant `Cons` to `List` that holds two things: an `i32` and the rest of the list inside a box. Write it as `Cons(i32, Box<List>)` and keep the variant `Nil`.
2. **`prepend(self, value: i32) -> List`** returns a new list with `value` in front of the old one. Return `List::Cons(value, Box::new(self))`.
3. **`sum(&self) -> i32`** adds up all values. Use a `match self` with two arms:
   - `List::Cons(value, rest)` returns `value + rest.sum()`
   - `List::Nil` returns `0`

```
List::Nil.sum()                                  → 0
List::Nil.prepend(3)                             → Cons(3, Nil)
List::Nil.prepend(3).prepend(2).prepend(1)       → Cons(1, Cons(2, Cons(3, Nil)))
List::Nil.prepend(3).prepend(2).prepend(1).sum() → 6
```

## Expected Result

- `List` compiles, which it only does when the rest of the list is boxed.
- `prepend` puts the new value at the **front**. The last value prepended is the first one in the list.
- `sum` returns the sum of all values, and `0` for the empty list.
- `main` prints `Cons(1, Cons(2, Cons(3, Nil)))` and `sum = 6`.
