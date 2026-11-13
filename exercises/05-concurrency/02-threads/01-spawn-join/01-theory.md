# Spawning and Joining Threads

`thread::spawn` runs a closure in a **new thread**, alongside the thread that called it. It returns a `JoinHandle` right away. Calling `join()` on that handle waits for the thread and returns the closure's value.

```rust
use std::thread;

fn main() {
    let name = String::from("Ferris");

    let handle = thread::spawn(move || {
        println!("worker runs on {:?}", thread::current().id());
        name.len() // last expression = return value of the thread
    });

    let length = handle.join().expect("worker does not panic");
    println!("{length}"); // 6
}
```

## Spawn and join

- The closure's last expression is the **return value** of the thread. Its type becomes the `T` of `JoinHandle<T>`.
- `join()` **blocks** the caller until the thread is done. It returns a `Result`: `Ok(value)`, or `Err` if the thread panicked.
- Without `join`, `main` can end first and the thread is killed mid-run.

## Moving data into the thread

- A thread may outlive the function that spawned it, so it cannot borrow local variables.
- `move` makes the closure **take ownership** of everything it uses. After the spawn, `name` is no longer usable in `main`.

## Thread ids

Every thread has a unique `ThreadId`. `thread::current().id()` returns the id of the thread that executes the call, so inside the closure it is the worker's id.
