---
id          = "conc_broadcast"
name        = "Broadcast"
language    = "rust"
difficulty  = 4
description = "Fan out messages to several subscriber tasks with a broadcast channel."
topics      = ["channel", "broadcast", "tokio", "subscribe", "fan-out"]
---

# Broadcast

A theatre announces events to all its displays. Every display is a task that subscribes to the announcements and collects everything it receives in its own inbox. At the end, each inbox must hold **every** message, in the order it was sent.

## Your Task

Replace the `todo!()` in `announce(messages: Vec<String>, subscribers: usize) -> Vec<Vec<String>>`. It returns one inbox per subscriber. Do not change the signature, `main`, the tests or `Cargo.toml`.

**Step 1 - the channel.** Create a broadcast channel that can hold all messages at once: `let (tx, _) = broadcast::channel(messages.len().max(1));`. The capacity must be at least 1, hence the `max(1)`. The first receiver is not needed, so it is discarded with `_`.

**Step 2 - the subscribers.** Create an empty vector `handles`. Loop `subscribers` times with `for _ in 0..subscribers`. In the loop:

1. Subscribe: `let mut rx = tx.subscribe();`. This must happen **before** any message is sent.
2. Spawn a task with `tokio::spawn(async move { ... })` and push its handle into `handles`.
3. Inside the async block, create `let mut inbox = Vec::new();`, receive until the channel is closed with `while let Ok(message) = rx.recv().await { inbox.push(message); }`, and make `inbox` the last expression of the block, so the task returns it.

**Step 3 - send.** After the loop, send every message: `for message in messages { let _ = tx.send(message); }`. The `let _ =` ignores the error that `send` returns when there is no subscriber.

**Step 4 - close.** Drop the sender: `drop(tx);`. Without this line the subscribers wait for more messages forever.

**Step 5 - collect.** Create an empty vector `inboxes`. Await every handle in order with `handle.await.expect("the task does not panic")`, push each returned inbox into `inboxes`, and return `inboxes`.

```
announce(vec!["a", "b", "c"], 2).await → [["a", "b", "c"], ["a", "b", "c"]]
announce(vec!["a", "b"], 0).await      → []            no subscriber, no inbox
announce(vec![], 2).await              → [[], []]      two empty inboxes
```

## Expected Result

- The result has exactly `subscribers` inboxes.
- Every inbox contains **all** messages, in the order they were sent. No message is missed, also for more than a few messages.
- The function returns for any input. It never waits forever.
- `main` prints three lines, one per subscriber, each with the same three announcements.

> The first build downloads Tokio from crates.io, so a working internet connection is required.
