fn speedup(parallel_fraction: f64, processors: u32) -> f64 {
    1.0 / ((1.0 - parallel_fraction) + parallel_fraction / f64::from(processors))
}

fn max_speedup(parallel_fraction: f64) -> f64 {
    // TODO: return the speedup with infinitely many processors
    todo!()
}

fn cores_needed(parallel_fraction: f64, target: f64) -> Option<u32> {
    // TODO: return the smallest number of processors that reaches `target`,
    //       or `None` if `target` can never be reached
    todo!()
}

fn main() {
    let parallel_fraction = 0.95;
    println!("limit: {:.1}x", max_speedup(parallel_fraction));

    for target in [4.0, 8.0, 12.0, 17.0, 25.0] {
        match cores_needed(parallel_fraction, target) {
            Some(processors) => println!("{target:>4}x needs {processors} processors"),
            None => println!("{target:>4}x is out of reach"),
        }
    }
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 0.001,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn test_max_speedup() {
        assert_close(max_speedup(0.5), 2.0);
        assert_close(max_speedup(0.9), 10.0);
        assert_close(max_speedup(0.95), 20.0);
        assert_close(max_speedup(0.0), 1.0);
    }

    #[test]
    fn test_max_speedup_fully_parallel_is_unlimited() {
        assert!(max_speedup(1.0).is_infinite());
    }

    #[test]
    fn test_cores_needed() {
        assert_eq!(cores_needed(0.9, 3.0), Some(4));
        assert_eq!(cores_needed(0.9, 6.0), Some(14));
        assert_eq!(cores_needed(0.5, 1.7), Some(6));
    }

    #[test]
    fn test_one_processor_is_enough() {
        assert_eq!(cores_needed(0.9, 0.8), Some(1));
    }

    #[test]
    fn test_target_out_of_reach() {
        assert_eq!(cores_needed(0.9, 12.0), None);
        assert_eq!(cores_needed(0.5, 2.0), None);
    }

    #[test]
    fn test_fully_parallel_program() {
        assert_eq!(cores_needed(1.0, 8.0), Some(8));
    }
}
