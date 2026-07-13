// TODO: `gcd(a: u64, b: u64) -> u64`
// Base case: b == 0 → a
// Recursive: gcd(b, a % b)
// fn gcd(a: u64, b: u64) -> u64 { ... }

fn main() {
    // println!("{}", gcd(48, 18));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(7, 3), 1);
        assert_eq!(gcd(100, 10), 10);
        assert_eq!(gcd(0, 5), 5);
    }
}
