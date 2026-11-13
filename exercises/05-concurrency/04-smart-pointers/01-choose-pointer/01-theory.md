# Choosing a Smart Pointer

This is a **text exercise**. Write your answer in `main.md`, below the `<!-- Write your answer below -->` line. LangQuest checks it for the key concepts.

## What you need to know

A **smart pointer** owns data and adds behaviour to it: heap allocation, reference counting or locking. Two questions pick the right one:

- **How many owners?** One owner, or several parts of the program that co-own the value.
- **How many threads?** Everything on one thread, or the value crosses a thread boundary.

| Need | Single thread | Multi-thread |
|---|---|---|
| heap, one owner | `Box<T>` | `Box<T>` |
| shared ownership, read only | `Rc<T>` | `Arc<T>` |
| shared ownership, mutable | | `Arc<Mutex<T>>` |

Two more facts:

- The thread-safe pointers cost a little more at run time. Pick the single-thread one unless the value really crosses threads.
- A `Weak<T>` is a non-owning handle to an `Rc` or `Arc` value. It does not keep the value alive.

## Where to look

- *Smart Pointers Overview*
- *Choosing a smart pointer*
- *`Rc`/`Arc`: strong vs. weak counts*
