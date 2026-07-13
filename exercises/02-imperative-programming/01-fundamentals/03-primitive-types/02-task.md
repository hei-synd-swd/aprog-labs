---
id          = "impp_primitive_types"
name        = "Primitive Types"
language    = "rust"
difficulty  = 2
description = "Work with integer associated constants, char methods, and bool logic."
topics      = ["primitive-types", "integer", "char", "bool"]
---

# Primitive Types

Implement three small functions that each touch a different scalar type:

1. `u8_max() -> u8` — return the largest value a `u8` can hold, using its
   associated constant (no magic number).
2. `is_vowel(c: char) -> bool` — return `true` if `c` is one of the lowercase
   English vowels `a, e, i, o, u`.
3. `logical_xor(a: bool, b: bool) -> bool` — return `true` when **exactly one**
   of the arguments is `true` (exclusive OR).

## Expected Result

```
u8_max()                  → 255
is_vowel('e')             → true
is_vowel('z')             → false
logical_xor(true, false)  → true
logical_xor(true, true)   → false
```
