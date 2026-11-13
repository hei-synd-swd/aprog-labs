// TODO: implement `encode`
// compress runs of the same character into count + character
fn encode(input: &str) -> String {
    todo!()
}

// TODO: implement `decode`
// expand count + character pairs back into the original string
fn decode(input: &str) -> String {
    todo!()
}

fn main() {
    let original = "aaabbc";
    let encoded = encode(original);
    println!("original: {}", original);
    println!("encoded:  {}", encoded);
    println!("decoded:  {}", decode(&encoded));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_basic() {
        assert_eq!(encode("aaabbc"), "3a2b1c");
    }

    #[test]
    fn test_encode_single_chars() {
        assert_eq!(encode("abc"), "1a1b1c");
    }

    #[test]
    fn test_encode_empty() {
        assert_eq!(encode(""), "");
    }

    #[test]
    fn test_decode_basic() {
        assert_eq!(decode("3a2b1c"), "aaabbc");
    }

    #[test]
    fn test_decode_multi_digit_count() {
        assert_eq!(decode("12a1b"), "aaaaaaaaaaaab");
    }

    #[test]
    fn test_decode_empty() {
        assert_eq!(decode(""), "");
    }

    #[test]
    fn test_round_trip() {
        let original = "wwwwaaadexxxxxxyyyyyyyyz";
        assert_eq!(decode(&encode(original)), original);
    }
}
