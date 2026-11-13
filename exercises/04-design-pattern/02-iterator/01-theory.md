# The Iterator Trait

Any type can become an iterator. It has to implement the trait `Iterator`, which asks for one type and one method:

```rust
struct Evens {
    current: u32,
    limit: u32,
}

impl Iterator for Evens {
    type Item = u32; // what the iterator yields

    fn next(&mut self) -> Option<u32> {
        if self.current > self.limit {
            return None; // finished
        }
        let value = self.current;
        self.current += 2; // remember where to continue
        Some(value)
    }
}

let big: Vec<u32> = Evens { current: 0, limit: 6 }
    .filter(|&value| value > 2)
    .collect(); // [4, 6]
```

- `type Item` is the type of the values.
- `next` returns `Some(value)` for the next value and `None` when there are no more. It takes `&mut self`, because the iterator keeps its position in its fields.
- With `next` in place, the whole toolbox works on your type: `for` loops, and adaptors such as `map`, `filter` and `take`.
- Iterators are **lazy**: nothing is computed until a consumer such as `collect`, `sum` or a loop asks for values.
