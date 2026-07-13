/// `n` right-aligned in a field of width 8, padded with leading zeros.
fn zero_pad(n: u32) -> String {
    // TODO: format `n` with width 8 and leading zeros
    todo!()
}

/// `n` as an 8-bit binary string with leading zeros.
fn as_binary(n: u8) -> String {
    // TODO: format `n` as 8-bit binary with leading zeros
    todo!()
}

/// `x` formatted with exactly two decimal places.
fn two_decimals(x: f64) -> String {
    // TODO: format `x` with exactly 2 decimal places
    todo!()
}

/// `n` as lowercase hex with a `0x` prefix, padded to 4 hex digits.
fn as_hex(n: u16) -> String {
    // TODO: format `n` as lowercase hex with 0x prefix, padded to 4 digits
    todo!()
}

fn main() {
    println!(
        "{} {} {} {}",
        zero_pad(42),
        as_binary(5),
        two_decimals(3.14159),
        as_hex(255)
    );
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_pad() {
        assert_eq!(zero_pad(42), "00000042");
    }

    #[test]
    fn test_as_binary() {
        assert_eq!(as_binary(5), "00000101");
    }

    #[test]
    fn test_two_decimals() {
        assert_eq!(two_decimals(3.14159), "3.14");
    }

    #[test]
    fn test_as_hex() {
        assert_eq!(as_hex(255), "0x00ff");
    }
}
