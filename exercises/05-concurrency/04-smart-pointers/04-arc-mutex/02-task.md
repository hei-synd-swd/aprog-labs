---
id          = "conc_arc_mutex"
name        = "Arc, Mutex and Atomics"
language    = "rust"
difficulty  = 3
description = "Let several threads fill a shared vector behind a Mutex and update a shared atomic counter."
topics      = ["arc", "mutex", "atomic", "shared state", "threads"]
---

# Arc, Mutex and Atomics

Two small jobs in which several threads work on the same value: once on a vector, which needs a `Mutex`, and once on a counter, for which an atomic is enough.

## Your Task

Replace the two `todo!()`. Do not change the signatures, `main` or the tests. All imports you need are at the top of the file.

**`collect_squares(workers: u64) -> Vec<u64>`** starts one thread per worker id `1..=workers`. Each thread pushes the square of its id into a shared vector.

1. Create the shared vector: `let squares = Arc::new(Mutex::new(Vec::new()));` and an empty vector `handles`.
2. Loop with `for id in 1..=workers`. In the loop:
   - clone the `Arc` for this thread: `let squares = Arc::clone(&squares);`
   - spawn a thread with a `move` closure that locks the mutex and pushes `id * id`: `squares.lock().expect("no thread panicked while holding the lock").push(id * id);`
   - push the handle into `handles`
3. After the loop, join every handle with `.join().expect("the worker does not panic")`.
4. Copy the vector out of the mutex: `let mut sorted = squares.lock().expect("no thread panicked while holding the lock").clone();`.
5. The threads finish in any order, so sort the copy with `sorted.sort();` and return it.

**`count_long_words(words: Vec<String>, min_len: usize) -> usize`** starts one thread per word. A thread adds 1 to a shared counter if its word has at least `min_len` bytes.

1. Create the shared counter: `let count = Arc::new(AtomicUsize::new(0));` and an empty vector `handles`.
2. Loop with `for word in words`. In the loop:
   - clone the `Arc`: `let count = Arc::clone(&count);`
   - spawn a thread with a `move` closure: if `word.len() >= min_len`, call `count.fetch_add(1, Ordering::Relaxed);`
   - push the handle into `handles`
3. After the loop, join every handle.
4. Return `count.load(Ordering::Relaxed)`.

```
collect_squares(4) → [1, 4, 9, 16]
collect_squares(0) → []

count_long_words(vec!["a", "bbb", "cccc", "dd"], 3) → 2
count_long_words(vec![], 3)                         → 0
```

## Expected Result

- `collect_squares` returns the squares of `1..=workers` in ascending order.
- `count_long_words` returns the number of words with at least `min_len` bytes.
- `main` prints `[1, 4, 9, 16, 25, 36, 49, 64]` and `3 long words`.
