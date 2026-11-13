---
id          = "conc_async_await"
name        = "Async and Await"
language    = "rust"
difficulty  = 2
description = "Turn a function into an async fn and await another future inside it."
topics      = ["async", "await", "future", "tokio"]
---

# Async and Await

A chat server greets every user by name. The name comes from a slow database: the given function `load_user(id)` is an `async fn` that takes 50 ms and returns the name `user-<id>`.

The function `greeting(id)` should load the name and return the greeting. At the moment it is an ordinary function that uses the placeholder name `?`.

## Your Task

Change `greeting` in two places. Do not change `load_user`, `main`, the tests or `Cargo.toml`.

1. Make the function asynchronous: write `async` in front of `fn`, so the first line reads `async fn greeting(id: u32) -> String`. Leave the return type `String` as it is.
2. Replace the placeholder line `let name = String::from("?");` with a call to `load_user` that waits for the result: `let name = load_user(id).await;`.
3. Keep the last line `format!("Hello, {name}!")`.

The file does **not compile** at first. `main` and the tests already call `greeting(...).await`, and an ordinary function returns a `String`, which cannot be awaited. The error disappears with step 1.

```
greeting(7).await  → "Hello, user-7!"
greeting(42).await → "Hello, user-42!"
```

## Expected Result

- `greeting` is an `async fn` that returns the greeting with the loaded user name.
- It takes at least 50 ms, because it really waits for `load_user`.
- `main` prints `Hello, user-1!` and `Hello, user-2!`.

> The first build downloads Tokio from crates.io, so a working internet connection is required.
