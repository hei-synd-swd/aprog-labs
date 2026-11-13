# `oneshot`: One Value, Once

A `oneshot` channel carries **exactly one** value. It is the natural way for a background task to deliver its result, or to answer a request.

```rust
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        let answer = 6 * 7; // some work in the background
        let _ = tx.send(answer);
    });

    match rx.await {
        Ok(answer) => println!("the answer is {answer}"),
        Err(_) => println!("the task gave no answer"),
    }
}
```

## The two ends

- `oneshot::channel()` returns `(tx, rx)`: a `Sender<T>` and a `Receiver<T>`.
- `tx.send(value)` **consumes** the sender. A second send is a compile error, the type system guarantees "once".
- `send` is not async and never waits. It returns `Err(value)` if the receiver was dropped, which means nobody is interested in the answer any more. Ignoring that case with `let _ = ...` is usually fine.

## Receiving

- The receiver is itself a **future**. `rx.await` waits for the value without blocking the thread.
- It yields a `Result`: `Ok(value)`, or `Err` if the sender was dropped without sending.
- A `Receiver<T>` can be returned from a function. The caller decides when to await it.

## Async blocks

`tokio::spawn` takes a future. An `async move { ... }` block creates one on the spot and moves the variables it uses, here `tx`, into the task. Inside the block `.await` is allowed.
