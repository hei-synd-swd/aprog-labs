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

// TODO: define ShapeEnum with Rectangle and Circle variants
// TODO: implement Shape for ShapeEnum using match

fn main() {
    // TODO: create a Vec<ShapeEnum> with one Rectangle and one Circle
    // TODO: iterate and call .area() on each (no match needed here!)
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_enum_in_vec() {
        let shapes = vec![
            ShapeEnum::Rectangle(Rectangle::new(3, 4)),
            ShapeEnum::Circle(Circle::new(1.0)),
        ];
        let total: f64 = shapes.iter().map(|s| s.area()).sum();
        assert!((total - (12.0 + consts::PI)).abs() < 0.001);
    }
}
