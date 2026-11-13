# Many Producers, One Consumer

`mpsc` stands for **multi-producer, single-consumer**: many threads can send into the same channel, one thread receives.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    for name in ["north", "south"] {
        let tx = tx.clone(); // one sender per thread
        thread::spawn(move || {
            tx.send(format!("{name} gate open")).expect("receiver is alive");
        });
    }
    drop(tx); // the original sender is not needed any more

    for message in rx {
        println!("{message}"); // both messages, in any order
    }
}
```

## Cloning the sender

- `tx.clone()` creates another sender for the **same** channel. Clone it once per thread and `move` the clone into the thread.
- The receiver cannot be cloned. There is exactly one consumer.

## Receiving until the end

- `for message in rx` and `rx.iter().collect()` receive value after value. They stop when the channel is **closed**: all senders are dropped and no value is left.
- The clones are dropped when their threads end. The **original** `tx` lives on in the function that created the channel, and as long as it exists the receiver **waits forever**. Call `drop(tx)` after the last clone has been made.
- Messages of different threads arrive in the order the scheduler runs them. Do not rely on it.
