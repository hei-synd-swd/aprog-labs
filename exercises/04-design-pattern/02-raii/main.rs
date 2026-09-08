struct TempDir {
    // TODO: Add a field to hold the directory name
}

impl TempDir {
    // TODO: Implement `TempDir` with a `new` constructor
    // fn new(name: &str -> Self {...}
}

impl Drop for TempDir {
    // TODO: Implement `Drop` for `TempDir`
    // fn drop(&mut self) {...}
}

fn main() {
    // TODO: Create a `TempDir` inside a block, then print after the block
    println!("Main continues");
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp_dir_creation() {
        let _dir = TempDir::new("test_dir");
    }
}
