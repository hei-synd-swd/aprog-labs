use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// TODO: implement Display for Person so {} prints as "Name (Age)"
// impl fmt::Display for Person { ... }

/// Format using Debug ({:?}) — shows the struct field dump
fn format_debug(p: &Person) -> String {
    // TODO: format using Debug
    todo!()
}

/// Format using Display ({}) — shows "Name (Age)"
fn format_display(p: &Person) -> String {
    // TODO: format using Display
    todo!()
}

fn main() {
    let p = Person {
        name: "Alice".into(),
        age: 30,
    };
    println!("Debug:   {}", format_debug(&p));
    println!("Display: {}", format_display(&p));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug() {
        let p = Person {
            name: "Bob".into(),
            age: 25,
        };
        let d = format_debug(&p);
        assert!(d.contains("Bob"), "Debug output should contain the name");
        assert!(d.contains("25"), "Debug output should contain the age");
    }

    #[test]
    fn test_display() {
        let p = Person {
            name: "Charlie".into(),
            age: 42,
        };
        assert_eq!(format_display(&p), "Charlie (42)");
    }
}
