---
id          = "encapsulation_temperature"
name        = "Temperature"
language    = "rust"
difficulty  = 3
description = "Add a getter, a validating setter, and a conversion method to a Temperature type."
topics      = ["encapsulation", "getter", "setter", "validation", "private"]
---

# Temperature

A `Temperature` struct is provided with a **private** field `celsius: f64`.
The constructor `new(celsius: f64)` is already implemented.

1. **Add a getter** `pub fn celsius(&self) -> f64` that returns the value.

2. **Add a setter** `pub fn set_celsius(&mut self, value: f64)` that only
   updates the field if `value >= -273.15`. Invalid values are silently
   ignored.

3. **Add a conversion method** `pub fn to_fahrenheit(&self) -> f64` using the
   formula: `celsius × 9.0 / 5.0 + 32.0`.

## Expected Result

- The getter returns the current temperature.
- `set_celsius(100.0)` updates the temperature.
- `set_celsius(-300.0)` is rejected (temperature unchanged).
- `to_fahrenheit()` converts correctly (0°C → 32°F, 100°C → 212°F).
