//! Counting primes: the sequential baseline and the parallel (spawn / join)
//! version.
//!
//! `count_primes_sequential` is pre-given - it is the reference your parallel
//! version must match. `count_primes_parallel` is your task.

use crate::primes::is_prime;
#[allow(unused_imports)]
use std::thread;

/// Counts the prime numbers in `numbers` sequentially (single thread).
///
/// Pre-given: this is the baseline your parallel version must reproduce exactly.
pub fn count_primes_sequential(numbers: &[u64]) -> usize {
  numbers.iter().filter(|&&n| is_prime(n)).count()
}

/// Counts the prime numbers in `numbers` in parallel, using at most
/// `num_threads` worker threads.
///
/// The slice is split into contiguous chunks, one thread is spawned per chunk
/// to count the primes in it, and the partial counts returned by `join()` are
/// summed. The result must always equal `count_primes_sequential(numbers)`.
///
/// # Your task
///
/// Implement this function with `std::thread::spawn` and `JoinHandle::join`.
///
/// Steps:
///  1. Treat `num_threads == 0` as `1`, and return `0` for an empty slice
///     (a zero-sized chunk would panic).
///  2. Compute a `chunk_size`, e.g. `numbers.len().div_ceil(num_threads)`.
///  3. For each `chunk` of `numbers.chunks(chunk_size)`, spawn a thread that
///     counts the primes in that chunk. `spawn` needs `'static` data, so give
///     each thread an *owned* copy (`chunk.to_vec()`) and `move` it in.
///  4. Push every `JoinHandle` into a `Vec`, then `join()` each one and sum
///     the partial counts it returns.
///
/// Use `is_prime` (imported above) to test each number.
pub fn count_primes_parallel(numbers: &[u64], num_threads: usize) -> usize {
  let _ = (numbers, num_threads);
  todo!("count the primes in parallel using std::thread spawn/join")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sequential_count() {
    // Primes up to 20: 2, 3, 5, 7, 11, 13, 17, 19 = 8
    let nums: Vec<u64> = (0..=20).collect();
    assert_eq!(count_primes_sequential(&nums), 8);
  }

  #[test]
  fn test_01_small_known() {
    let nums: Vec<u64> = (0..=20).collect();
    assert_eq!(count_primes_parallel(&nums, 4), 8);
  }

  #[test]
  fn test_02_empty() {
    assert_eq!(count_primes_parallel(&[], 4), 0);
  }

  #[test]
  fn test_03_single_thread_matches_sequential() {
    let nums: Vec<u64> = (2..=2000).collect();
    assert_eq!(
      count_primes_parallel(&nums, 1),
      count_primes_sequential(&nums)
    );
  }

  #[test]
  fn test_04_more_threads_than_items() {
    // 2, 3, 5 are prime, 4 is not.
    let nums = vec![2, 3, 4, 5];
    assert_eq!(count_primes_parallel(&nums, 16), 3);
  }

  #[test]
  fn test_05_matches_sequential_on_larger_input() {
    let nums: Vec<u64> = (0..5000).map(|i| i * 7 + 3).collect();
    assert_eq!(
      count_primes_parallel(&nums, 8),
      count_primes_sequential(&nums)
    );
  }

  #[test]
  fn test_06_zero_threads_is_treated_as_one() {
    let nums: Vec<u64> = (2..=100).collect();
    assert_eq!(
      count_primes_parallel(&nums, 0),
      count_primes_sequential(&nums)
    );
  }
}
