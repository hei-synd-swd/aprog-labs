# Concurrency vs. Parallelism

This is a **text exercise**. Write your answer in `main.md`, below the `<!-- Write your answer below -->` line. LangQuest checks it for the key concepts.

## What you need to know

- **Concurrency** and **parallelism** are not the same thing. One describes how a program is *structured*, the other how it is *executed*.
- A single core can only run one thread at a time, yet many tasks can still make progress on it.
- A request can be handled **synchronously** or **asynchronously**. The difference is what the caller does while the answer is on its way.
- The OS scheduler moves every thread between three **states** and decides who gets the CPU. A thread can lose the CPU because it is forced to, or because it has to wait for something.

## Where to look

The answers are on these slides of the Concurrency chapter:

- *Concurrency ≠ Parallelism*
- *Synchronous vs. asynchronous*
- *Threads & the OS scheduler*
- *Preemptive scheduling in action*
