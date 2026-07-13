// TODO: define a Circle struct with a field radius: f64
// TODO: implement the following methods:
//   - fn area(&self) -> f64            — returns π × r²
//   - fn circumference(&self) -> f64   — returns 2 × π × r
//   - fn grow(&mut self, factor: f64)  — multiplies radius by factor

fn main() {
    // TODO: create a Circle with radius 5.0
    // TODO: print its area and circumference
    // TODO: call grow(2.0) and print the new radius and area
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let c = Circle { radius: 1.0 };
        assert!((c.area() - std::f64::consts::PI).abs() < 0.0001);
    }

    #[test]
    fn test_circumference() {
        let c = Circle { radius: 1.0 };
        assert!((c.circumference() - 2.0 * std::f64::consts::PI).abs() < 0.0001);
    }

    #[test]
    fn test_grow() {
        let mut c = Circle { radius: 3.0 };
        c.grow(2.0);
        assert!((c.radius - 6.0).abs() < 0.0001);
    }

    #[test]
    fn test_area_after_grow() {
        let mut c = Circle { radius: 2.0 };
        c.grow(3.0);
        let expected = std::f64::consts::PI * 36.0;
        assert!((c.area() - expected).abs() < 0.0001);
    }
}
