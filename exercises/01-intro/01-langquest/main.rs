//! Your first LangQuest exercise: fix the greeting program so every test passes.
//! Edit the two functions below, save the file, and watch the Output page.

/// Return the classic greeting string.
fn hello() -> String {
    // TODO: this should return exactly "Hello, world!"
    String::from("Goodbye, world!")
}

/// Build a personal greeting: `greet("Ada")` must return "Hello, Ada!".
fn greet(name: &str) -> String {
    // TODO: return a greeting for `name` in the form "Hello, <name>!"
    format!("Hi, {}", name)
}

fn main() {
    println!("{}", hello());
    println!("{}", greet("HEI-Vs"));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello, world!");
    }

    #[test]
    fn test_greet_simple() {
        assert_eq!(greet("Ada"), "Hello, Ada!");
    }

    #[test]
    fn test_greet_other_name() {
        assert_eq!(greet("HEI-Vs"), "Hello, HEI-Vs!");
    }

    #[test]
    fn test_greet_empty() {
        assert_eq!(greet(""), "Hello, !");
    }
}
