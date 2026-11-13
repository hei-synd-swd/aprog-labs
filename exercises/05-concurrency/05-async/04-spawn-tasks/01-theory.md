# Spawning Tasks

`tokio::spawn` hands a future to the runtime as an independent **task**. The task starts running in the background right away, and `spawn` returns a `JoinHandle` for its result.

```rust
async fn measure(sensor: u32) -> u32 {
    tokio::time::sleep(Duration::from_millis(200)).await;
    sensor * 10
}

#[tokio::main]
async fn main() {
    let mut handles = Vec::new();
    for sensor in 1..=3 {
        handles.push(tokio::spawn(measure(sensor))); // start all tasks
    }
    for handle in handles {
        let value = handle.await.expect("task does not panic"); // collect
        println!("{value}"); // 10, 20, 30 - after 200 ms in total
    }
}
```

## Tasks and handles

- A task is the async counterpart of a thread, but far cheaper: thousands of tasks share a handful of threads.
- `tokio::spawn(future)` takes the future itself, **without** `.await`.
- Awaiting the `JoinHandle` yields a `Result`: `Ok(value)`, or `Err` if the task panicked.
- Spawn everything first and await afterwards. Awaiting each handle right after its spawn runs the tasks one after the other.

## Never block a task

Tasks are scheduled **cooperatively**: a task keeps its thread until it reaches an `.await`. A blocking call never yields, so every other task on that thread stands still.

- `std::thread::sleep(d)` blocks the whole thread. `tokio::time::sleep(d).await` yields and lets the others run.
- Long computations belong in `tokio::task::spawn_blocking`, a thread or Rayon.
