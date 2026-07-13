struct Rectangle {
    width: u32,
    height: u32,
}

// TODO: add an impl block with a `new` constructor function
// fn new(width: u32, height: u32) -> Rectangle

fn main() {
    let rect = Rectangle::new(10, 20);
    println!("Rectangle: {} x {}", rect.width, rect.height);
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let rect = Rectangle::new(5, 8);
        assert_eq!(rect.width, 5);
        assert_eq!(rect.height, 8);
    }
}
