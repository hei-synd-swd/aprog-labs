---
id          = "impp_temperature_conversion"
name        = "Temperature Conversion"
language    = "rust"
difficulty  = 3
description = "Implement a custom From conversion between temperature units."
topics      = ["conversion", "from", "into", "traits", "structs"]
---

# Temperature Conversion

Two newtype structs are provided: `Celsius(f64)` and `Fahrenheit(f64)`.

Implement `From<Celsius> for Fahrenheit` using the formula:

```
°F = °C × 9 / 5 + 32
```

Once `From` is implemented, `Into` works automatically — both
`Fahrenheit::from(Celsius(c))` and `let f: Fahrenheit = Celsius(c).into();`
must produce the right value.

## Expected Result

```
Celsius(0.0)   → Fahrenheit(32.0)
Celsius(100.0) → Fahrenheit(212.0)
Celsius(-40.0) → Fahrenheit(-40.0)
```
