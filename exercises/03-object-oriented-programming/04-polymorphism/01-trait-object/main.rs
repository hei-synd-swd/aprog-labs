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

// TODO: write fn print_area_dyn(shape: &dyn Shape)

fn main() {
    let rect = Rectangle::new(10, 20);
    let circle = Circle::new(5.0);

    // TODO: create Box<dyn Shape> from rect and circle
    // TODO: call .area() on each and print

    // TODO: call print_area_dyn with references to rect and circle
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_object_rectangle() {
        let rect = Rectangle::new(5, 8);
        let shape: Box<dyn Shape> = Box::new(rect);
        assert!((shape.area() - 40.0).abs() < 0.001);
    }

    #[test]
    fn test_trait_object_circle() {
        let circle = Circle::new(1.0);
        let shape: Box<dyn Shape> = Box::new(circle);
        assert!((shape.area() - consts::PI).abs() < 0.001);
    }

    #[test]
    fn test_dyn_reference() {
        let rect = Rectangle::new(3, 4);
        let shape: &dyn Shape = &rect;
        assert!((shape.area() - 12.0).abs() < 0.001);
    }
}
