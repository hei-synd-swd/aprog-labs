---
id          = "conc_mpsc_send_recv"
name        = "Send and Receive"
language    = "rust"
difficulty  = 2
description = "Let a worker thread send its result through an mpsc channel."
topics      = ["channel", "mpsc", "send", "recv", "threads"]
---

# Send and Receive

A worker thread computes a square and reports it through a channel. The caller only gets the receiving end and picks up the result when it is ready.

## Your Task

Replace the `todo!()` in `start_worker(n: u64) -> Receiver<u64>`. Do not change the signature, `main` or the tests.

1. Create a channel: `let (tx, rx) = mpsc::channel();`.
2. Spawn a thread with a `move` closure, so that the closure takes ownership of `tx`.
3. Inside the closure, send the square: `tx.send(n * n).expect("the receiver is still alive");`.
4. After the spawn, return `rx`. Do not join the thread and do not call `recv` here: receiving is the caller's job.

```
let rx = start_worker(7);
rx.recv() → Ok(49)     the value the worker sent
rx.recv() → Err(..)    the worker has finished, its sender is gone
```

## Expected Result

- `start_worker` returns right away with the receiver. The worker runs in the background.
- The first `recv()` on the returned receiver yields `Ok(n * n)`.
- A second `recv()` yields `Err`, because the worker sends exactly one value and then its sender is dropped.
- `main` prints `main waits for the worker...` and then `the worker sent 144`.
