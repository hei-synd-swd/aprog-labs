// TODO: define the `Level` enum with `Low`, `Medium`, `High`.
// Remember to add the right `#[derive(...)]` above it!

type Priority = u8;

// TODO: implement `level_priority` that maps Low→1, Medium→2, High→3.
// fn level_priority(level: &Level) -> Priority { ... }

// TODO: implement `can_skip` that returns true only for Low.
// fn can_skip(level: &Level) -> bool { ... }

fn main() {
    let level = Level::Medium;
    println!("level: {:?}, priority: {}", level, level_priority(&level));
    println!("can skip? {}", can_skip(&level));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_low() {
        assert_eq!(level_priority(&Level::Low), 1);
    }

    #[test]
    fn test_priority_medium() {
        assert_eq!(level_priority(&Level::Medium), 2);
    }

    #[test]
    fn test_priority_high() {
        assert_eq!(level_priority(&Level::High), 3);
    }

    #[test]
    fn test_can_skip() {
        assert!(can_skip(&Level::Low));
        assert!(!can_skip(&Level::Medium));
        assert!(!can_skip(&Level::High));
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(Level::Low, Level::Low);
        assert_ne!(Level::Low, Level::High);
    }

    #[test]
    fn test_debug_format() {
        let text = format!("{:?}", Level::Medium);
        assert_eq!(text, "Medium");
    }

    #[test]
    fn test_copy_semantics() {
        let a = Level::High;
        let b = a;
        assert_eq!(a, b);
    }
}
