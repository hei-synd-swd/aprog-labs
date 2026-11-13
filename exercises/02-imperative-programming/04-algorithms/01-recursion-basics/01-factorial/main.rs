// TODO: implement `factorial`
// Base case: n == 0 → 1
// Recursive: n * factorial(n - 1)
fn factorial(n: u64) -> u64 {
    todo!()
}

fn main() {
    println!("{}", factorial(5));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_cases() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn test_small_number() {
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn test_larger_number() {
        assert_eq!(factorial(10), 3_628_800);
    }
}
