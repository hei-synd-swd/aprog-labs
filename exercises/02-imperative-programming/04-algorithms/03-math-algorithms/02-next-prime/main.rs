// TODO: `next_prime(n: u64) -> u64`
// Loop from n upward, return the first number where is_prime is true.
// fn next_prime(n: u64) -> u64 { ... }

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
    fn test_next_prime() {
        assert_eq!(next_prime(10), 11);
        assert_eq!(next_prime(13), 13);
        assert_eq!(next_prime(1), 2);
        assert_eq!(next_prime(100), 101);
    }
}
