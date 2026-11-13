---
id          = "std_traits"
name        = "Standard Library Traits"
language    = "rust"
difficulty  = 2
description = "Derive the common traits for a type and overload the + operator with Add."
topics      = ["traits", "derive", "default", "copy", "partialord", "operator", "add"]
---

# Standard Library Traits

`Money` stores an amount in cents. Make it behave like a number: printable, copyable, comparable, with a default of zero and a working `+`.

## Your Task

1. Put one `#[derive(...)]` line above `struct Money` with `Debug`, `Clone`, `Copy`, `PartialEq`, `PartialOrd` and `Default`.
2. Implement the trait `Add` for `Money`: set `type Output = Money;` and write `fn add(self, other: Money) -> Money`, which returns a `Money` with the sum of both `cents`.
3. Replace the `todo!()` in `total(prices: &[Money]) -> Money`: start with `Money::default()` and add every price to it with `+`.

```
Money::default()                          → Money { cents: 0 }
Money { cents: 450 } + Money { cents: 620 } → Money { cents: 1070 }
Money { cents: 450 } < Money { cents: 620 } → true
total(&[])                                → Money { cents: 0 }
```

## Expected Result

- `Money` can be printed with `{:?}`, compared with `==` and `<`, and used again after it was assigned to another variable.
- `a + b` returns the sum, and `total` returns the sum of a whole slice (zero for an empty one).
- `main` prints `Money { cents: 1070 }`, `coffee is cheaper: true` and `Money { cents: 1520 }`.
