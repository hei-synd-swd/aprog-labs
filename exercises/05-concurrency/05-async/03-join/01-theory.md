# Running Futures Concurrently

Calling an `async fn` does not run it. It only creates a **future**, which runs once it is awaited. How you await several futures decides whether their waiting times add up or overlap.

## One after the other

Each `.await` finishes before the next line starts, so the waiting times **add up**:

```rust
let header = download("header.html").await; // 2 s
let body = download("body.html").await;     // 3 s
// ready after 5 s
```

## Both at once

`tokio::join!` drives several futures **concurrently** and returns a tuple with all their results:

```rust
let (header, body) = tokio::join!(
    download("header.html"), // 2 s
    download("body.html"),   // 3 s
);
// ready after 3 s
```

- Pass the futures to `join!` **without** `.await`. The macro awaits them for you.
- While one future waits, the other one makes progress. The total time is that of the **slowest** future.
- The results come back in the order of the arguments, not in the order of completion.

## Concurrent, not parallel

The futures share **one thread** and take turns at every `.await`. That speeds up waiting (network, disk, timers), not computing.

A pause in async code is written `tokio::time::sleep(duration).await`. It lets the other futures run, whereas `std::thread::sleep` would block all of them.
