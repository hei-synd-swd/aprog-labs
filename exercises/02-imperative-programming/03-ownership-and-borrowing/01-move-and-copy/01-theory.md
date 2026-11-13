# Ownership: Move and Copy

Rust has no garbage collector. It frees memory through **ownership**, with three rules:

1. Every value has exactly **one owner**.
2. There is only one owner at a time.
3. When the owner goes out of scope, the value is **dropped**.

```rust
fn main() {
    let a = String::from("Ferris");
    let b = a;           // the String MOVES from a to b
    // println!("{a}");  // error[E0382]: borrow of moved value: `a`
    let c = b.clone();   // a second, independent String

    let x = 5;
    let y = x;           // an i32 is COPIED, x stays usable

    println!("{b} {c} {x} {y}");
}
```

## Move

- Assigning a `String` or a `Vec` to another variable **moves** it. Only the pointer changes hands, the data on the heap is not duplicated.
- The old variable can no longer be used. That is what prevents two owners from freeing the same memory.
- `value.clone()` creates an independent duplicate with its own heap data. Both can then be used.

## Copy

- Simple values that live entirely on the stack are **copied** instead of moved: integers, floats, `bool`, `char`, and tuples of such types.
- After a copy, both variables stay usable. No `clone()` is needed.

## Functions

- Passing a value to a function works like an assignment: a `String` argument **moves into** the function, an `i32` argument is copied.
- A `return` value moves **out** to the caller.
- A parameter that is not returned is dropped when the function ends.
