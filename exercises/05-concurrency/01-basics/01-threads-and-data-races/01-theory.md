# Threads and Data Races

This is a **text exercise**. Write your answer in `main.md`, below the `<!-- Write your answer below -->` line. LangQuest checks it for the key concepts.

## What you need to know

- **Program, process, thread** are three different things: code that is only stored, that code being executed, and the paths of execution inside it.
- All threads of a process live in the **same memory**. Some areas belong to all threads, others exist once per thread.
- Sharing memory makes threads fast to communicate and dangerous: uncoordinated access to the same data is a **data race**, and its result is undefined.
- Rust rejects data races **at compile time**. It uses the borrowing rules you already know plus two marker traits that say what may cross a thread boundary.

## Where to look

The answers are on these slides of the Concurrency chapter:

- *Program, Process, Thread*
- *What threads share*
- *The hazard: data races*
