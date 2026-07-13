//! A simple Rust hello-world project.
//!
//! This project demonstrates the basics of Rust development, including:
//! - Basic program execution with `std::println!`
//! - Emoji output using UTF-8 strings
//! - Rustdoc documentation

/// Prints a friendly greeting to stdout.
///
/// # Examples
///
/// ```
/// hello_world::print_greeting();
/// ```
pub fn print_greeting() {
    println!("Hello, world! 🦀👋");
    println!("Welcome to your first Rust project!");
}

/// Returns the emoji version of the greeting.
///
/// # Examples
///
/// ```
/// let msg = hello_world::greet();
/// assert!(msg.contains("🦀"));
/// ```
pub fn greet() -> String {
    format!("Hello from Rust! 🦀✨")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_contains_emoji() {
        let msg = greet();
        assert!(msg.contains("🦀"), "Greeting should contain the Rust crab emoji");
    }

    #[test]
    fn test_greet_contains_hello() {
        let msg = greet();
        assert!(msg.to_lowercase().contains("hello"), "Greeting should contain 'hello'");
    }
}
