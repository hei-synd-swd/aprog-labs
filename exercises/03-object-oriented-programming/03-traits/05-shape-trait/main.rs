struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
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

// TODO: define the Shape trait with area(&self) -> f64

// TODO: implement Shape for Rectangle
//   (convert width/height to f64 with f64::from())

// TODO: implement Shape for Circle
//   (use std::f64::consts::PI)

// TODO: write fn compare_areas(a: &impl Shape, b: &impl Shape) -> String

fn main() {
    let rect = Rectangle::new(10, 20);
    let circle = Circle::new(7.0);
    println!("{}", compare_areas(&rect, &circle));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle::new(5, 8);
        assert!((rect.area() - 40.0).abs() < 0.001);
    }

    #[test]
    fn test_circle_area() {
        let circle = Circle::new(1.0);
        assert!((circle.area() - std::f64::consts::PI).abs() < 0.001);
    }

    #[test]
    fn test_compare_first_larger() {
        let big = Rectangle::new(100, 100);
        let small = Circle::new(1.0);
        assert_eq!(compare_areas(&big, &small), "First is larger");
    }

    #[test]
    fn test_compare_equal() {
        let a = Rectangle::new(10, 10);
        let b = Rectangle::new(10, 10);
        assert_eq!(compare_areas(&a, &b), "Equal areas");
    }
}
