---
id          = "impp_variables"
name        = "Variables and Mutability"
language    = "rust"
difficulty  = 1
description = "Use a mutable binding to accumulate a sum inside a loop."
topics      = ["variables", "let", "mut", "loops"]
---

# Variables and Mutability

Implement `sum_readings`, which adds up all sensor readings in a slice and
returns the total. Use a **mutable accumulator** (`let mut`) updated inside a
loop.

## Expected Result

```
sum_readings(&[])             → 0.0
sum_readings(&[1.5, 2.5])     → 4.0
sum_readings(&[17.3, 19.1])   → 36.4
```
