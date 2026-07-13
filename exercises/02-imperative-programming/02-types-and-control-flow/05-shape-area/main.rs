// TODO: define the `Shape` enum with Circle, Rect, and Triangle variants.

// TODO: implement `area` as a free function.
// fn area(shape: &Shape) -> f64 { ... }

fn main() {
    let c = Shape::Circle { radius: 1.0 };
    let r = Shape::Rect {
        width: 3.0,
        height: 4.0,
    };
    let t = Shape::Triangle {
        base: 4.0,
        height: 3.0,
    };
    println!(
        "circle: {:.4}, rect: {:.4}, triangle: {:.4}",
        area(&c),
        area(&r),
        area(&t)
    );
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let shape = Shape::Circle { radius: 1.0 };
        let diff = (area(&shape) - std::f64::consts::PI).abs();
        assert!(diff < 1e-10, "area too far from PI: {}", diff);
    }

    #[test]
    fn test_rect_area() {
        let shape = Shape::Rect {
            width: 3.0,
            height: 4.0,
        };
        assert!((area(&shape) - 12.0).abs() < 1e-10);
    }

    #[test]
    fn test_triangle_area() {
        let shape = Shape::Triangle {
            base: 4.0,
            height: 3.0,
        };
        assert!((area(&shape) - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_zero_area() {
        assert!((area(&Shape::Circle { radius: 0.0 })).abs() < 1e-10);
    }

    #[test]
    fn test_large_rect() {
        let shape = Shape::Rect {
            width: 1000.0,
            height: 1000.0,
        };
        assert!((area(&shape) - 1_000_000.0).abs() < 1.0);
    }
}
