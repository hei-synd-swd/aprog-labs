use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

fn collect_squares(workers: u64) -> Vec<u64> {
    // TODO: one thread per id in `1..=workers` pushes `id * id` into a shared
    //       `Arc<Mutex<Vec<u64>>>`. Return the squares in ascending order.
    todo!()
}

fn count_long_words(words: Vec<String>, min_len: usize) -> usize {
    // TODO: one thread per word adds 1 to a shared `Arc<AtomicUsize>` if the
    //       word has at least `min_len` bytes. Return the final count.
    todo!()
}

fn main() {
    println!("{:?}", collect_squares(8));

    let words = vec![
        String::from("fearless"),
        String::from("concurrency"),
        String::from("in"),
        String::from("Rust"),
        String::from("threads"),
    ];
    println!("{} long words", count_long_words(words, 7));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    fn words(list: &[&str]) -> Vec<String> {
        list.iter().map(|word| word.to_string()).collect()
    }

    #[test]
    fn test_collect_squares() {
        assert_eq!(collect_squares(4), vec![1, 4, 9, 16]);
    }

    #[test]
    fn test_collect_squares_single_worker() {
        assert_eq!(collect_squares(1), vec![1]);
    }

    #[test]
    fn test_collect_squares_no_worker() {
        assert!(collect_squares(0).is_empty());
    }

    #[test]
    fn test_collect_squares_many_workers() {
        let expected: Vec<u64> = (1..=50).map(|id| id * id).collect();
        assert_eq!(collect_squares(50), expected);
    }

    #[test]
    fn test_count_long_words() {
        assert_eq!(count_long_words(words(&["a", "bbb", "cccc", "dd"]), 3), 2);
    }

    #[test]
    fn test_count_long_words_all_or_none() {
        assert_eq!(count_long_words(words(&["aa", "bb", "cc"]), 2), 3);
        assert_eq!(count_long_words(words(&["aa", "bb", "cc"]), 3), 0);
    }

    #[test]
    fn test_count_long_words_empty_input() {
        assert_eq!(count_long_words(Vec::new(), 3), 0);
    }
}
