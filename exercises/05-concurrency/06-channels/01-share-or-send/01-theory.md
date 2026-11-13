# Share Memory or Pass Messages

This is a **text exercise**. Write your answer in `main.md`, below the `<!-- Write your answer below -->` line. LangQuest checks it for the key concepts.

## What you need to know

- Threads have **two ways** to coordinate and hand data to each other. One keeps a single value that all threads access, the other moves values from thread to thread.
- The two ways carry different risks. One of them avoids data races and most deadlocks by construction.
- Sending a value through a channel changes who **owns** it.
- Channels come in several kinds. They differ in how many senders and receivers they allow and in how many values travel through them.
- Channels connect threads inside **one process**. Separate processes need mechanisms provided by the operating system.

> "Don't communicate by sharing memory; share memory by communicating."

## Where to look

The answers are on these slides of the Concurrency chapter:

- *What is inter-thread communication?*
- *Ways to pass messages*
- *The workhorse: an `mpsc` channel*
