---
id          = "builder"
name        = "Builder Pattern"
language    = "rust"
difficulty  = 3
description = "Construct a complex object step by step with a chainable builder."
topics      = ["struct", "builder", "method_chaining"]
---

# Builder Pattern

Build a `Pizza` step by step with a chainable `PizzaBuilder`. The `Pizza` struct (fields `size: String` and `toppings: Vec<String>`) is already defined for you.

## Your Task

1. Give `PizzaBuilder` one field: a work-in-progress `Pizza`.
2. `PizzaBuilder::new()` starts a `Pizza` with size `"medium"` and no toppings.
3. Add two chaining methods that take `self` and return `Self`: `size(self, s: &str)` sets the size, and `topping(self, t: &str)` appends one topping.
4. `build(self) -> Pizza` consumes the builder and returns the inner `Pizza`.
5. In `main`, build a `"large"` pizza with at least two toppings and print it with the `Debug` formatter (`{:?}`).

## Expected Result

`PizzaBuilder::new().build()` yields a medium pizza with no toppings, while `PizzaBuilder::new().size("large").topping("cheese").topping("pepperoni").build()` yields `Pizza { size: "large", toppings: ["cheese", "pepperoni"] }`.
