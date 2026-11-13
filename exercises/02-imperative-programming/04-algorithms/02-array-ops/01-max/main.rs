// TODO: implement `max`
// If arr is empty → None, otherwise track the largest seen.
fn max(arr: &[i32]) -> Option<i32> {
    todo!()
}

fn main() {
    println!("{:?}", max(&[3, 7, 2, 9]));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(max(&[]), None);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(max(&[5]), Some(5));
    }

    #[test]
    fn test_several_elements() {
        assert_eq!(max(&[3, 7, 2, 9]), Some(9));
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(max(&[-5, -2, -10]), Some(-2));
    }
}
