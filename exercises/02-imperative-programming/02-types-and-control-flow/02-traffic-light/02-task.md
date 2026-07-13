---
id          = "impp_traffic_light"
name        = "Traffic Light Controller"
language    = "rust"
difficulty  = 2
description = "Model traffic light states with an enum and match-based transitions."
topics      = ["enum", "match"]
---

# Traffic Light Controller

Model a traffic light with three states and transitions between them.

1. Define an enum `TrafficLight` with variants `Red`, `Yellow`, `Green`.
2. Write a function `next(light: &TrafficLight) -> TrafficLight` that cycles:
   - `Red` → `Green`
   - `Green` → `Yellow`
   - `Yellow` → `Red`
3. Write a function `is_stop(light: &TrafficLight) -> bool` that returns
   `true` only when the light is `Red`.

## Expected Result

```
let mut light = TrafficLight::Red;
light = next(&light);       // Green
is_stop(&light)              // false
light = next(&light);       // Yellow
light = next(&light);       // Red
is_stop(&light)              // true
```
