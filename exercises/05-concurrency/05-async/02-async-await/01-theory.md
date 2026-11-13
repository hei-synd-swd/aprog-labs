# `async` and `.await`

Asynchronous code reads like sequential code, but it does not block the thread while it waits.

```rust
use std::time::Duration;
use tokio::time::sleep;

async fn boil_water() -> u32 {
    sleep(Duration::from_secs(1)).await; // wait without blocking
    100
}

async fn make_tea() -> String {
    let temperature = boil_water().await; // wait for the result
    format!("tea at {temperature} degrees")
}

#[tokio::main]
async fn main() {
    println!("{}", make_tea().await);
}
```

## `async fn`

- The keyword `async` in front of `fn` turns a function into one that returns a **future**: a value that describes work to be done.
- Calling it runs **nothing**. `boil_water()` alone only creates the future.
- The return type stays as written. `async fn boil_water() -> u32` produces a `u32` once it is awaited.

## `.await`

- `.await` runs a future to completion and yields its result: `boil_water().await` is a `u32`.
- While the future is not ready, `.await` **pauses** the current function and lets the thread do other work. Nothing is blocked.
- `.await` is only allowed **inside** an `async fn` or an async block. A function that awaits must therefore be `async` itself.

## The runtime

Somebody has to drive the outermost future. `#[tokio::main]` starts the Tokio **runtime** and runs the async `main` on it. In tests, `#[tokio::test]` does the same.
