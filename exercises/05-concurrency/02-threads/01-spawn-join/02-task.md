---
id          = "conc_spawn_join"
name        = "Spawn and Join"
language    = "rust"
difficulty  = 2
description = "Run a computation in a new thread and collect its result."
topics      = ["threads", "spawn", "join", "move", "closure"]
---

# Spawn and Join

Let a worker thread sum a list of numbers while the caller waits for the result.

## Your Task

Replace the `todo!()` in `sum_in_thread(numbers: Vec<u64>) -> (u64, ThreadId)`. Do not change the signature, `main` or the tests.

1. Call `thread::spawn` with a `move` closure and store the returned handle: `let handle = thread::spawn(move || { ... });`. The `move` gives the closure ownership of `numbers`.
2. **Inside** the closure, compute the sum: `let sum: u64 = numbers.iter().sum();`.
3. Still inside the closure, make the tuple `(sum, thread::current().id())` the last expression, without a semicolon. This is the value the thread returns.
4. **After** the closure, call `handle.join()`. It returns a `Result`. Take the tuple out with `.expect("the worker does not panic")` and return it from the function.

```
sum_in_thread(vec![1, 2, 3, 4]) → (10, ThreadId(2))
sum_in_thread(vec![])           → (0,  ThreadId(3))
```

The exact `ThreadId` numbers vary from run to run.

## Expected Result

- The first tuple element is the sum of all numbers, `0` for an empty vector.
- The second tuple element is the id of the **worker** thread. It differs from the id of the calling thread, and every call returns a new id.
- `main` prints the sum with the worker's id, then the id of the main thread.
