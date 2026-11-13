---
id          = "conc_async_concepts"
name        = "Async Concepts"
language    = "text"
difficulty  = 2
description = "Explain lazy futures, cooperative scheduling, the runtime, and when to use threads or async."
topics      = ["async", "future", "cooperative scheduling", "runtime", "tokio"]
---

# Async Concepts

Explain how asynchronous code is executed and when it is the right tool.

## Your Task

Open `main.md` and write your answers **below** the `<!-- Write your answer below -->` line. Answer all five questions in your own words, in full sentences, and use the technical terms from the slides.

1. What does **calling** an `async fn` return, and when does its body run? Use the word that describes this "nothing happens yet" behaviour.
2. What is the difference between **preemptive** and **cooperative** scheduling? Say who decides when a thread loses the CPU, and at which point in the code an async task hands over control.
3. Why must an `async fn` never call `std::thread::sleep` or run a long computation? Say what happens to the other tasks on the same thread, and name what to use instead for a pause and for heavy CPU work.
4. Which part of the **runtime** polls the futures, and which handle tells it that a parked task can be polled again?
5. When do you choose **threads or Rayon**, and when **async with Tokio**? Give the kind of work for each.

## Expected Result

Your answer explains lazy futures, contrasts preemptive with cooperative scheduling, explains why blocking is forbidden in async code and what replaces it, names the executor and the waker, and assigns CPU-bound and I/O-bound work to the right model. LangQuest checks your answer for 10 key concepts and you pass once at least 8 of them are covered (75%).
