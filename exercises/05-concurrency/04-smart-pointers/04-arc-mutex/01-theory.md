# Shared State Across Threads

Threads that work on the **same** value need shared ownership that is safe across threads (`Arc`) and a way to change the value without a data race (`Mutex` or an atomic).

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let log = Arc::new(Mutex::new(Vec::new()));

    let mut handles = Vec::new();
    for id in 0..3 {
        let log = Arc::clone(&log); // one owner per thread
        handles.push(thread::spawn(move || {
            log.lock().expect("lock is not poisoned").push(id);
        }));
    }
    for handle in handles {
        handle.join().expect("worker does not panic");
    }

    println!("{:?}", log.lock().expect("lock is not poisoned")); // e.g. [0, 2, 1]
}
```

## `Arc<T>` and `Mutex<T>`

- `Arc` is the thread-safe twin of `Rc`. Clone it **before** the spawn and `move` the clone into the thread.
- `lock()` blocks until the mutex is free and returns a **guard**, through which the value is read and changed.
- The guard unlocks the mutex when it goes out of scope. `lock()` is `Err` only if a thread panicked while holding the lock.

## `Atomic*`

A single number or flag needs no lock. An atomic type updates it in one indivisible step:

```rust
let hits = AtomicUsize::new(0);
hits.fetch_add(1, Ordering::Relaxed);     // add 1
let total = hits.load(Ordering::Relaxed); // read
```

Atomics exist only for `Copy` values such as integers and booleans. A `Vec` or a `String` needs a `Mutex`.
