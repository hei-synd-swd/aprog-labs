// TODO: add a method total_area(&self) -> f64 to Drawing
// TODO: add a method add_shape(&mut self, shape: Box<dyn Shape>)
// (This exercise combines composition with trait objects)

use std::f64::consts;

trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        consts::PI * self.radius * self.radius
    }

    fn name(&self) -> &str {
        "circle"
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        f64::from(self.width) * f64::from(self.height)
    }

    fn name(&self) -> &str {
        "rectangle"
    }
}

// Composition: Drawing has-a collection of shapes
struct Drawing {
    shapes: Vec<Box<dyn Shape>>,
    label: String,
}

impl Drawing {
    fn new(label: &str) -> Self {
        Self {
            shapes: Vec::new(),
            label: label.into(),
        }
    }

    // TODO: add pub fn add_shape(&mut self, shape: Box<dyn Shape>)
    //   Pushes shape into self.shapes

    // TODO: add pub fn total_area(&self) -> f64
    //   Sum of all shape areas

    // TODO: add pub fn describe(&self) -> String
    //   Format: "Drawing '{label}' has {n} shapes, total area: {total:.2}"
}

fn main() {
    let mut drawing = Drawing::new("My Art");
    drawing.add_shape(Box::new(Circle::new(2.0)));
    drawing.add_shape(Box::new(Rectangle::new(3, 4)));
    println!("{}", drawing.describe());
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_drawing() {
        let d = Drawing::new("empty");
        assert!((d.total_area() - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_add_shape() {
        let mut d = Drawing::new("test");
        d.add_shape(Box::new(Circle::new(1.0)));
        assert!((d.total_area() - consts::PI).abs() < 0.001);
    }

    #[test]
    fn test_multiple_shapes() {
        let mut d = Drawing::new("multi");
        d.add_shape(Box::new(Circle::new(2.0))); // ~12.57
        d.add_shape(Box::new(Rectangle::new(3, 4))); // 12.0
        d.add_shape(Box::new(Rectangle::new(2, 2))); // 4.0
        let expected = consts::PI * 4.0 + 12.0 + 4.0;
        assert!((d.total_area() - expected).abs() < 0.001);
    }

    #[test]
    fn test_describe() {
        let mut d = Drawing::new("test");
        d.add_shape(Box::new(Rectangle::new(3, 4)));
        let desc = d.describe();
        assert!(desc.contains("test"));
        assert!(desc.contains("1 shapes"));
    }
}
