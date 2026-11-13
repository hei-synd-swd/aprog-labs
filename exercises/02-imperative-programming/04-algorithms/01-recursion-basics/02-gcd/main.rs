// TODO: implement `gcd`
// Base case: b == 0 → a
// Recursive: gcd(b, a % b)
fn gcd(a: u64, b: u64) -> u64 {
    todo!()
}

fn main() {
    println!("{}", gcd(48, 18));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_divisor() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(100, 10), 10);
    }

    #[test]
    fn test_coprime_numbers() {
        assert_eq!(gcd(7, 3), 1);
    }

    #[test]
    fn test_zero() {
        assert_eq!(gcd(0, 5), 5);
    }
}
