// TODO: implement `digit_sum`
// Loop: add n % 10 to sum, then n /= 10, until n == 0.
fn digit_sum(mut n: u64) -> u64 {
    todo!()
}

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
    fn test_zero() {
        assert_eq!(digit_sum(0), 0);
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(digit_sum(5), 5);
    }

    #[test]
    fn test_several_digits() {
        assert_eq!(digit_sum(123), 6);
        assert_eq!(digit_sum(9999), 36);
    }
}
