# Strategy Pattern

The Strategy pattern defines a family of algorithms, encapsulates each one, and makes them interchangeable. In Rust, this is often done with **trait objects** (`Box<dyn Trait>`).

```rust
trait TaxStrategy {
    fn calculate_tax(&self, amount: f64) -> f64;
}

struct NoTax;
impl TaxStrategy for NoTax {
    fn calculate_tax(&self, amount: f64) -> f64 { 0.0 }
}

struct FlatTax(f64);
impl TaxStrategy for FlatTax {
    fn calculate_tax(&self, amount: f64) -> f64 { amount * self.0 }
}

struct Order {
    total: f64,
    tax_strategy: Box<dyn TaxStrategy>,
}

impl Order {
    fn tax(&self) -> f64 {
        self.tax_strategy.calculate_tax(self.total)
    }
}
```

The `Order` doesn't know *which* tax rule is applied, it just calls `calculate_tax`.
