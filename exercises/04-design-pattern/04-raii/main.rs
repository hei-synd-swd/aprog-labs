use std::cell::RefCell;

//------------------------------------------------------------------------------
// Provided: `report` prints a line and remembers it, so that the tests can
// check what was printed and in which order. Use it instead of `println!`.
//------------------------------------------------------------------------------
thread_local! {
    static REPORTED: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

fn report(message: String) {
    println!("{message}");
    REPORTED.with(|lines| lines.borrow_mut().push(message));
}

struct TempDir {
    // TODO: add a field that holds the directory name
}

impl TempDir {
    fn new(name: &str) -> Self {
        // TODO: report `Created temp dir: <name>` and store the name
        todo!()
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        // TODO: report `Removed temp dir: <name>`
    }
}

fn main() {
    // TODO: create a `TempDir` inside a `{ }` block, in front of this line
    report(String::from("Main continues"));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    fn reported() -> Vec<String> {
        REPORTED.with(|lines| lines.borrow().clone())
    }

    #[test]
    fn test_creation_is_reported() {
        let _dir = TempDir::new("test_dir");
        assert_eq!(reported(), ["Created temp dir: test_dir"]);
    }

    #[test]
    fn test_removed_at_the_end_of_the_block() {
        {
            let _dir = TempDir::new("test_dir");
        }
        assert_eq!(
            reported(),
            ["Created temp dir: test_dir", "Removed temp dir: test_dir"]
        );
    }

    #[test]
    fn test_removed_in_reverse_order() {
        {
            let _first = TempDir::new("a");
            let _second = TempDir::new("b");
        }
        assert_eq!(
            reported(),
            [
                "Created temp dir: a",
                "Created temp dir: b",
                "Removed temp dir: b",
                "Removed temp dir: a",
            ]
        );
    }
}
