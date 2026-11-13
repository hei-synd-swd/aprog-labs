// TODO: implement `fib_iter`
// Use (a, b) = (0, 1) and loop n times updating both.
fn fib_iter(n: u32) -> u64 {
    todo!()
}

fn main() {
    println!("{}", fib_iter(10));
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
        assert_eq!(fib_iter(0), 0);
        assert_eq!(fib_iter(1), 1);
    }

    #[test]
    fn test_tenth() {
        assert_eq!(fib_iter(10), 55);
    }

    #[test]
    fn test_large_number() {
        assert_eq!(fib_iter(50), 12_586_269_025);
    }
}
