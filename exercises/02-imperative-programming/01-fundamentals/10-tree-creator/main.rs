/// Build an ASCII Christmas tree of the given height.
/// Row i (0-indexed) has 2*i + 1 stars, center-aligned in width 2*height - 1.
fn build_tree(height: u32) -> String {
    // TODO: loop over rows, center-align stars with format padding
    todo!()
}

fn main() {
    print!("{}", build_tree(4));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_height_1() {
        assert_eq!(build_tree(1), "*\n");
    }

    #[test]
    fn test_tree_height_3() {
        assert_eq!(build_tree(3), "  *  \n *** \n*****\n");
    }

    #[test]
    fn test_tree_height_4() {
        let tree = build_tree(4);
        let lines: Vec<&str> = tree.lines().collect();
        assert_eq!(lines.len(), 4);
        assert_eq!(lines[3], "*******");
    }
}
