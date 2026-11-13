# The Limits of Amdahl's Law

Amdahl's Law gives the speedup for a parallel fraction `f` on `p` processors:

```
S = 1 / ((1 - f) + f / p)
```

## The maximum speedup

With more and more processors the term `f / p` shrinks towards zero. What remains is the serial part, and it sets a hard limit:

```
S_max = 1 / (1 - f)
```

| Parallel fraction `f` | Serial fraction | Maximum speedup |
|---|---|---|
| 0.50 | 50 % | 2x |
| 0.90 | 10 % | 10x |
| 0.95 | 5 % | 20x |

The limit is only approached, never reached: a program with `f = 0.9` gets close to 10x, but no number of processors makes it exactly 10 times faster.

## A value that may not exist

A question such as "how many processors do I need for a speedup of 12?" has no answer if 12 is above the limit. Rust expresses this with `Option`:

```rust
fn first_even(numbers: &[u32]) -> Option<u32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n); // found one
        }
    }
    None // there is none
}
```

`Some(value)` carries a result, `None` says that there is no result.
