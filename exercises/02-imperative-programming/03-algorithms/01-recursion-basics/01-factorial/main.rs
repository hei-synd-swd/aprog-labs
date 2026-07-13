// TODO: `factorial(n: u64) -> u64`
// Base case: n == 0 → 1
// Recursive: n * factorial(n - 1)
// fn factorial(n: u64) -> u64 { ... }

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
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3_628_800);
    }
}
