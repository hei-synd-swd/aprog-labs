fn speedup(parallel_fraction: f64, processors: u32) -> f64 {
    // TODO: return Amdahl's Law: 1 / ((1 - f) + f / p)
    todo!()
}

fn main() {
    for processors in [1, 2, 4, 8, 16, 1024] {
        println!(
            "{processors:>5} processors: {:.2}x",
            speedup(0.9, processors)
        );
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
    fn test_half_parallel() {
        assert_close(speedup(0.5, 2), 1.3333);
    }

    #[test]
    fn test_mostly_parallel() {
        assert_close(speedup(0.9, 10), 5.2632);
        assert_close(speedup(0.75, 4), 2.2857);
    }

    #[test]
    fn test_nothing_parallel() {
        assert_close(speedup(0.0, 8), 1.0);
    }

    #[test]
    fn test_everything_parallel() {
        assert_close(speedup(1.0, 4), 4.0);
    }

    #[test]
    fn test_one_processor() {
        assert_close(speedup(0.9, 1), 1.0);
    }
}
