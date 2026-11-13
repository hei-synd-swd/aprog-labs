---
id          = "conc_oneshot_reply"
name        = "Oneshot Reply"
language    = "rust"
difficulty  = 3
description = "Start a background task that delivers its result through a oneshot channel."
topics      = ["channel", "oneshot", "tokio", "spawn", "async"]
---

# Oneshot Reply

A slow computation runs as a background task. The caller gets a receiver right away and awaits the single result whenever it wants.

The given function `slow_square(n)` is an `async fn` that takes 100 ms and returns `n * n`.

## Your Task

Replace the `todo!()` in `start_job(n: u64) -> oneshot::Receiver<u64>`. Do not change `slow_square`, the signature, `main`, the tests or `Cargo.toml`. Note that `start_job` is **not** an `async fn`: it only starts the job and returns.

1. Create a oneshot channel: `let (tx, rx) = oneshot::channel();`.
2. Spawn a task with an async block that owns the sender: `tokio::spawn(async move { ... });`.
3. Inside the block, compute the result: `let square = slow_square(n).await;`.
4. Still inside the block, send it: `let _ = tx.send(square);`. The `let _ =` ignores the error that `send` returns when the receiver was dropped.
5. After the spawn, return `rx`. Do not await anything in `start_job`.

```
let reply = start_job(7);     returns immediately
reply.await → Ok(49)          about 100 ms later
```

## Expected Result

- `start_job` returns the receiver without waiting for the computation.
- Awaiting the receiver yields `Ok(n * n)` after about 100 ms.
- Two jobs started one after the other run concurrently: both replies are there after about 100 ms, not 200 ms.
- `main` prints `job started, main is free to do other things` and then `the job answered 144`.

> The first build downloads Tokio from crates.io, so a working internet connection is required.
