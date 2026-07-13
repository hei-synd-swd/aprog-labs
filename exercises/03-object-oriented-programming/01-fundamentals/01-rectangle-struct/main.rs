// TODO: define the Rectangle struct with width and height fields (both u32).

fn main() {
    // TODO: create an instance of Rectangle with width 10 and height 20
    // TODO: print the rectangle's dimensions
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_rectangle() {
        let rect = Rectangle {
            width: 5,
            height: 8,
        };
        assert_eq!(rect.width, 5);
        assert_eq!(rect.height, 8);
    }
}
