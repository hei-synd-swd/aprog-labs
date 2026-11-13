# `Box<T>` and Recursive Types

`Box<T>` puts a value on the **heap** and owns it. The box itself is a pointer with a fixed size, and it frees the value when it goes out of scope.

A type that contains itself directly, such as `enum Doll { Outer(Doll), Innermost }`, would have an infinite size and is rejected by the compiler. With a `Box` in between, the outer doll only stores a pointer to the next one:

```rust
enum Doll {
    Outer(Box<Doll>),
    Innermost,
}

impl Doll {
    fn wrap(self) -> Doll {
        Doll::Outer(Box::new(self)) // move `self` to the heap
    }

    fn count(&self) -> u32 {
        match self {
            Doll::Outer(inner) => 1 + inner.count(),
            Doll::Innermost => 1,
        }
    }
}

fn main() {
    let dolls = Doll::Innermost.wrap().wrap();
    println!("{}", dolls.count()); // 3
}
```

- `Box::new(value)` moves `value` to the heap and returns the box.
- A box behaves like the value inside it: `inner.count()` calls the method of `Doll` directly.
- A recursive type is processed with a recursive method: one `match` arm handles the current element and calls the method on the rest, the other arm ends the recursion.
