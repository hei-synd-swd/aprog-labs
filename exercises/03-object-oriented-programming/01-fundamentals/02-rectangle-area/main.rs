struct Rectangle {
    width: u32,
    height: u32,
}

// TODO: add an impl block with two methods:
//   - fn area(&self) -> u32     — returns width × height
//   - fn perimeter(&self) -> u32 — returns 2 × (width + height)

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 20,
    };
    println!("Rectangle: {} x {}", rect.width, rect.height);
    // TODO: print the area and perimeter once methods are implemented
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let rect = Rectangle {
            width: 5,
            height: 8,
        };
        assert_eq!(rect.area(), 40);
    }

    #[test]
    fn test_perimeter() {
        let rect = Rectangle {
            width: 5,
            height: 8,
        };
        assert_eq!(rect.perimeter(), 26);
    }

    #[test]
    fn test_square() {
        let square = Rectangle {
            width: 7,
            height: 7,
        };
        assert_eq!(square.area(), 49);
        assert_eq!(square.perimeter(), 28);
    }
}
