# Exponentiation

Raising a number to a power means multiplying the base by itself `exp` times: `2^4 = 2 × 2 × 2 × 2 = 16`, and `base^0 = 1` for any base.

This has a natural **recursive** shape - one factor of `base` times a smaller power:

- Base case: `base^0 = 1`
- Recursive case: `base^exp = base × base^(exp - 1)`

Each call peels off one factor and recurses on `exp - 1`, until `exp` reaches `0` and the base case stops the recursion.
