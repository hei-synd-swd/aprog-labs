# Trait Objects — `dyn Trait`

A **trait object** lets you store a value of *any* type that implements a trait behind a pointer. Method dispatch happens at **runtime** via a **vtable**.

```rust
trait Animal { fn speak(&self) -> String; }
struct Dog { name: String }
struct Cat { name: String }
impl Animal for Dog { fn speak(&self) -> String { format!("{} says woof", self.name) } }
impl Animal for Cat { fn speak(&self) -> String { format!("{} says meow", self.name) } }

// &dyn Animal — borrow, no heap allocation
let dog = Dog { name: "Rex".into() };
let a: &dyn Animal = &dog;

// Box<dyn Animal> — owned, heap-allocated
let cat = Cat { name: "Luna".into() };
let b: Box<dyn Animal> = Box::new(cat);
```

`&dyn Animal` is a **fat pointer** (data ptr + vtable ptr). `Box<dyn Animal>` owns the data on the heap.

| Form | Ownership | Use case |
|---|---|---|
| `&dyn Trait` | Borrow | Parameters, temporary access |
| `Box<dyn Trait>` | Owned | Storing in collections, returning |
