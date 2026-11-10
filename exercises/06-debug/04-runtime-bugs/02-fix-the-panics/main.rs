/// First and last entry of a log; `None` when the log is empty.
fn first_and_last(entries: &[u32]) -> Option<(u32, u32)> {
    Some((entries[0], entries[entries.len() - 1]))
}

/// Average of the entries, truncated to an integer; `None` when empty.
fn average(entries: &[u32]) -> Option<u32> {
    let sum: u32 = entries.iter().sum();
    Some(sum / entries.len() as u32)
}

/// The `index`-th character of a line (characters, not bytes!); `None`
/// if the line is shorter.
fn nth_char(line: &str, index: usize) -> Option<char> {
    Some(line.as_bytes()[index] as char)
}

fn main() {
    let entries = [12, 30, 44];
    println!("first and last: {:?}", first_and_last(&entries));
    println!("average: {:?}", average(&entries));
    println!("3rd char of \"sensor-42\": {:?}", nth_char("sensor-42", 2));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_and_last() {
        assert_eq!(first_and_last(&[12, 30, 44]), Some((12, 44)));
        assert_eq!(first_and_last(&[7]), Some((7, 7)));
        assert_eq!(first_and_last(&[]), None);
    }

    #[test]
    fn test_average() {
        assert_eq!(average(&[10, 20, 30]), Some(20));
        assert_eq!(average(&[0]), Some(0));
        assert_eq!(average(&[]), None);
        assert_eq!(
            average(&[4_000_000_000, 400_000_000, 200_000_000]),
            Some(1_533_333_333)
        );
    }

    #[test]
    fn test_nth_char() {
        assert_eq!(nth_char("hello", 1), Some('e'));
        assert_eq!(nth_char("héllo", 1), Some('é'));
        assert_eq!(nth_char("hi", 9), None);
        assert_eq!(nth_char("🩷", 0), Some('🩷'));
        assert_eq!(nth_char("", 0), None);
    }
}
