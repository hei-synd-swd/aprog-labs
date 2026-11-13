---
id          = "conc_parallel_map"
name        = "Parallel Map"
language    = "rust"
difficulty  = 5
description = "Write a generic, order-preserving map that runs on several threads."
topics      = ["threads", "generics", "send", "static", "function pointer", "chunks"]
---

# Parallel Map

Build your own small version of a parallel `map`: apply a function to every item of a vector, on several threads, and return the results in the original order. It has to work for **any** item type, including types that cannot be copied.

## Your Task

Complete `parallel_map<T, R>(items: Vec<T>, workers: usize, f: fn(T) -> R) -> Vec<R>`. Do not change the parameters, the return type, `main` or the tests. `workers` is at least 1, you do not have to check this.

**Step 1 - the bounds.** Add a `where` clause to the function so that `T` and `R` may cross a thread boundary: `T: Send + 'static` and `R: Send + 'static`.

**Step 2 - the chunk size.** Compute `items.len().div_ceil(workers)`. For an empty vector this is 0, so make it at least 1 with `.max(1)`.

**Step 3 - spawn one thread per chunk.**

1. Turn the vector into an owning iterator: `let mut items = items.into_iter();`.
2. In a `loop`, take the next chunk out of the iterator: `let chunk: Vec<T> = items.by_ref().take(chunk_size).collect();`.
3. If the chunk is empty, all items are handed out: `break`.
4. Otherwise spawn a thread with a `move` closure. Inside it, apply `f` to every item of the chunk and collect the results: `chunk.into_iter().map(f).collect::<Vec<R>>()`.
5. Push the handle into a vector `handles`.

**Step 4 - collect in order.** Join the handles in the order they were created. Each `join` returns a `Vec<R>`. Append all of them to one result vector (for example with `extend`) and return it.

```
parallel_map(vec![1, 2, 3, 4, 5], 2, |n| n * n)             → [1, 4, 9, 16, 25]
parallel_map(vec!["a".to_string(), "bcd".to_string()], 2,
             |word| word.len())                              → [1, 3]
parallel_map(Vec::<u32>::new(), 4, |n| n + 1)                → []
```

## Expected Result

- The result has one entry per item, in the **same order** as the input.
- The items are **moved** into the threads, never cloned, so the function works for `String` and other non-`Copy` types.
- There is one thread per chunk and at most `workers` threads: 8 items with 4 workers run on exactly 4 threads.
- `main` prints the lengths of four words: `[8, 11, 2, 4]`.
