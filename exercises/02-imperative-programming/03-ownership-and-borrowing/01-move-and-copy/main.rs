fn pair_of(n: u32) -> (u32, u32) {
    // TODO: return the number twice
    todo!()
}

fn two_copies(text: String) -> (String, String) {
    // TODO: return the text twice, as two independent strings
    todo!()
}

fn keep_longer(a: String, b: String) -> String {
    // TODO: return the longer text, `a` if both have the same length
    todo!()
}

fn main() {
    let n = 21;
    println!("{:?}", pair_of(n));
    println!("n is still usable: {n}");

    let text = String::from("Ferris");
    let (first, second) = two_copies(text);
    // println!("{text}"); // try it: `text` was moved into `two_copies`
    println!("{first} and {second}");

    println!(
        "{}",
        keep_longer(String::from("Rust"), String::from("Ferris"))
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
    fn test_pair_of() {
        assert_eq!(pair_of(7), (7, 7));
        assert_eq!(pair_of(0), (0, 0));
    }

    #[test]
    fn test_copy_types_stay_usable() {
        let n = 21;
        let pair = pair_of(n);
        assert_eq!(pair.0 + pair.1, 2 * n);
    }

    #[test]
    fn test_two_copies() {
        let (first, second) = two_copies(String::from("ab"));
        assert_eq!(first, "ab");
        assert_eq!(second, "ab");
    }

    #[test]
    fn test_two_copies_are_independent() {
        let (mut first, second) = two_copies(String::from("ab"));
        first.push('!');
        assert_eq!(first, "ab!");
        assert_eq!(second, "ab");
    }

    #[test]
    fn test_keep_longer() {
        let longer = keep_longer(String::from("Rust"), String::from("Ferris"));
        assert_eq!(longer, "Ferris");
        let longer = keep_longer(String::from("ownership"), String::from("move"));
        assert_eq!(longer, "ownership");
    }

    #[test]
    fn test_keep_longer_prefers_the_first_on_a_tie() {
        let longer = keep_longer(String::from("abc"), String::from("xyz"));
        assert_eq!(longer, "abc");
    }
}
