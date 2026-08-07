//! Prime testing.
//!
//! This module is pre-implemented for you. It provides the CPU-bound work
//! (`is_prime`) that the counting functions in `counting.rs` build on.

/// Returns `true` if `n` is a prime number.
///
/// Uses simple trial division. It is intentionally CPU-bound so that splitting
/// the work across several threads is actually worthwhile.
pub fn is_prime(n: u64) -> bool {
  if n < 2 {
    return false;
  }
  if n % 2 == 0 {
    return n == 2;
  }
  let mut divisor = 3;
  while divisor * divisor <= n {
    if n % divisor == 0 {
      return false;
    }
    divisor += 2;
  }
  true
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_prime_basics() {
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(!is_prime(4));
    assert!(is_prime(97));
    assert!(!is_prime(100));
  }
}
