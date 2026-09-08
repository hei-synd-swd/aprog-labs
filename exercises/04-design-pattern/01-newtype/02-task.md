---
id          = "newtype"
name        = "Newtype Pattern"
language    = "rust"
difficulty  = 2
description = "Wrap values in one-field structs for extra type safety."
topics      = ["struct", "newtype", "from"]
---

# Newtype Pattern

Model two length units as distinct types and convert between them. The `From<Feet>` conversion for `Meters` and the `Display` impl for `Meters` are already provided below `main` - you do not write the maths.

## Your Task

1. Define two tuple structs, `Feet` and `Meters`, each wrapping a single `f64` (the `#[derive(Debug, PartialEq)]` lines are already in place for you).
2. Write a function `feet_to_meters(d: Feet) -> Meters` that converts by calling `.into()` on the provided `From` impl.
3. In `main`, create a `Feet` value, convert it, and print the resulting `Meters` **twice**: once with `Debug` (`{:?}`) and once with `Display` (`{}`).

## Expected Result

`feet_to_meters(Feet(10.0))` equals `Meters(3.048)`, and `main` prints the value in both formats, for example `Meters(7.0104)` and `7.0104 Meters!`.
