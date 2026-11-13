// TODO: `linear_search(arr: &[i32], target: i32) -> Option<usize>`
// Loop over indices, return the first where arr[i] == target.
// fn linear_search(arr: &[i32], target: i32) -> Option<usize> { ... }

fn main() {
    println!("{:?}", linear_search(&[5, 3, 8, 1], 8));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        assert_eq!(linear_search(&[5, 3, 8, 1], 8), Some(2));
        assert_eq!(linear_search(&[5, 3, 8, 1], 9), None);
        assert_eq!(linear_search(&[], 1), None);
    }
}
