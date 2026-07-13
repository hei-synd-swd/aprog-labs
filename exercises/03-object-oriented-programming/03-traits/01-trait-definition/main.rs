struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

// TODO: define the Describe trait with describe(&self) -> String

// TODO: implement Describe for Rectangle

fn main() {
    let rect = Rectangle::new(10, 20);
    println!("{}", rect.describe());
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_rectangle() {
        let rect = Rectangle::new(5, 8);
        assert_eq!(rect.describe(), "Rectangle: 5 x 8");
    }

    #[test]
    fn test_describe_another() {
        let rect = Rectangle::new(3, 15);
        assert_eq!(rect.describe(), "Rectangle: 3 x 15");
    }
}
