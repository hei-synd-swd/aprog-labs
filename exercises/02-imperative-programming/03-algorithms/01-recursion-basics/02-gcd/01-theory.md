# Euclid's Algorithm

The greatest common divisor (GCD) of two numbers is the largest number
that divides both evenly. Euclid's algorithm is an elegant recursive
solution:

```
gcd(a, b) = gcd(b, a % b)  — keep replacing (a, b) with (b, a mod b)
gcd(a, 0) = a              — until b reaches 0
```

For example, `gcd(48, 18)`:
- `gcd(48, 18)` → `gcd(18, 48 % 18 = 12)`
- `gcd(18, 12)` → `gcd(12, 18 % 12 = 6)`
- `gcd(12, 6)`  → `gcd(6, 12 % 6 = 0)`
- `gcd(6, 0)`   → `6`
