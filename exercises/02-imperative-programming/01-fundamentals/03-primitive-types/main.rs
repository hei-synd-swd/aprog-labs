/// Return the largest value a `u8` can hold.
fn u8_max() -> u8 {
    // TODO: use the associated constant for u8's maximum.
    todo!()
}

/// Return true if `c` is a lowercase English vowel (a, e, i, o, u).
fn is_vowel(c: char) -> bool {
    // TODO: return true if c is one of a, e, i, o, u.
    todo!()
}

/// Return true when exactly one of the booleans is true (XOR).
fn logical_xor(a: bool, b: bool) -> bool {
    // TODO: return true when a and b differ.
    todo!()
}

fn main() {
    println!(
        "{} {} {}",
        u8_max(),
        is_vowel('e'),
        logical_xor(true, false)
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
    fn test_u8_max() {
        assert_eq!(u8_max(), 255);
    }

    #[test]
    fn test_is_vowel() {
        for c in 'a'..='z' {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    assert!(is_vowel(c), "{} should be a vowel", c);
                }
                _ => {
                    assert!(!is_vowel(c), "{} should not be a vowel", c);
                }
            }
        }
    }
    #[test]
    fn test_xor_true_false() {
        assert!(logical_xor(true, false));
        assert!(logical_xor(false, true));
    }

    #[test]
    fn test_xor_both_same() {
        assert!(!logical_xor(true, true));
        assert!(!logical_xor(false, false));
    }
}
