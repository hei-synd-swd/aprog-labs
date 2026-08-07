//! Loading the numbers data file.
//!
//! This module is pre-implemented for you.

use std::fs;
use std::io;
use std::path::Path;

/// Loads a newline-separated list of `u64` numbers from `path`.
///
/// Empty lines and lines starting with `#` (comments) are ignored.
pub fn load_numbers<P: AsRef<Path>>(path: P) -> io::Result<Vec<u64>> {
  let content = fs::read_to_string(path)?;
  let numbers = content
    .lines()
    .map(str::trim)
    .filter(|line| !line.is_empty() && !line.starts_with('#'))
    .filter_map(|line| line.parse::<u64>().ok())
    .collect();
  Ok(numbers)
}
