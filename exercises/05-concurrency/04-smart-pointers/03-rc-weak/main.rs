use std::rc::{Rc, Weak};

fn add_owner(document: &Rc<String>) -> Rc<String> {
    // TODO: return one more owner of the same document
    todo!()
}

fn observe(document: &Rc<String>) -> Weak<String> {
    // TODO: return a non-owning handle to the document
    todo!()
}

fn read(observer: &Weak<String>) -> Option<String> {
    // TODO: return a copy of the text if the document still exists
    todo!()
}

fn main() {
    let document = Rc::new(String::from("fearless concurrency"));
    let editor = add_owner(&document);
    let preview = observe(&document);

    println!(
        "strong = {}, weak = {}",
        Rc::strong_count(&document),
        Rc::weak_count(&document)
    );
    println!("both owners alive: {:?}", read(&preview));

    drop(document);
    println!("one owner left:    {:?}", read(&preview));

    drop(editor);
    println!("no owner left:     {:?}", read(&preview));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_owner_shares_the_same_value() {
        let document = Rc::new(String::from("hello"));
        let editor = add_owner(&document);
        assert!(Rc::ptr_eq(&document, &editor));
        assert_eq!(Rc::strong_count(&document), 2);
    }

    #[test]
    fn test_every_owner_counts() {
        let document = Rc::new(String::from("hello"));
        let _first = add_owner(&document);
        let _second = add_owner(&document);
        assert_eq!(Rc::strong_count(&document), 3);
    }

    #[test]
    fn test_observe_does_not_own() {
        let document = Rc::new(String::from("hello"));
        let _preview = observe(&document);
        assert_eq!(Rc::strong_count(&document), 1);
        assert_eq!(Rc::weak_count(&document), 1);
    }

    #[test]
    fn test_read_while_alive() {
        let document = Rc::new(String::from("hello"));
        let preview = observe(&document);
        assert_eq!(read(&preview), Some(String::from("hello")));
    }

    #[test]
    fn test_value_lives_as_long_as_one_owner() {
        let document = Rc::new(String::from("hello"));
        let editor = add_owner(&document);
        let preview = observe(&document);
        drop(document);
        assert_eq!(read(&preview), Some(String::from("hello")));
        drop(editor);
        assert_eq!(read(&preview), None);
    }

    #[test]
    fn test_reading_does_not_keep_the_value_alive() {
        let document = Rc::new(String::from("hello"));
        let preview = observe(&document);
        let _text = read(&preview);
        assert_eq!(Rc::strong_count(&document), 1);
    }
}
