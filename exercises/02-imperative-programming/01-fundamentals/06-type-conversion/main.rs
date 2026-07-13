/// Convert with an `as` cast (fast, but truncates when out of range).
fn truncate_to_u8(n: i32) -> u8 {
    // TODO: cast `n` to a u8 with `as`.
    todo!()
}

/// Convert safely: Some(value) if it fits in a u8, otherwise None.
fn safe_to_u8(n: i32) -> Option<u8> {
    // TODO: use `u8::try_from` to safely convert `n` to a `u8`, returning `None` on overflow.
    todo!()
}

fn main() {
    println!("{} {:?}", truncate_to_u8(300), safe_to_u8(300));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate() {
        assert_eq!(truncate_to_u8(42), 42);
        assert_eq!(truncate_to_u8(300), 44); // 300 - 256
    }

    #[test]
    fn test_safe_ok() {
        assert_eq!(safe_to_u8(200), Some(200));
        assert_eq!(safe_to_u8(0), Some(0));
    }

    #[test]
    fn test_safe_none() {
        assert_eq!(safe_to_u8(300), None);
        assert_eq!(safe_to_u8(-1), None);
    }
}
