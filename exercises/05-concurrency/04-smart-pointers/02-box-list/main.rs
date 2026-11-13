#[derive(Debug)]
enum List {
    // TODO: add a variant `Cons` holding an `i32` and the rest of the list in a `Box`
    Nil,
}

impl List {
    fn prepend(self, value: i32) -> List {
        // TODO: return a `Cons` with `value` in front of `self`
        todo!()
    }

    fn sum(&self) -> i32 {
        // TODO: add up all values with a `match` and a recursive call
        todo!()
    }
}

fn main() {
    let list = List::Nil.prepend(3).prepend(2).prepend(1);

    println!("{list:?}");
    println!("sum = {}", list.sum());
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list_sums_to_zero() {
        assert_eq!(List::Nil.sum(), 0);
    }

    #[test]
    fn test_prepend_builds_a_cons() {
        let list = List::Nil.prepend(7);
        assert!(matches!(list, List::Cons(7, _)));
    }

    #[test]
    fn test_prepend_puts_the_value_in_front() {
        let list = List::Nil.prepend(3).prepend(2).prepend(1);
        match list {
            List::Cons(first, rest) => {
                assert_eq!(first, 1);
                assert!(matches!(*rest, List::Cons(2, _)));
            }
            List::Nil => panic!("the list must not be empty"),
        }
    }

    #[test]
    fn test_sum() {
        assert_eq!(List::Nil.prepend(3).prepend(2).prepend(1).sum(), 6);
        assert_eq!(List::Nil.prepend(-5).prepend(5).sum(), 0);
    }

    #[test]
    fn test_long_list() {
        let mut list = List::Nil;
        for value in 1..=1000 {
            list = list.prepend(value);
        }
        assert_eq!(list.sum(), 500_500);
    }
}
