---
id          = "conc_chunked_sum"
name        = "Chunked Sum"
language    = "rust"
difficulty  = 4
description = "Split a vector into chunks and sum every chunk in its own thread."
topics      = ["threads", "chunks", "spawn", "join", "ownership"]
---

# Chunked Sum

Sum a large vector with several threads: cut it into chunks, sum each chunk in its own thread and return the partial sums.

## Your Task

Replace the `todo!()` in `chunk_sums(numbers: Vec<u64>, workers: usize) -> Vec<u64>`. Do not change the signature, `main` or the tests. `workers` is at least 1, you do not have to check this.

1. If `numbers` is empty, return an empty vector right away: `return Vec::new();`.
2. Compute the chunk size, rounded up: `let chunk_size = numbers.len().div_ceil(workers);`.
3. Create an empty vector `handles`.
4. Loop over `numbers.chunks(chunk_size)`. For every chunk:
   - make an owned copy: `let chunk = chunk.to_vec();`
   - spawn a thread with a `move` closure that returns the sum of the chunk: `chunk.iter().sum::<u64>()`
   - push the handle into `handles`
5. After the loop, join all handles **in order** with `handle.join().expect("the worker does not panic")` and collect the returned sums into a `Vec<u64>`.
6. Return that vector.

```
chunk_sums((1..=10).collect(), 3) → [10, 26, 19]    chunk size 4: [1,2,3,4] [5,6,7,8] [9,10]
chunk_sums((1..=10).collect(), 1) → [55]            one chunk with everything
chunk_sums(vec![1, 2, 3, 4], 8)   → [1, 2, 3, 4]    chunk size 1, never more chunks than items
chunk_sums(vec![], 4)             → []
```

## Expected Result

- The result holds one partial sum per chunk, in the order of the chunks.
- There are at most `workers` chunks, and all of them except the last have `chunk_size` items.
- The partial sums add up to the sum of all numbers.
- `main` prints the four partial sums of the numbers 1 to 1 000 000 and their total, 500000500000.
