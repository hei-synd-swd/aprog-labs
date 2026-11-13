---
id          = "conc_join_all"
name        = "Join All Threads"
language    = "rust"
difficulty  = 3
description = "Spawn one thread per input and collect the results in input order."
topics      = ["threads", "spawn", "join", "JoinHandle", "vec"]
---

# Join All Threads

Analyse several texts at the same time, one thread per text.

The function `analyze(text)` is given. It returns a `Report` with the number of vowels in the text and the id of the thread that counted them.

## Your Task

Replace the `todo!()` in `analyze_all(texts: Vec<String>) -> Vec<Report>`. Do not change `Report`, `analyze`, the signature, `main` or the tests.

1. Create an empty vector for the handles: `let mut handles = Vec::new();`.
2. Loop over the texts with `for text in texts`. For every text, spawn a thread with a `move` closure that calls `analyze(&text)`, and push the returned handle: `handles.push(thread::spawn(move || analyze(&text)));`.
3. **After** that loop, create an empty vector for the results: `let mut reports = Vec::new();`.
4. Loop over the handles with `for handle in handles`. Join each one with `handle.join().expect("the worker does not panic")` and push the returned `Report` into `reports`.
5. Return `reports`.

Do not call `join` inside the first loop. All threads must be started before the first one is joined.

```
analyze_all(vec!["aaa", "b", "ee"])   → reports with 3, 0 and 2 vowels, from three different threads
analyze_all(vec![])                   → an empty vector
```

## Expected Result

- The result has one `Report` per text, in the **same order** as the input.
- Every text is analysed by its **own** thread: all `worker` ids differ from each other and from the id of the calling thread.
- `main` prints the vowel count and the worker id for three texts.
