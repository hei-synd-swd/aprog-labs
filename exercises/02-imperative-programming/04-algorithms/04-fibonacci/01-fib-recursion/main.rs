// TODO: `fib_rec(n: u32) -> u64`
// Base: n == 0 → 0, n == 1 → 1
// Recursive: fib_rec(n - 1) + fib_rec(n - 2)
// fn fib_rec(n: u32) -> u64 { ... }

fn main() {
    println!("{}", fib_rec(10));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_rec() {
        assert_eq!(fib_rec(0), 0);
        assert_eq!(fib_rec(1), 1);
        assert_eq!(fib_rec(10), 55);
        assert_eq!(fib_rec(20), 6765);
    }
}
