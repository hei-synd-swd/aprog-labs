---
id          = "inheritance_composition_car"
name        = "Car Engine Composition"
language    = "rust"
difficulty  = 3
description = "Model a has-a relationship by composing a Car with an Engine via delegation."
topics      = ["composition", "delegation", "has-a", "inheritance"]
---

# Car Engine Composition

An `Engine` struct is provided with a `start()` method. A `Car` struct holds
an **`engine: Engine`** field — a **has-a** relationship.

The `Car` already delegates `start()` to its engine.

1. **Add a method** `fn fuel_type(&self) -> &str` to `Engine` that returns a
   reference to the engine's `fuel` field.

2. **Add a method** `fn fuel_type(&self) -> String` to `Car` that delegates
   to `self.engine.fuel_type()` and returns a string in the format:
   `"{make} uses {fuel}"`.

## Expected Result

- `Engine::fuel_type()` returns the fuel type string.
- `Car::fuel_type()` delegates to the engine and includes the car's make.
- The car's `start()` method still works and includes both make and engine
  horsepower.
