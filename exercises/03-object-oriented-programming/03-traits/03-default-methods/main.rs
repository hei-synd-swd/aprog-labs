trait Describe {
    fn describe(&self) -> String;

    fn introduction(&self) -> String {
        format!("I am a {}", self.describe())
    }
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

// TODO: implement Describe for Rectangle
//   - describe() returns "Rectangle: {width} x {height}"
//   - introduction() uses the default (no need to override)

struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }
}

// TODO: implement Describe for Circle
//   - describe() returns "Circle: radius {radius}"
//   - override introduction() to return "I am a circle with radius {radius}"

fn main() {
    let rect = Rectangle::new(10, 20);
    let circle = Circle::new(5.0);
    println!("{}", rect.introduction());
    println!("{}", circle.introduction());
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_uses_default() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.introduction(), "I am a Rectangle: 10 x 20");
    }

    #[test]
    fn test_circle_overrides() {
        let circle = Circle::new(5.0);
        assert_eq!(circle.introduction(), "I am a circle with radius 5");
    }
}
