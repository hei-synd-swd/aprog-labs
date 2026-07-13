struct Celsius(f64);
struct Fahrenheit(f64);

// TODO: implement `From<Celsius> for Fahrenheit`
// using the formula: °F = °C * 9 / 5 + 32
impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        // TODO: apply °F = °C * 9 / 5 + 32 and return a Fahrenheit.
        todo!()
    }
}

fn main() {
    let boiling = Fahrenheit::from(Celsius(100.0));
    println!("{}", boiling.0);
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_zero() {
        assert!((Fahrenheit::from(Celsius(0.0)).0 - 32.0).abs() < 1e-9);
    }

    #[test]
    fn test_from_boiling() {
        assert!((Fahrenheit::from(Celsius(100.0)).0 - 212.0).abs() < 1e-9);
    }

    #[test]
    fn test_into_minus_forty() {
        let f: Fahrenheit = Celsius(-40.0).into();
        assert!((f.0 - (-40.0)).abs() < 1e-9);
    }
}
