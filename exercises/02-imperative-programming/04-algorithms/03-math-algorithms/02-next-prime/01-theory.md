# Next Prime

The *next prime* of `n` is the smallest prime greater than or equal to `n`. There is no formula for it - you **search**: test `n`, then `n + 1`, `n + 2`, ... and stop at the first value that is prime.

This reuses a primality test (`is_prime`) as a building block inside a loop:

- Start a candidate at `n`.
- While the candidate is not prime, increment it by one.
- Return the first candidate that passes `is_prime`.

Primes are common enough that this loop stops after only a few steps for reasonable values of `n`.
