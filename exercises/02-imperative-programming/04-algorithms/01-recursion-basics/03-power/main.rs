// TODO: implement `power`
// Base case: exp == 0 → 1
// Recursive: base * power(base, exp - 1)
fn power(base: u64, exp: u32) -> u64 {
    todo!()
}

fn main() {
    println!("{}", power(2, 10));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponent_zero() {
        assert_eq!(power(2, 0), 1);
    }

    #[test]
    fn test_power_of_two() {
        assert_eq!(power(2, 10), 1024);
    }

    #[test]
    fn test_other_bases() {
        assert_eq!(power(5, 3), 125);
        assert_eq!(power(3, 4), 81);
    }
}
