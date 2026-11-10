// Library book-log.
//
// This file does NOT compile: it contains four ownership / borrow errors.
// Read the compiler output, fix the errors, keep the intent of the code.

fn summarize_first(title: String, pages: u32) -> String {
    let mut summary = title;
    summary.push_str(&format!(" ({pages} pages)"));
    println!("Logging: {title}");
    summary
}

fn add_placeholder(library: &mut Vec<String>) {
    let first = &library[0];
    library.push(String::from("placeholder"));
    println!("first book: {first}");
}

fn shout(title: String) -> &str {
    let loud = format!("{title}!!!");
    &loud
}

fn main() {
    let title = String::from("Dune");
    let summary = summarize_first(title, 412);
    println!("summary: {summary}");

    let mut library = vec![String::from("Dune"), String::from("Neuromancer")];
    let top = &library[0];
    println!("loudest: {}", shout(String::from("Rust")));
    library.push(String::from("placeholder"));
    println!("library: {library:?}");
    println!("top book: {top} ({} chars)", top.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summarize_first() {
        assert_eq!(
            summarize_first(String::from("Dune"), 412),
            "Dune (412 pages)"
        );
    }

    #[test]
    fn test_add_placeholder() {
        let mut library = vec![String::from("Dune")];
        add_placeholder(&mut library);
        assert_eq!(
            library,
            vec![String::from("Dune"), String::from("placeholder")]
        );
    }

    #[test]
    fn test_shout() {
        assert_eq!(shout(String::from("Rust")), "Rust!!!");
    }
}
