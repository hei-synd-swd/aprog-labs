// TODO: add a set_celsius method that rejects values below -273.15
// TODO: add a getter for the field (no get_ prefix!)
// TODO: add a to_fahrenheit(&self) -> f64 conversion method

pub struct Temperature {
    celsius: f64, // private field
}

impl Temperature {
    pub fn new(celsius: f64) -> Self {
        Self { celsius }
    }

    // TODO: add pub fn celsius(&self) -> f64 { ... }

    // TODO: add pub fn set_celsius(&mut self, value: f64)
    //   If value >= -273.15, update celsius; otherwise do nothing

    // TODO: add pub fn to_fahrenheit(&self) -> f64
    //   Formula: celsius * 9.0 / 5.0 + 32.0
}

fn main() {
    let mut t = Temperature::new(20.0);
    println!("Celsius: {}", t.celsius());
    println!("Fahrenheit: {:.1}", t.to_fahrenheit());

    t.set_celsius(100.0);
    println!("Boiling: {}°C = {:.1}°F", t.celsius(), t.to_fahrenheit());

    t.set_celsius(-300.0); // should be rejected
    println!("Still: {}°C", t.celsius()); // still 100
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_getter() {
        let t = Temperature::new(25.0);
        assert!((t.celsius() - 25.0).abs() < 0.001);
    }

    #[test]
    fn test_set_celsius_valid() {
        let mut t = Temperature::new(0.0);
        t.set_celsius(30.0);
        assert!((t.celsius() - 30.0).abs() < 0.001);
    }

    #[test]
    fn test_set_celsius_invalid() {
        let mut t = Temperature::new(10.0);
        t.set_celsius(-300.0);
        assert!((t.celsius() - 10.0).abs() < 0.001); // unchanged
    }

    #[test]
    fn test_absolute_zero_boundary() {
        let mut t = Temperature::new(0.0);
        t.set_celsius(-273.15);
        assert!((t.celsius() - (-273.15)).abs() < 0.001);
    }

    #[test]
    fn test_to_fahrenheit() {
        let t = Temperature::new(0.0);
        assert!((t.to_fahrenheit() - 32.0).abs() < 0.001);
    }

    #[test]
    fn test_to_fahrenheit_boiling() {
        let t = Temperature::new(100.0);
        assert!((t.to_fahrenheit() - 212.0).abs() < 0.001);
    }

    #[test]
    fn test_field_is_private() {
        let t = Temperature::new(5.0);
        // t.celsius;  // ✗ compile error if uncommented
        let _ = t.celsius(); // must use getter
    }
}
