// TODO: helper with memo parameter
// fn fib_memo_dyn(n: u32, memo: &mut Vec<Option<u64>>) -> u64 { ... }

// TODO: `fib_memo(n: u32) -> u64`
// Create a Vec<Option<u64>>, call the helper, return the result.
// fn fib_memo(n: u32) -> u64 { ... }

fn main() {
    println!("{}", fib_memo(10));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_memo() {
        assert_eq!(fib_memo(0), 0);
        assert_eq!(fib_memo(1), 1);
        assert_eq!(fib_memo(10), 55);
        assert_eq!(fib_memo(50), 12_586_269_025);
    }
}
