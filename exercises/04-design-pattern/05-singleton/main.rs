use std::sync::OnceLock;

struct Logger {
    // TODO: Add a field to hold the log level
}

impl Logger {
    // TODO: Implement `global()` returning `&'static Logger`
    // fn global() -> &'static Logger {...}

    fn level(&self) -> &str {
        &self.level
    }
}

fn main() {
    // TODO: Access the singleton and print the log level
    {
        // TODO: Access the singleton again and print the log level
    }
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singleton_returns_same_instance() {
        let a = Logger::global();
        let b = Logger::global();
        assert!(std::ptr::eq(a, b));
    }

    #[test]
    fn test_log_level() {
        let logger = Logger::global();
        assert_eq!(logger.level(), "INFO");
    }
}
