fn level(line: &str) -> &str {
    // TODO: return the part before the first ':', or the whole line
    todo!()
}

// TODO: add the lifetime annotations
fn message_or(line: &str, fallback: &str) -> &str {
    // TODO: return the part after the first ": ", or `fallback`
    todo!()
}

// TODO: add the lifetime annotations
fn strip_tag(line: &str, tag: &str) -> &str {
    // TODO: return the line without the leading `tag`
    todo!()
}

fn main() {
    let line = String::from("WARN: disk almost full");

    println!("{}", level(&line));
    println!("{}", message_or(&line, "n/a"));
    println!("{}", message_or("broken line", "n/a"));
    println!("{}", strip_tag("[db] connected", "[db] "));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level() {
        assert_eq!(level("WARN: disk almost full"), "WARN");
        assert_eq!(level("ERROR: code: 42"), "ERROR");
    }

    #[test]
    fn test_level_without_colon() {
        assert_eq!(level("plain text"), "plain text");
    }

    #[test]
    fn test_message_or() {
        assert_eq!(
            message_or("WARN: disk almost full", "n/a"),
            "disk almost full"
        );
        assert_eq!(message_or("ERROR: code: 42", "n/a"), "code: 42");
    }

    #[test]
    fn test_message_or_uses_the_fallback() {
        assert_eq!(message_or("broken line", "n/a"), "n/a");
    }

    #[test]
    fn test_strip_tag() {
        assert_eq!(strip_tag("[db] connected", "[db] "), "connected");
        assert_eq!(strip_tag("connected", "[db] "), "connected");
    }

    #[test]
    fn test_result_does_not_borrow_from_the_tag() {
        let line = String::from("[net] timeout");
        let rest;
        {
            let tag = String::from("[net] ");
            rest = strip_tag(&line, &tag);
        } // `tag` is dropped here, `rest` must still be valid
        assert_eq!(rest, "timeout");
    }
}
