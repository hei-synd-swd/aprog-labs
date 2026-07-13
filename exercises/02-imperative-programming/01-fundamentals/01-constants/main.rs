// Building-block constants (already defined for you).
const SECONDS_PER_MINUTE: u32 = 60;
const MINUTES_PER_HOUR: u32 = 60;
const HOURS_PER_DAY: u32 = 24;

// TODO: define MAX_LOGIN_ATTEMPTS as a `u8` constant equal to 5.
const MAX_LOGIN_ATTEMPTS: u8 = 0; // TODO

// TODO: compute SECONDS_PER_DAY as a `u32` constant expression using the
// three building-block constants above (do not write 86400 directly).
const SECONDS_PER_DAY: u32 = 0; // TODO

fn main() {
    println!("max login attempts: {MAX_LOGIN_ATTEMPTS}");
    println!("seconds per day: {SECONDS_PER_DAY}");
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_login_attempts() {
        assert_eq!(MAX_LOGIN_ATTEMPTS, 5);
    }

    #[test]
    fn test_seconds_per_day() {
        assert_eq!(SECONDS_PER_DAY, 86_400);
    }
}
