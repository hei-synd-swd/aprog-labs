# Getters & Setters

When a struct has private fields, you often provide **getter** and **setter**
methods to let callers read and write field values in a controlled way.

## Getter Pattern

In Rust, the idiomatic getter is named **after the field** — no `get_` prefix:

```rust
struct Player {
    level: u32,
}

impl Player {
    pub fn level(&self) -> u32 {
        self.level
    }
}
```

## Setter Pattern

A setter typically uses `set_` prefix and takes `&mut self`. This is where you
**validate** the new value before applying it:

```rust
impl Player {
    pub fn set_level(&mut self, value: u32) {
        if value <= 100 {
            self.level = value;
        }
    }
}
```
