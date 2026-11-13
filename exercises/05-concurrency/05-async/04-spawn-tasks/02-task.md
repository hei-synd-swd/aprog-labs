---
id          = "conc_spawn_tasks"
name        = "Spawn Tasks"
language    = "rust"
difficulty  = 4
description = "Remove a blocking call from an async fn and run many downloads as concurrent tasks."
topics      = ["async", "tokio", "spawn", "JoinHandle", "blocking"]
---

# Spawn Tasks

A download manager fetches several files at once. `download(id)` simulates one download: it takes 100 ms and returns the file name `file-<id>`. `download_all(ids)` should run all downloads **concurrently**, one task per file.

The file contains a bug and a missing function.

## Your Task

Fix `download` and implement `download_all`. Do not change the signatures, `main`, the tests or `Cargo.toml`.

**Part 1 - `download(id: u32) -> String` blocks.** It waits with `std::thread::sleep`, which freezes the whole thread and with it every other task.

1. Replace the line `std::thread::sleep(Duration::from_millis(100));` with the non-blocking version: `tokio::time::sleep(Duration::from_millis(100)).await;`.
2. Keep the last line `format!("file-{id}")`.

**Part 2 - `download_all(ids: Vec<u32>) -> Vec<String>`** starts one task per id and collects the file names.

1. Create an empty vector `handles`.
2. Loop with `for id in ids`. Spawn a task for each id and push its handle: `handles.push(tokio::spawn(download(id)));`. Do not write `.await` here.
3. **After** that loop, create an empty vector `files`.
4. Loop with `for handle in handles`. Await each handle and push the file name: `files.push(handle.await.expect("the task does not panic"));`.
5. Return `files`.

```
download(7).await                      → "file-7" after 100 ms
download_all(vec![3, 1, 2]).await      → ["file-3", "file-1", "file-2"] after 100 ms, not 300 ms
download_all(vec![]).await             → []
```

## Expected Result

- `download` waits 100 ms **without blocking** the thread.
- `download_all` returns one file name per id, in the **same order** as the input.
- All downloads run concurrently: five downloads take about 100 ms in total, not 500 ms.
- `main` prints the five file names and an elapsed time of about 100 ms.

> The first build downloads Tokio from crates.io, so a working internet connection is required.
