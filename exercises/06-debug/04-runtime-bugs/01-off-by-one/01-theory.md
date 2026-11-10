# Runtime Bugs

Some bugs hide from the compiler: the program **compiles cleanly** but it still misbehaves. There are two types of panics that often occur:

**Panic messages.** When Rust detects an impossible state at runtime (index out of bounds, wrong range, arithmetic overflow in debug builds, explicit `panic!`), it aborts the thread with a message that names the file, line and *reason*:

```text
thread 'main' panicked at src/main.rs:14:30:
index out of bounds: the len is 12 but the index is 15
```

Read it as a fact, not an insult: *something asked for element 15 of a 12-element slice*. The interesting question is which computation produced the 15 - usually an off-by-one or a missing clamp.

**Assertion failures.** `assert_eq!(got, expected)` prints both sides:

```text
assertion `left == right` failed
  left: 9
 right: 10
```

`left` is what the code produced, `right` is what the spec says it should be. The gap between them is the bug's fingerprint - here, a count that did not round up.

Classic integer traps behind such bugs:

- **Integer division truncates**: `11 / 10` is `1`, but you usually need 2 pages. Round up with `a.div_ceil(b)` (or `(a + b - 1) / b`).
- **Exclusive ends**: a page showing items 20..30 of 25 does not exist - clamp ends with `.min(limit)` so `start <= end <= total` always holds.
- **Last index vs count**: 3 pages have last *index* 2. If a `0` result is possible, `saturating_sub(1)` keeps the subtraction from underflowing.

Note what you cannot rely on: a green `cargo build`. For these bugs the test suite *is* the debugger - each red assertion is a fact about the gap between specification and behaviour.
