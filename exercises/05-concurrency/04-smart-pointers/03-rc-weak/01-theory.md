# `Rc<T>` and `Weak<T>`

`Rc<T>` gives a value **several owners** on a single thread. It counts the owners and frees the value when the last one is dropped. A `Weak<T>` refers to the same value **without owning** it.

```rust
use std::rc::{Rc, Weak};

fn main() {
    let first = Rc::new(5);
    let second = Rc::clone(&first);              // another owner
    let watcher: Weak<i32> = Rc::downgrade(&first); // not an owner

    println!("{}", Rc::strong_count(&first));    // 2
    println!("{}", Rc::weak_count(&first));      // 1
    println!("{:?}", watcher.upgrade());         // Some(5)

    drop(first);
    drop(second);                                // last owner gone
    println!("{:?}", watcher.upgrade());         // None
}
```

## Strong: owners

- `Rc::clone(&rc)` creates another owner of the **same** value. Nothing is copied, only the strong count goes up by one.
- The value is dropped when the **strong count** reaches 0.
- An `Rc` only hands out shared access (`&T`), the value cannot be changed through it.

## Weak: observers

- `Rc::downgrade(&rc)` creates a `Weak`. It raises the weak count and does **not** keep the value alive.
- A `Weak` cannot be read directly. `weak.upgrade()` returns an `Option<Rc<T>>`: `Some(rc)` while the value is alive, `None` once it is gone. Freed memory can therefore never be read.
- `Weak` breaks **reference cycles**. A child that points back to its parent with a strong `Rc` would keep the parent alive forever.

`Rc` is for a single thread. Its thread-safe twin is `Arc`, with the same methods.
