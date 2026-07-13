// TODO: implement distance(&self, other: &Point) -> f64
// TODO: add a named constructor origin() -> Self
// TODO: add a method translate(&mut self, dx: f64, dy: f64)

use std::f64;

/// A 2D point with private coordinates.
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    // TODO: add pub fn origin() -> Self that returns Point at (0.0, 0.0)

    // Getters - Rust idiom, no get_ prefix
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    // TODO: add pub fn distance(&self, other: &Point) -> f64
    // Formula: sqrt((x2 - x1)² + (y2 - y1)²)

    // TODO: add pub fn translate(&mut self, dx: f64, dy: f64)
    // Adds dx to x, dy to y
}

fn main() {
    let a = Point::new(1.0, 2.0);
    let b = Point::new(4.0, 6.0);
    println!("Point a: ({}, {})", a.x(), a.y());
    println!("Point b: ({}, {})", b.x(), b.y());
    println!("Distance: {:.2}", a.distance(&b));

    let mut p = Point::origin();
    p.translate(3.0, 4.0);
    println!("Translated origin: ({}, {})", p.x(), p.y());
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_getters() {
        let p = Point::new(3.0, 4.0);
        assert!((p.x() - 3.0).abs() < 0.001);
        assert!((p.y() - 4.0).abs() < 0.001);
    }

    #[test]
    fn test_origin() {
        let p = Point::origin();
        assert!((p.x() - 0.0).abs() < 0.001);
        assert!((p.y() - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_distance() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(3.0, 4.0);
        assert!((a.distance(&b) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_distance_same_point() {
        let p = Point::new(2.0, 3.0);
        assert!((p.distance(&p) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_translate() {
        let mut p = Point::new(1.0, 1.0);
        p.translate(2.0, 3.0);
        assert!((p.x() - 3.0).abs() < 0.001);
        assert!((p.y() - 4.0).abs() < 0.001);
    }

    #[test]
    fn test_fields_private() {
        let p = Point::new(1.0, 2.0);
        // p.x = 5.0;  // ✗ compile error if uncommented
        let _ = p.x(); // must use getter
    }
}
