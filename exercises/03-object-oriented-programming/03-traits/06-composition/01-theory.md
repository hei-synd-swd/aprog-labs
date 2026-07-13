# Composition over Inheritance

Rust **does not have class inheritance** (`struct Dog : Animal` is not valid).
Instead, Rust encourages **composition** — building types by combining smaller
parts.

## Has-a Relationships

Instead of "a Computer **is a** Machine" (inheritance), say "a Computer
**has a** Processor" (composition):

```rust
struct Processor { cores: u32, speed_ghz: f64 }

struct Computer {
    processor: Processor,   // has-a
    ram_gb: u32,
}
```

## Delegation

The outer type forwards calls to the inner field:

```rust
impl Processor {
    fn boot(&self) -> String {
        format!("{} cores at {} GHz", self.cores, self.speed_ghz)
    }
}

impl Computer {
    fn boot(&self) -> String {
        self.processor.boot()   // delegation
    }
}
```
