// TODO: `count_chars(text: &str) -> usize`
// fn count_chars(text: &str) -> usize { ... }

// TODO: `first_word(text: &str) -> &str` (first whitespace-separated word)
// fn first_word(text: &str) -> &str { ... }

// TODO: `add_exclamation(text: &mut String)`
// fn add_exclamation(text: &mut String) { ... }

// TODO: `clear(text: &mut String)`
// fn clear(text: &mut String) { ... }

fn main() {
    let mut s = String::from("hello world");
    // Multiple immutable borrows at once — this works!
    println!("chars: {}, first word: {}", count_chars(&s), first_word(&s));

    add_exclamation(&mut s);
    println!("after exclamation: {}", s);

    clear(&mut s);
    println!("after clear: '{}'", s);
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_chars() {
        assert_eq!(count_chars("hello"), 5);
        assert_eq!(count_chars(""), 0);
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
    }

    #[test]
    fn test_add_exclamation() {
        let mut s = String::from("hi");
        add_exclamation(&mut s);
        assert_eq!(s, "hi!");
    }

    #[test]
    fn test_clear() {
        let mut s = String::from("something");
        clear(&mut s);
        assert_eq!(s, "");
    }

    #[test]
    fn test_multiple_immutable_borrows() {
        let s = String::from("hello world");
        // Both borrow s immutably — this must compile
        let chars = count_chars(&s);
        let word = first_word(&s);
        assert_eq!(chars, 11);
        assert_eq!(word, "hello");
    }
}
