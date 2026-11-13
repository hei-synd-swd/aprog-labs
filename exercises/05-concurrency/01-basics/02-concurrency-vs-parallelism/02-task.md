---
id          = "conc_concurrency_vs_parallelism"
name        = "Concurrency vs. Parallelism"
language    = "text"
difficulty  = 2
description = "Tell concurrency from parallelism, synchronous from asynchronous, and name the thread states."
topics      = ["concurrency", "parallelism", "synchronous", "asynchronous", "scheduler"]
---

# Concurrency vs. Parallelism

Explain how tasks make progress together and how the OS schedules them.

## Your Task

Open `main.md` and write your answers **below** the `<!-- Write your answer below -->` line. Answer all four questions in your own words, in full sentences, and use the technical terms from the slides.

1. What is the difference between **concurrency** and **parallelism**? Write one sentence per term. Say which one is about the structure of a program and which one is about tasks running at the same instant, and which one needs multiple cores.
2. How can a program be concurrent on a **single core**? Describe what the tasks do with the one core they have.
3. What is the difference between a **synchronous** and an **asynchronous** request? Say what the caller does while the answer is not there yet, in both cases.
4. Name the **three states** of a thread. Then name the two ways a running thread gives up the CPU: the forced one and the voluntary one, and the state it ends up in each time.

## Expected Result

Your answer tells concurrency from parallelism, explains interleaving on one core, contrasts synchronous with asynchronous and names the three thread states with both ways of leaving the CPU. LangQuest checks your answer for 10 key concepts and you pass once at least 8 of them are covered (75%).
