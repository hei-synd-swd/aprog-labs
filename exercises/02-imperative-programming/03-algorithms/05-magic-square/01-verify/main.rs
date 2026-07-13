// TODO: `is_magic(grid: &[Vec<u32>]) -> bool`
// Check: 3×3 grid, uses 1..=9 exactly once,
//        every row, column, and diagonal sums to 15.
// fn is_magic(grid: &[Vec<u32>]) -> bool { ... }

fn main() {
    let valid = vec![vec![8, 1, 6], vec![3, 5, 7], vec![4, 9, 2]];
    println!("{}", is_magic(&valid));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_classic() {
        let square = vec![vec![8, 1, 6], vec![3, 5, 7], vec![4, 9, 2]];
        assert!(is_magic(&square));
    }

    #[test]
    fn test_invalid_sums() {
        let square = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert!(!is_magic(&square));
    }

    #[test]
    fn test_wrong_numbers() {
        let square = vec![vec![5, 5, 5], vec![5, 5, 5], vec![5, 5, 5]];
        assert!(!is_magic(&square));
    }

    #[test]
    fn test_not_3x3() {
        let square = vec![vec![1, 2], vec![3, 4]];
        assert!(!is_magic(&square));
    }
}
