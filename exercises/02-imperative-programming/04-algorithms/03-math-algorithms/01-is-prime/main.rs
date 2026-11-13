// TODO: `is_prime(n: u64) -> bool`
// n < 2 → false. Check divisors 2..=sqrt(n). If any divides n → false.
// fn is_prime(n: u64) -> bool { ... }

fn main() {
    println!("{}", is_prime(17));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(17));
        assert!(is_prime(97));
        assert!(!is_prime(1));
        assert!(!is_prime(0));
        assert!(!is_prime(4));
        assert!(!is_prime(100));
    }
}
