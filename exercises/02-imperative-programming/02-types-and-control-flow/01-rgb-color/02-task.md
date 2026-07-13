---
id          = "impp_rgb_color"
name        = "RGB Color Structure"
language    = "rust"
difficulty  = 2
description = "Define a struct, write a formatting function, and create color constants."
topics      = ["struct", "constants", "formatting"]
---

# RGB Color Structure

Implement an `Rgb` struct that represents a 24-bit RGB color.

1. Define a struct `Rgb` with three `u8` fields: `r`, `g`, `b`.
2. Write a function `to_hex(rgb: &Rgb) -> String` that returns the color
   as `#RRGGBB` (e.g. `to_hex(&Rgb { r: 255, g: 0, b: 0 })` → `"#FF0000"`).
3. Define these **top-level constants**:
   - `WHITE`  → `Rgb { r: 255, g: 255, b: 255 }`
   - `BLACK`  → `Rgb { r: 0, g: 0, b: 0 }`
   - `RED`    → `Rgb { r: 255, g: 0, b: 0 }`
   - `GREEN`  → `Rgb { r: 0, g: 255, b: 0 }`
   - `BLUE`   → `Rgb { r: 0, g: 0, b: 255 }`

## Expected Result

```
to_hex(&WHITE)  → "#FFFFFF"
to_hex(&BLACK)  → "#000000"
to_hex(&RED)    → "#FF0000"
to_hex(&Rgb { r: 17, g: 34, b: 51 }) → "#112233"
```
