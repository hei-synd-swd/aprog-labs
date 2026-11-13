---
id          = "conc_multi_producer"
name        = "Multiple Producers"
language    = "rust"
difficulty  = 3
description = "Let several threads send into one mpsc channel and receive until the channel is closed."
topics      = ["channel", "mpsc", "clone", "drop", "producer", "consumer"]
---

# Multiple Producers

Several worker threads produce numbers and send them into **one** channel. The calling thread receives all of them, until every worker is done.

Worker `w` sends `per_worker` numbers: `w * 100`, `w * 100 + 1`, `w * 100 + 2`, and so on. So worker 0 sends 0, 1, 2, ... and worker 2 sends 200, 201, 202, ...

## Your Task

Replace the `todo!()` in `gather(workers: u32, per_worker: u32) -> Vec<u32>`. Do not change the signature, `main` or the tests.

1. Create a channel: `let (tx, rx) = mpsc::channel();`.
2. Loop with `for worker in 0..workers`. In the loop:
   - clone the sender for this thread: `let tx = tx.clone();`
   - spawn a thread with a `move` closure
   - inside the closure, loop with `for i in 0..per_worker` and send each number: `tx.send(worker * 100 + i).expect("the receiver is still alive");`
3. **After** the loop, drop the original sender: `drop(tx);`. Without this line the channel never closes and step 4 waits forever.
4. Receive everything until the channel is closed: `let mut numbers: Vec<u32> = rx.iter().collect();`.
5. The numbers of different workers arrive in any order. Sort them with `numbers.sort();` and return `numbers`.

You do not need to join the threads. The receiving loop ends exactly when the last worker has dropped its sender.

```
gather(3, 2) → [0, 1, 100, 101, 200, 201]
gather(1, 3) → [0, 1, 2]
gather(0, 5) → []      no worker, the channel is closed by `drop(tx)` alone
gather(4, 0) → []      four workers that send nothing
```

## Expected Result

- The result contains every number of every worker exactly once, in ascending order.
- The function returns for any input, also for zero workers. It never waits forever.
- `main` prints `[0, 1, 100, 101, 200, 201]`.
