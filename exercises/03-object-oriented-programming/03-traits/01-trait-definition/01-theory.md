# Traits

A **trait** defines a set of method signatures that types can implement. It's
Rust's way of defining shared behaviour — similar to an *interface* in Java or
Go.

## Defining a Trait

Use the `trait` keyword followed by the method signatures:

```rust
trait Area {
    fn area(&self) -> f64;
}
```

## Implementing a Trait

Use `impl TraitName for Type`:

```rust
struct Square { side: f64 }

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}
```

Now any `Square` has the `.area()` method.

## Calling Trait Methods

You call them just like regular methods — with dot notation:

```rust
let s = Square { side: 4.0 };
println!("{}", s.area()); // 16
```

## Key Points

- The trait only declares the method signature — no implementation.
- The `impl Trait for Type` block provides the implementation.
- A type can have many traits implemented on it.
- The method must have the exact same signature as the trait declaration.
