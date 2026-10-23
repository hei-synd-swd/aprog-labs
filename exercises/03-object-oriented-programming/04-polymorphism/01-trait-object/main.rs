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

// TODO: return the area formatted as "Area: X.XX" (two decimals),
// using the `&dyn Shape` borrow form of a trait object.
fn describe(shape: &dyn Shape) -> String {
    let _ = shape;
    todo!("return the area formatted as the string Area: X.XX")
}

// TODO: sum the area of every shape in the slice. Each element is an owned
// `Box<dyn Shape>` trait object - call .area() through dynamic dispatch.
fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    let _ = shapes;
    todo!("sum shape.area() over the slice")
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle::new(10, 20)),
        Box::new(Circle::new(5.0)),
    ];

    for shape in &shapes {
        println!("{}", describe(shape.as_ref()));
    }
    println!("Total area: {:.2}", total_area(&shapes));
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
        let rect = Rectangle::new(3, 4);
        assert_eq!(describe(&rect), "Area: 12.00");
    }

    #[test]
    fn test_describe_circle() {
        let circle = Circle::new(1.0);
        assert_eq!(describe(&circle), "Area: 3.14");
    }

    #[test]
    fn test_total_area() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Rectangle::new(5, 8)),
            Box::new(Circle::new(1.0)),
        ];
        assert!((total_area(&shapes) - (40.0 + consts::PI)).abs() < 0.001);
    }

    #[test]
    fn test_total_area_empty() {
        let shapes: Vec<Box<dyn Shape>> = vec![];
        assert_eq!(total_area(&shapes), 0.0);
    }
}
