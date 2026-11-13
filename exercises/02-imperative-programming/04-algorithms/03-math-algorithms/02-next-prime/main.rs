// TODO: implement `is_prime` (reuse your solution of the previous exercise)
fn is_prime(n: u64) -> bool {
    todo!()
}

// TODO: implement `next_prime`
// Loop from n upward, return the first number where is_prime is true.
fn next_prime(n: u64) -> u64 {
    todo!()
}

fn main() {
    println!("{}", next_prime(10));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_after_composite_number() {
        assert_eq!(next_prime(10), 11);
        assert_eq!(next_prime(100), 101);
    }

    #[test]
    fn test_prime_stays() {
        assert_eq!(next_prime(13), 13);
    }

    #[test]
    fn test_below_two() {
        assert_eq!(next_prime(1), 2);
    }
}
