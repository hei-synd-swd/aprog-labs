trait Describe {
    fn describe(&self) -> String;
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

struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }
}

// TODO: implement Describe for Rectangle
//   describe() returns "Rectangle: {width} x {height}"

// TODO: implement Describe for Circle
//   describe() returns "Circle: radius {radius}"

fn main() {
    let rect = Rectangle::new(10, 20);
    let circle = Circle::new(5.0);
    println!("{}", rect.describe());
    println!("{}", circle.describe());
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
        let rect = Rectangle::new(5, 8);
        assert_eq!(rect.describe(), "Rectangle: 5 x 8");
    }

    #[test]
    fn test_describe_circle() {
        let circle = Circle::new(3.0);
        assert_eq!(circle.describe(), "Circle: radius 3");
    }
}
