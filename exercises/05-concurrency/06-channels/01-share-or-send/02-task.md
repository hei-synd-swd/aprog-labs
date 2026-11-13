---
id          = "conc_share_or_send"
name        = "Share or Send"
language    = "text"
difficulty  = 1
description = "Compare shared memory with message passing and name the channel kinds and OS-level IPC mechanisms."
topics      = ["channel", "message passing", "shared memory", "mpsc", "ipc"]
---

# Share or Send

Describe the ways threads and processes exchange data.

## Your Task

Open `main.md` and write your answers **below** the `<!-- Write your answer below -->` line. Answer all five questions in your own words, in full sentences, and use the technical terms from the slides.

1. What are the **two ways** for threads to exchange data? Describe each in one sentence and name the Rust tool it uses.
2. What happens to the **ownership** of a value when it is sent through a channel? Say who can use the value afterwards.
3. Which two problems of shared memory does message passing **avoid**?
4. Name the **three kinds of channels** from the slides and say what each one is for (how many senders, how many receivers, how many values).
5. Channels only work inside one process. Name at least **two OS-level mechanisms** that connect separate processes.

## Expected Result

Your answer contrasts shared memory behind a lock with message passing through a channel, explains that a sent value moves to the receiver, names data races and deadlocks, the channel kinds `mpsc`, `oneshot` and `broadcast`, and OS-level mechanisms such as pipes and sockets. LangQuest checks your answer for 10 key concepts and you pass once at least 8 of them are covered (75%).
