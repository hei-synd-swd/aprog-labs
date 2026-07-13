# Borrowing Rules

Multiple parts of your code can **read** the same data simultaneously
through immutable references (`&T`):

```rust
let text = String::from("hello world");
let r1 = &text;
let r2 = &text;       // ok — multiple readers
println!("{r1} and {r2}");
```

## Mutable borrows are exclusive

A mutable reference (`&mut T`) is the **only** way to change data through
a borrow. While it exists, no other references to the same data are allowed:

```rust
let mut s = String::from("hi");
let r = &mut s;
r.push('!');          // ok — exclusive access
// let r2 = &s;       // compile error — already mutably borrowed
```

## Scope matters

A borrow lasts from its creation until its **last use**. After that, other
borrows can be made:

```rust
let mut s = String::from("hello");
let r1 = &s;          // immutable borrow
println!("{r1}");     // last use of r1
let r2 = &mut s;      // now ok — r1 is done
r2.push_str(" world");
```
