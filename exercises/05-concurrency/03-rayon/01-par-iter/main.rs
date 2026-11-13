use rayon::prelude::*;
use std::time::Instant;

fn collatz_steps(mut n: u64) -> u64 {
    let mut steps = 0;
    while n != 1 {
        n = if n.is_multiple_of(2) {
            n / 2
        } else {
            3 * n + 1
        };
        steps += 1;
    }
    steps
}

fn total_steps_seq(numbers: &[u64]) -> u64 {
    numbers.iter().map(|&n| collatz_steps(n)).sum()
}

fn total_steps_par(numbers: &[u64]) -> u64 {
    // TODO: same computation as `total_steps_seq`, with a parallel iterator
    todo!()
}

fn main() {
    let numbers: Vec<u64> = (1..=500_000).collect();

    let start = Instant::now();
    let seq = total_steps_seq(&numbers);
    let seq_time = start.elapsed();

    let start = Instant::now();
    let par = total_steps_par(&numbers);
    let par_time = start.elapsed();

    println!("sequential: {seq} steps in {seq_time:?}");
    println!("parallel:   {par} steps in {par_time:?}");
    println!(
        "speedup:    {:.2}x",
        seq_time.as_secs_f64() / par_time.as_secs_f64()
    );
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_values() {
        assert_eq!(total_steps_par(&[1]), 0);
        assert_eq!(total_steps_par(&[27]), 111);
        assert_eq!(total_steps_par(&[1, 2, 3]), 8);
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(total_steps_par(&[]), 0);
    }

    #[test]
    fn test_matches_sequential() {
        let numbers: Vec<u64> = (1..=10_000).collect();
        assert_eq!(total_steps_par(&numbers), total_steps_seq(&numbers));
    }
}
