use std::f64::consts;

trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        f64::from(self.width) * f64::from(self.height)
    }
}

struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        consts::PI * self.radius * self.radius
    }
}

// TODO: write fn total_area(shapes: &[Box<dyn Shape>]) -> f64

fn main() {
    // TODO: create a Vec<Box<dyn Shape>> with at least one Rectangle and one Circle

    // TODO: print the total area
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_area_mixed() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Rectangle::new(3, 4)), // 12.0
            Box::new(Circle::new(1.0)),     // π ≈ 3.14
        ];
        let total = total_area(&shapes);
        assert!((total - (12.0 + consts::PI)).abs() < 0.001);
    }

    #[test]
    fn test_total_area_empty() {
        let shapes: Vec<Box<dyn Shape>> = vec![];
        assert!((total_area(&shapes) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_total_area_three() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Rectangle::new(2, 5)), // 10.0
            Box::new(Circle::new(2.0)),     // π * 4 ≈ 12.57
            Box::new(Rectangle::new(3, 3)), // 9.0
        ];
        let expected = 10.0 + consts::PI * 4.0 + 9.0;
        assert!((total_area(&shapes) - expected).abs() < 0.001);
    }
}
