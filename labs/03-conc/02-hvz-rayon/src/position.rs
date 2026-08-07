#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
  x: i32,
  y: i32,
}

impl Position {
  pub fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }

  pub fn x(&self) -> i32 {
    self.x
  }

  pub fn y(&self) -> i32 {
    self.y
  }

  pub fn set_x(&mut self, x: i32) {
    self.x = x;
  }

  pub fn set_y(&mut self, y: i32) {
    self.y = y;
  }

  pub fn distance(&self, other: &Position) -> f64 {
    let dx = (self.x - other.x) as f64;
    let dy = (self.y - other.y) as f64;
    (dx * dx + dy * dy).sqrt()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_01_new() {
    let p = Position::new(3, 5);
    assert_eq!(p.x(), 3);
    assert_eq!(p.y(), 5);
  }

  #[test]
  fn test_02_setters() {
    let mut p = Position::new(0, 0);
    p.set_x(7);
    p.set_y(2);
    assert_eq!(p.x(), 7);
    assert_eq!(p.y(), 2);
  }

  #[test]
  fn test_03_distance() {
    let a = Position::new(0, 0);
    let b = Position::new(3, 4);
    let d = a.distance(&b);
    assert!((d - 5.0).abs() < 1e-10, "Expected 5.0, got {}", d);
  }

  #[test]
  fn test_04_distance_zero() {
    let a = Position::new(5, 5);
    let d = a.distance(&a);
    assert!((d - 0.0).abs() < 1e-10, "Expected 0.0, got {}", d);
  }

  #[test]
  fn test_05_distance_negative() {
    let a = Position::new(-1, -1);
    let b = Position::new(2, 3);
    let d = a.distance(&b);
    assert!((d - 5.0).abs() < 1e-10, "Expected 5.0, got {}", d);
  }

  #[test]
  fn test_06_copy() {
    let a = Position::new(1, 2);
    let b = a;
    assert_eq!(a.x(), b.x());
    assert_eq!(a.y(), b.y());
  }
}
