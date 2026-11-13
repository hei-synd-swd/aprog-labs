---
id          = "conc_choose_pointer"
name        = "Choose the Smart Pointer"
language    = "text"
difficulty  = 1
description = "Pick the right smart pointer for five situations and justify the choice."
topics      = ["smart pointer", "box", "rc", "arc", "mutex", "weak"]
---

# Choose the Smart Pointer

Five situations, five smart pointers.

## Your Task

Open `main.md` and write your answers **below** the `<!-- Write your answer below -->` line. For **each** situation, name the smart pointer type (for example `Box<T>`) and give the reason in one sentence. The reason must say how many owners there are and whether threads are involved.

1. A binary tree: every node contains up to two child nodes of the same type. There is one owner per node.
2. A read-only configuration that several parts of a **single-threaded** program own together.
3. The same read-only configuration, now owned by **several threads**.
4. A hit counter that **several threads** own together and all **increment**.
5. A child node that must refer to its parent, but must **not** keep the parent alive.

## Expected Result

Your answer names `Box`, `Rc`, `Arc`, `Arc<Mutex<...>>` and `Weak` for the five situations, each with a reason. LangQuest checks your answer for 8 key concepts and you pass once at least 6 of them are covered (75%).
