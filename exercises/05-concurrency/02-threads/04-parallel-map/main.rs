huse std::thread;

// TODO: add the bounds that allow `T` and `R` to cross a thread boundary
fn parallel_map<T, R>(items: Vec<T>, workers: usize, f: fn(T) -> R) -> Vec<R> {
    // TODO: split `items` into at most `workers` owned chunks, map every chunk
    //       in its own thread and return all results in input order
    todo!()
}

fn main() {
    let words = vec![
        String::from("fearless"),
        String::from("concurrency"),
        String::from("in"),
        String::from("Rust"),
    ];

    let lengths = parallel_map(words, 2, |word| word.len());

    println!("{lengths:?}");
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::thread::ThreadId;

    #[test]
    fn test_keeps_the_input_order() {
        let numbers: Vec<u64> = (1..=10).collect();
        let squares = parallel_map(numbers, 3, |n| n * n);
        assert_eq!(squares, vec![1, 4, 9, 16, 25, 36, 49, 64, 81, 100]);
    }

    #[test]
    fn test_works_for_types_that_are_not_copy() {
        let words = vec![String::from("a"), String::from("bcd"), String::from("ef")];
        assert_eq!(parallel_map(words, 2, |word| word.len()), vec![1, 3, 2]);
    }

    #[test]
    fn test_empty_input() {
        let results = parallel_map(Vec::<u32>::new(), 4, |n| n + 1);
        assert!(results.is_empty());
    }

    #[test]
    fn test_more_workers_than_items() {
        assert_eq!(parallel_map(vec![1, 2], 8, |n: i32| n * 2), vec![2, 4]);
    }

    #[test]
    fn test_one_worker() {
        assert_eq!(
            parallel_map(vec![1, 2, 3], 1, |n: i32| n + 1),
            vec![2, 3, 4]
        );
    }

    #[test]
    fn test_one_thread_per_chunk() {
        let ids = parallel_map(vec![0_u8; 8], 4, |_| thread::current().id());
        let distinct: HashSet<ThreadId> = ids.iter().copied().collect();
        assert_eq!(distinct.len(), 4);
        assert!(!distinct.contains(&thread::current().id()));
    }
}
