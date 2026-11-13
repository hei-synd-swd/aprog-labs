// TODO: `max(arr: &[i32]) -> Option<i32>`
// If arr is empty → None, otherwise track the largest seen.
// fn max(arr: &[i32]) -> Option<i32> { ... }

fn main() {
    // println!("{:?}", max(&[3, 7, 2, 9]));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
        assert_eq!(max(&[]), None);
        assert_eq!(max(&[5]), Some(5));
        assert_eq!(max(&[3, 7, 2, 9]), Some(9));
        assert_eq!(max(&[-5, -2, -10]), Some(-2));
    }
}
