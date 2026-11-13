# The Asynchronous Execution Model

This is a **text exercise**. Write your answer in `main.md`, below the `<!-- Write your answer below -->` line. LangQuest checks it for the key concepts.

## What you need to know

- Async is **not** threads. It is a separate model: many cheap **tasks** driven by a **runtime**, instead of threads scheduled by the OS.
- An `async fn` does not run when it is called. What it returns, and when its body actually executes, is the first thing to understand.
- OS threads and async tasks give up the CPU in different ways. One model can be interrupted at any time, the other one only hands over control at certain points in the code.
- Because of that, one careless call inside an `async fn` can freeze every other task on the same thread.
- The runtime consists of a few parts that **poll** futures until they are done and get notified when a waiting task can continue.
- Threads and async are good at different kinds of work: computing versus waiting.

## Where to look

The answers are on these slides of the Concurrency chapter:

- *A different execution model*
- *`async` / `.await`*
- *Cooperative scheduling*
- *Under the hood: a `Future` is lazy*
- *Under the hood: the async runtime*
- *Threads vs. async - which one?*
