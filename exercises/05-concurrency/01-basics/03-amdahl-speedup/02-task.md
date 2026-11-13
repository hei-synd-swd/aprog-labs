---
id          = "conc_amdahl_speedup"
name        = "Amdahl's Law"
language    = "rust"
difficulty  = 2
description = "Compute the speedup of a program with Amdahl's Law."
topics      = ["amdahl", "speedup", "parallel fraction", "f64"]
---

# Amdahl's Law

Compute how much faster a program gets when a part of it runs on several processors.

## Your Task

Replace the `todo!()` in `speedup(parallel_fraction: f64, processors: u32) -> f64`. Do not change the signature, `main` or the tests.

1. Convert `processors` to a floating point number: `let p = f64::from(processors);`.
2. Compute the serial fraction: `1.0 - parallel_fraction`.
3. Return Amdahl's Law: `1.0 / (serial_fraction + parallel_fraction / p)`.

`parallel_fraction` is always between `0.0` and `1.0`, and `processors` is at least 1. You do not have to check this.

```
speedup(0.5, 2)  → 1.3333
speedup(0.9, 10) → 5.2632
speedup(0.0, 8)  → 1.0      (nothing is parallel)
speedup(1.0, 4)  → 4.0      (everything is parallel)
speedup(0.9, 1)  → 1.0      (one processor)
```

## Expected Result

- `speedup` returns the value of Amdahl's Law for the given parallel fraction and number of processors.
- `main` prints the speedup of a program that is 90 % parallel for 1, 2, 4, 8, 16 and 1024 processors. The values grow more and more slowly and stay below 10.
