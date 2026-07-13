/// Sum every reading in the slice and return the total.
fn sum_readings(readings: &[f64]) -> f64 {
    // TODO: declare a mutable accumulator starting at 0.0,
    // add each reading to it in a loop, then return it.
    todo!()
}

fn main() {
    let readings = [17.3, 19.1];
    println!("sum = {}", sum_readings(&readings));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(sum_readings(&[]), 0.0);
    }

    #[test]
    fn test_two_values() {
        assert_eq!(sum_readings(&[1.5, 2.5]), 4.0);
    }

    #[test]
    fn test_readings() {
        assert!((sum_readings(&[17.3, 19.1]) - 36.4).abs() < 1e-9);
    }
}
