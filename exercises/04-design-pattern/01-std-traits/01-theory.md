# Standard Library Traits

Much of what a type can do comes from **traits of the standard library**. Some are written by the compiler for you, others you implement.

## Derived traits

One attribute line asks the compiler to generate the implementations:

```rust
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
struct Meters(f64);
```

- `Debug` enables printing with `{:?}`.
- `Clone` and `Copy` enable `.clone()` and silent copies instead of moves.
- `PartialEq` enables `==`, `PartialOrd` enables `<` and `>`.
- `Default` enables `Meters::default()`, with every field at its default (0, `false`, empty).

## Operators are traits

`a - b` is a call to the method `sub` of the trait `Sub` from `std::ops`. Implementing the trait makes the operator work for your type:

```rust
impl Sub for Meters {
    type Output = Meters; // the type of `a - b`

    fn sub(self, other: Meters) -> Meters {
        Meters(self.0 - other.0)
    }
}

let rest = Meters(10.0) - Meters(2.5); // Meters(7.5)
```

The same pattern holds for `+` (`Add`), `*` (`Mul`) and `/` (`Div`).
