# Prime Numbers

A prime is a natural number greater than 1 that has no positive divisors
other than 1 and itself.

To check if `n` is prime, test if any number `d` from 2 to `√n` divides
`n` evenly (`n % d == 0`). If none do, `n` is prime.

You only need to check up to `√n` because if `n = a × b`, at least one
factor is ≤ √n.
