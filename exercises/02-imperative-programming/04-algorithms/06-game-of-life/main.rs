// TODO: implement `count_neighbors`
fn count_neighbors(grid: &[Vec<bool>], row: usize, col: usize) -> u8 {
    todo!()
}

// TODO: implement `next_generation`
fn next_generation(grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    todo!()
}

fn main() {
    // TODO: create a blinker pattern and print the next generation
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_neighbors_center() {
        let grid = vec![
            vec![true, true, true],
            vec![true, false, true],
            vec![true, true, true],
        ];
        assert_eq!(count_neighbors(&grid, 1, 1), 8);
    }

    #[test]
    fn test_count_neighbors_corner() {
        let grid = vec![vec![false, true], vec![true, true]];
        assert_eq!(count_neighbors(&grid, 0, 0), 3);
    }

    #[test]
    fn test_blinker_oscillates() {
        let blinker = vec![
            vec![false, true, false],
            vec![false, true, false],
            vec![false, true, false],
        ];
        let next = next_generation(&blinker);
        // Vertical blinker becomes horizontal
        assert!(!next[0][0]);
        assert!(!next[0][1]);
        assert!(!next[0][2]);
        assert!(next[1][0]);
        assert!(next[1][1]);
        assert!(next[1][2]);
        assert!(!next[2][0]);
        assert!(!next[2][1]);
        assert!(!next[2][2]);
    }

    #[test]
    fn test_block_stable() {
        let block = vec![
            vec![false, false, false, false],
            vec![false, true, true, false],
            vec![false, true, true, false],
            vec![false, false, false, false],
        ];
        let next = next_generation(&block);
        assert_eq!(next, block);
    }
}
