# Splitting Work into Chunks

One thread per item is wasteful when there are a million items. The usual pattern is to cut the data into a few **chunks** and give each chunk to one thread.

```rust
use std::thread;

fn main() {
    let words = vec!["a", "bb", "ccc", "dddd", "eeeee"];
    let chunk_size = words.len().div_ceil(2); // 5 items, 2 workers -> 3

    let mut handles = Vec::new();
    for chunk in words.chunks(chunk_size) {
        let chunk = chunk.to_vec(); // an owned copy for the thread
        handles.push(thread::spawn(move || chunk.len()));
    }

    for handle in handles {
        println!("{}", handle.join().expect("worker does not panic")); // 3, then 2
    }
}
```

## Cutting a vector

- `slice.chunks(n)` yields consecutive slices of `n` items. The last one may be shorter.
- `chunks(0)` panics. An empty input has to be handled before.
- `a.div_ceil(b)` divides and rounds **up**: `10usize.div_ceil(3)` is 4. With that chunk size, 10 items give at most 3 chunks.

## Giving a chunk to a thread

- A chunk is a **borrowed** slice (`&[T]`). A spawned thread cannot borrow from the vector, because it may outlive the function.
- `chunk.to_vec()` copies the chunk into a new `Vec` that the thread can **own**.
- Inside the thread, `chunk.iter().sum::<u64>()` adds up the items. The `::<u64>` tells `sum` which type to produce.
