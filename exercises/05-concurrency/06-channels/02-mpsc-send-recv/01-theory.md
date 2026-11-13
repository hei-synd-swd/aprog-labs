# Channels: Send and Receive

A **channel** moves values from one thread to another. It has two ends: a **sender** and a **receiver**.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel(); // sender, receiver

    thread::spawn(move || {
        let report = String::from("backup finished");
        tx.send(report).expect("receiver is alive"); // `report` moves away
    });

    let message = rx.recv().expect("sender sent a value"); // blocks
    println!("{message}");
}
```

## The two ends

- `mpsc::channel()` returns the pair `(tx, rx)`. The type of the values is inferred from what is sent.
- The sender usually goes to another thread. A `move` closure takes ownership of `tx`.
- The receiver can be kept, or returned from a function, like any other value. Its type is `Receiver<T>`.

## `send`

- `tx.send(value)` **moves** the value into the channel. The sending thread cannot use it afterwards.
- It never waits. It returns `Err` only if the receiver no longer exists.

## `recv`

- `rx.recv()` **blocks** until a value arrives and returns `Ok(value)`.
- When **every sender has been dropped** and the channel is empty, it returns `Err`. That is how a receiver learns that nothing more will come.
- A sender is dropped when it goes out of scope, for example when the thread that owns it ends.
