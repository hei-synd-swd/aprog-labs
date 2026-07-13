// TODO: `digit_sum(n: u64) -> u64`
// Loop: add n % 10 to sum, then n /= 10, until n == 0.
// fn digit_sum(mut n: u64) -> u64 { ... }

fn main() {
    println!("{}", digit_sum(123));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_sum() {
        assert_eq!(digit_sum(0), 0);
        assert_eq!(digit_sum(5), 5);
        assert_eq!(digit_sum(123), 6);
        assert_eq!(digit_sum(9999), 36);
    }
}
