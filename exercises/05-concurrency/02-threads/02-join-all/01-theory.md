# Many Threads, Many Handles

To run several jobs at the same time, spawn one thread per job and keep every `JoinHandle`. Join the handles only **after** all threads have been started.

```rust
use std::thread;

fn main() {
    let names = vec![String::from("Ferris"), String::from("Corro")];

    // 1. start all threads, collect their handles
    let mut handles = Vec::new();
    for name in names {
        handles.push(thread::spawn(move || name.len()));
    }

    // 2. wait for all of them, collect their results
    let mut lengths = Vec::new();
    for handle in handles {
        lengths.push(handle.join().expect("worker does not panic"));
    }

    println!("{lengths:?}"); // [6, 5]
}
```

## Spawn first, join later

- If you `join` inside the first loop, every thread finishes before the next one starts. The program is then sequential again.
- With two loops all threads run at the same time, and the second loop only collects.

## The order of the results

- The threads may **finish** in any order. The OS scheduler decides that.
- The results still come back in the order of the **handles**, because each `join` waits for one specific thread.
- `for name in names` moves each `String` out of the vector, and the `move` closure passes it on to its thread.
