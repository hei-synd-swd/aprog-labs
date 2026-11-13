use std::thread;

fn chunk_sums(numbers: Vec<u64>, workers: usize) -> Vec<u64> {
    // TODO: split `numbers` into at most `workers` chunks, sum every chunk
    //       in its own thread and return the partial sums in chunk order
    todo!()
}

fn main() {
    let numbers: Vec<u64> = (1..=1_000_000).collect();

    let partial_sums = chunk_sums(numbers, 4);

    println!("partial sums: {partial_sums:?}");
    println!("total:        {}", partial_sums.iter().sum::<u64>());
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_workers() {
        assert_eq!(chunk_sums((1..=10).collect(), 3), vec![10, 26, 19]);
    }

    #[test]
    fn test_one_worker() {
        assert_eq!(chunk_sums((1..=10).collect(), 1), vec![55]);
    }

    #[test]
    fn test_even_split() {
        assert_eq!(chunk_sums((1..=8).collect(), 4), vec![3, 7, 11, 15]);
    }

    #[test]
    fn test_more_workers_than_numbers() {
        assert_eq!(chunk_sums(vec![1, 2, 3, 4], 8), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_empty_input() {
        assert!(chunk_sums(Vec::new(), 4).is_empty());
    }

    #[test]
    fn test_total_is_preserved() {
        let partial_sums = chunk_sums((1..=1000).collect(), 7);
        assert_eq!(partial_sums.len(), 7);
        assert_eq!(partial_sums.iter().sum::<u64>(), 500_500);
    }
}
