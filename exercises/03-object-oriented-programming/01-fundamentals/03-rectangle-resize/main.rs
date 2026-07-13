struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

// TODO: add mutable methods:
//   - fn scale(&mut self, factor: u32)   — multiply width and height by factor
//   - fn shrink(&mut self, factor: u32)  — divide width and height by factor

fn main() {
    let mut rect = Rectangle {
        width: 10,
        height: 20,
    };
    println!(
        "Before: {} x {} (area: {})",
        rect.width,
        rect.height,
        rect.area()
    );

    // TODO: call scale(2) and print the new dimensions
    // TODO: call shrink(4) and print the new dimensions
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
    fn test_scale() {
        let mut rect = Rectangle {
            width: 5,
            height: 8,
        };
        rect.scale(3);
        assert_eq!(rect.width, 15);
        assert_eq!(rect.height, 24);
    }

    #[test]
    fn test_shrink() {
        let mut rect = Rectangle {
            width: 20,
            height: 12,
        };
        rect.shrink(4);
        assert_eq!(rect.width, 5);
        assert_eq!(rect.height, 3);
    }

    #[test]
    fn test_area_after_scale() {
        let mut rect = Rectangle {
            width: 10,
            height: 10,
        };
        rect.scale(2);
        assert_eq!(rect.area(), 400);
    }
}
