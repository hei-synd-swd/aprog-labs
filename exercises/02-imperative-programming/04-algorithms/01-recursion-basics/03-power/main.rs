// TODO: `power(base: u64, exp: u32) -> u64`
// Base case: exp == 0 → 1
// Recursive: base * power(base, exp - 1)
// fn power(base: u64, exp: u32) -> u64 { ... }

fn main() {
    // println!("{}", power(2, 10));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power() {
        assert_eq!(power(2, 0), 1);
        assert_eq!(power(2, 10), 1024);
        assert_eq!(power(5, 3), 125);
        assert_eq!(power(3, 4), 81);
    }
}
