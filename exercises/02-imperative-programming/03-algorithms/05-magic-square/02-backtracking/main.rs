fn is_magic(grid: &[Vec<u32>]) -> bool {
    if grid.len() != 3 || grid.iter().any(|r| r.len() != 3) {
        return false;
    }
    let mut seen = [false; 10];
    for row in grid {
        for &n in row {
            if n < 1 || n > 9 || seen[n as usize] {
                return false;
            }
            seen[n as usize] = true;
        }
    }
    const MAGIC: u32 = 15;
    for row in grid {
        if row.iter().sum::<u32>() != MAGIC {
            return false;
        }
    }
    for c in 0..3 {
        if grid[0][c] + grid[1][c] + grid[2][c] != MAGIC {
            return false;
        }
    }
    if grid[0][0] + grid[1][1] + grid[2][2] != MAGIC {
        return false;
    }
    if grid[0][2] + grid[1][1] + grid[2][0] != MAGIC {
        return false;
    }
    true
}

// TODO: recursive helper
// fn solve(grid: &mut Vec<Vec<u32>>, used: &mut [bool; 10], pos: usize, count: &mut u32) { ... }

// TODO: count_magic_squares() -> u32
// fn count_magic_squares() -> u32 { ... }

fn main() {
    println!("{}", count_magic_squares());
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        assert_eq!(count_magic_squares(), 8);
    }
}
