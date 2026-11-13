# Generic Code Across Threads

A generic function that spawns threads has to promise the compiler that its type parameters are safe to hand to another thread:

```rust
fn in_thread<T, R>(value: T, f: fn(T) -> R) -> R
where
    T: Send + 'static,
    R: Send + 'static,
{
    thread::spawn(move || f(value))
        .join()
        .expect("worker does not panic")
}

let length = in_thread(String::from("Ferris"), |name| name.len()); // 6
```

## The bounds

- **`Send`** - a value of this type may be **moved** to another thread. Most types are `Send`, `Rc<T>` is not.
- **`'static`** - the type holds no borrowed data that could disappear while the thread still runs. Owned types such as `String` and `Vec<u64>` qualify.
- `T` needs both because the value travels **into** the thread, `R` because the result travels **back** through `join`.
- `fn(T) -> R` is a **function pointer**. It is `Copy`, so every thread can get its own copy. A closure that captures nothing converts to one automatically.

## Taking owned pieces from a vector

`chunks()` only lends slices. To **move** items out in groups, without cloning them, pull them from an owning iterator:

```rust
let mut items = vec!['a', 'b', 'c'].into_iter();

let first: Vec<char> = items.by_ref().take(2).collect();  // ['a', 'b']
let second: Vec<char> = items.by_ref().take(2).collect(); // ['c']
let third: Vec<char> = items.by_ref().take(2).collect();  // [] - nothing left
```

`by_ref()` borrows the iterator, so `take` does not consume it and the next call continues where the last one stopped.
