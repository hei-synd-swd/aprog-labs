# Encapsulation

**Encapsulation** bundles data with the operations that act on it and *hides
the internal representation* from outside code. This is the first pillar of OOP.

## Private Fields, Public Methods

In Rust, fields are **private by default** — they can only be accessed within
the module where the struct is defined. You expose a controlled public API
through methods:

```rust
mod library {
    pub struct Book {
        title: String,   // private — hidden outside `mod library`
        pages: u32,
    }

    impl Book {
        pub fn new(title: &str, pages: u32) -> Self {
            Self {
                title: title.into(),
                pages,
            }
        }

        pub fn pages(&self) -> u32 {
            self.pages
        }
    }
}
```

## Why Encapsulation?

- **Data protection** — outside code cannot corrupt internal state directly.
- **Invariants** — the type guarantees its own rules.
- **Flexibility** — you can change the internal implementation without
  breaking callers, as long as the public API stays the same.

## The `pub` Keyword

- `pub struct Foo` — the *type* is public, but its fields stay private.
- `pub fn foo()` — the function is public.
- `pub fn field(&self) -> &Type` — a *getter* exposes read access.
- `pub fn set_field(&mut self, val: Type)` — a *setter* controls mutation.
