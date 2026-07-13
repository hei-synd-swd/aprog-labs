/// Trim a padded numeric string and parse it into an `i32`,
/// reusing the name `input` through shadowing.
fn parse_padded(input: &str) -> i32 {
    // TODO: shadow `input` with its trimmed &str,
    // then shadow it again with the parsed i32, and return it.
    todo!()
}

fn main() {
    println!("{}", parse_padded("  42  "));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_padded() {
        assert_eq!(parse_padded("  42  "), 42);
    }

    #[test]
    fn test_plain() {
        assert_eq!(parse_padded("7"), 7);
    }

    #[test]
    fn test_negative() {
        assert_eq!(parse_padded("  -5"), -5);
    }
}
