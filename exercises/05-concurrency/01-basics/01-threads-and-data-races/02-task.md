---
id          = "conc_threads_data_races"
name        = "Threads and Data Races"
language    = "text"
difficulty  = 1
description = "Explain program, process and thread, what threads share, and what a data race is."
topics      = ["process", "thread", "data race", "send", "sync"]
---

# Threads and Data Races

Pin down the vocabulary of this chapter.

## Your Task

Open `main.md` and write your answers **below** the `<!-- Write your answer below -->` line. Answer all four questions in your own words, in full sentences, and use the technical terms from the slides.

1. What is the difference between a **program**, a **process** and a **thread**? Write one sentence per term. Say where a program lives, what a process is and what it owns, and what a thread is for the OS.
2. Which parts of a process are **shared** by all its threads, and which are **private** to each thread? Sort these five into the two groups: code, heap, open files, stack, registers.
3. Which **four conditions** must all hold for a data race to occur? Cover the memory location, the kind of access, the missing coordination between the threads and the timing.
4. How does Rust prevent data races **at compile time**? Name the borrowing rule (how many readers, how many writers) and the two marker traits, with what each trait allows.

## Expected Result

Your answer tells the three terms apart, sorts the five memory areas, lists all four data race conditions and names the Rust mechanisms. LangQuest checks your answer for 10 key concepts and you pass once at least 8 of them are covered (75%).
