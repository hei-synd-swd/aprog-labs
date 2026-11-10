# Writing Unit Tests

A test in Rust is any function annotated with `#[test]`, usually grouped in a module annotated with `#[cfg(test)]` so test code is only compiled when needed:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 90_percent_is_a_5_5() {
        assert_eq!(grade(90), "5.5");
    }
}
```

There are three assertion macros:

| Macro                      | Use for                 | Failure message shows          |
|----------------------------|-------------------------|--------------------------------|
| `assert!(expr)`            | any boolean property    | the expression text            |
| `assert_eq!(a, b)`         | exact equality          | `left` vs `right` values       |
| `assert_ne!(a, b)`         | inequality              | the equal values               |

All three accept a custom message: `assert!(ok, "okay's value: {}", ok)`.

**Expected panics:** some functions might be expected to panic. `#[should_panic]` asserts that they do. You may set `expected = "..."` to search for a substring in the panic message so that the test cannot pass for a wrong reason:

```rust
#[test]
#[should_panic(expected = "score must be 0..=100")]
fn grade_rejects_101() {
    grade(101);
}
```

Good tests are built from some *requirements*:

- **Enforce the specification**: Do not write tests that the code passes, write code that passes the tests instead. The specification is the source of truth.
- **Boundaries**: off-by-one bugs live at the boundaries. Make sure to test both the upper and lower threshold.
- **Cover all outcomes**: an `Option`-returning function needs a `Some` case *and* a `None` case.
- **Known fixed points**: mathematical functions normally have a value that acts is a fixed point (e.g. `-40 C == -40 F`, `3600s == 60m == 1h`).
- **One behaviour per test**: if a test name needs the word "and", it should probably be split into two separate ones.

Convention: name tests after the behaviour they verify: (e.g. `grade_rejects_101` not `test1`). A failing test name should give immediate info about what went wrong.
