// TODO: add #[derive(Debug)] to Book
// TODO: implement Display for Book

use std::fmt;

struct Book {
    title: String,
    author: String,
    year: u16,
}

impl Book {
    fn new(title: &str, author: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            author: String::from(author),
            year,
        }
    }
}

fn main() {
    let book = Book::new("1984", "George Orwell", 1949);
    println!("Debug: {:?}", book);
    println!("Display: {}", book);
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_format() {
        let book = Book::new("1984", "George Orwell", 1949);
        assert_eq!(format!("{}", book), "1984 by George Orwell (1949)");
    }

    #[test]
    fn test_display_another() {
        let book = Book::new("Dune", "Frank Herbert", 1965);
        assert_eq!(format!("{}", book), "Dune by Frank Herbert (1965)");
    }

    #[test]
    fn test_debug_derived() {
        let book = Book::new("Test", "Author", 2000);
        // Debug format should include field names
        let debug = format!("{:?}", book);
        assert!(debug.contains("title"));
        assert!(debug.contains("author"));
        assert!(debug.contains("year"));
    }
}
