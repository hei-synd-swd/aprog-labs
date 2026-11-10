# Panics and the Contracts That Prevent Them

A **panic** means that Rust found something that isn't possible or would result in undefined behaviour.

This might be:
- index out of bounds
- divide by zero
- integer overflow (in debug builds only)
- unwrapping a `None` or `Err`


The panic message points to a file, line and reason:

```text
thread 'main' (1055525) panicked at expr.rs:23:53:
called `Result::unwrap()` on an `Err` value: "Error!"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Option defines that some data might be absent. When a function returns `Option<T>`, it promises to return either the result of some code or none and does not panic if the Result is none. 

It is important to not run any code that could panic on failure in the body:

```rust
// Panics on empty input
fn first(v: &[u32]) -> Option<u32> {
    Some(v[0])
}

// Returns none if the slice is empty
fn first(v: &[u32]) -> Option<u32> {
    v.first().copied()
}
```

Standard library iterators and slices have a lot functions that return `Option<T>`: `first`/`last`, `get`, `chars().nth(i)`, `checked_add`, `saturating_sub`, `split_next`...

One more thing.. non-standard chars often require more than one byte. This can easily be visualised on unix-like systems with the help of `xxd`:

```
06-debug/04-runtime-bugs/02-fix-the-panics on  feat/06-debug [$✘!+?] via 🦀 v1.100.0-nightly 
❯ printf "a" | xxd
00000000: 61                                       a

06-debug/04-runtime-bugs/02-fix-the-panics on  feat/06-debug [$✘!+?] via 🦀 v1.100.0-nightly 
❯ printf "🩷" | xxd
00000000: f09f a9b7                                ....
```

