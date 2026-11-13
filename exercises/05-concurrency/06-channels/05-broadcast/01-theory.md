# `broadcast`: Every Subscriber Gets Every Message

In an `mpsc` channel each message is received **once**, by the single consumer. A `broadcast` channel delivers each message to **all** receivers: fan-out.

```rust
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut first) = broadcast::channel(8); // capacity 8
    let mut second = tx.subscribe();            // one more receiver

    tx.send("server restarts").expect("there are receivers");
    drop(tx); // close the channel

    while let Ok(message) = first.recv().await {
        println!("first got: {message}");
    }
    while let Ok(message) = second.recv().await {
        println!("second got: {message}");
    }
}
```

## Subscribing

- `broadcast::channel(capacity)` returns a sender and a first receiver.
- `tx.subscribe()` creates a further receiver. A receiver only gets messages sent **after** it subscribed, so subscribe before sending.
- Every receiver gets its own copy of each message. The message type must therefore be `Clone`.

## Sending

- `tx.send(message)` is not async and never waits. It returns `Err` only if there is **no receiver** at all.
- The channel keeps at most `capacity` messages. If a receiver is slower than that, it misses the oldest ones and its next `recv` returns an error. Choose the capacity large enough for the messages that can pile up.

## Receiving until the end

- `rx.recv().await` yields `Ok(message)`.
- When **all senders are dropped** and a receiver has read everything, `recv` returns `Err`. A `while let Ok(message) = rx.recv().await` loop ends at that point.
- As with `mpsc`, a forgotten sender keeps the channel open and the receivers wait forever. Call `drop(tx)` after the last send.
