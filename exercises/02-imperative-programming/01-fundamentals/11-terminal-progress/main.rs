/// Build a progress bar: "[###.......]  30%"
fn progress_bar(current: u32, total: u32, width: u32) -> String {
    // TODO: compute filled positions and percentage, format as bar
    todo!()
}

fn main() {
    println!("{}", progress_bar(3, 10, 10));
    println!("{}", progress_bar(7, 10, 10));
    println!("{}", progress_bar(10, 10, 10));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_30() {
        assert_eq!(progress_bar(3, 10, 10), "[###.......]  30%");
    }

    #[test]
    fn test_progress_50() {
        assert_eq!(progress_bar(5, 10, 10), "[#####.....]  50%");
    }

    #[test]
    fn test_progress_100() {
        assert_eq!(progress_bar(10, 10, 10), "[##########] 100%");
    }

    #[test]
    fn test_progress_0() {
        assert_eq!(progress_bar(0, 10, 10), "[..........]   0%");
    }
}
