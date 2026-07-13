# Collections of Trait Objects

A `Vec<Box<dyn Trait>>` lets you mix different types in one collection:

```rust
let mut animals: Vec<Box<dyn Animal>> = Vec::new();
animals.push(Box::new(Dog { name: "Rex".into() }));
animals.push(Box::new(Cat { name: "Luna".into() }));

let noises: String = animals.iter().map(|a| a.speak()).collect::<Vec<_>>().join(", ");
```

Each `.speak()` dispatches to the right implementation via the vtable — Dog's or Cat's — depending on what the box actually contains.

This is how classic OOP does heterogeneous collections. In Java it's `List<Animal>`; in Rust it's `Vec<Box<dyn Animal>>`.
