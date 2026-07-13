# Digit Manipulation

To extract digits from a number:
- `n % 10` gives the last digit (in base 10)
- `n / 10` removes the last digit

Loop until `n` becomes 0:
```rust
while n > 0 {
    let digit = n % 10;
    // use digit
    n /= 10;
}
```
