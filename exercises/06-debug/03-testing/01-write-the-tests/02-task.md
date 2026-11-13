---
id          = "debug_write_tests"
name        = "Write the Tests"
language    = "rust"
difficulty  = 3
description = "Write a unit test suite for three provided functions, covering boundaries, Option cases and expected panics."
topics      = ["testing", "#[test]", "assert", "should_panic", "edge cases"]
---

# Write the Tests

This time the code is **already written and correct**. You're now required to write the tests for this existing code.

Add your tests to the `student_tests` module in `main.rs` (a commented example shows the shape). Requirements:

1. **Boundaries**: test both sides of *two* grade thresholds (pick any two of `59`/`60`, `69`/`70`, `79`/`80`, `89`/`90`) and the extreme ends (`0`, `100`).
    - Both sides of a grade threshold (e.g. `59/60` or `79/80`)
    - Both extreme ends (`0`, `100`)

2. **Expected panics**: 
    - `grade` must panic on `101`. Use `#[should_panic(expected = "{snip}")]`

3. **`parse_temperature`**:
    - `Some` case
    - `None` case (e.g. missing unit, incorrect unit, etc.)

4. **`celsius_to_fahrenheit`**:
    - fixed point at `-40`
    - Test of integer truncation

## Specifications

| Function | Behaviour |
|----------|-----------|
| `grade(score: u32) -> &'static str` | `90..=100` -> `"A"`, `80..=89` -> `"B"`, `70..=79` -> `"C"`, `60..=69` -> `"D"`, `0..=59` -> `"F"`. Panics with a message containing `score must be 0..=100` for anything above 100. |
| `parse_temperature(raw: &str) -> Option<i32>` | Parses `"<integer>C"` (uppercase `C` suffix, optional leading `-`): `"21C"` -> `Some(21)`, `"-3C"` -> `Some(-3)`. Returns `None` for anything else (missing suffix, wrong unit, empty, non-numeric). |
| `celsius_to_fahrenheit(c: i32) -> i32` | `c * 9 / 5 + 32` using **integer** division - results truncate. |

## Expected Result

All tests pass. That includes yours *and* the provided ones.

A provided test checks that the `student_tests` module contains at least **6** `#[test]` functions and at least one test with `#[should_panic]`. The exercise does not pass before that.

> Do not edit the three provided functions or anything below the `DO NOT EDIT BELOW THIS LINE` marker.
