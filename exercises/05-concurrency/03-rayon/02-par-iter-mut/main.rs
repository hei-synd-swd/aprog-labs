use rayon::prelude::*;

fn calibrate(readings: &mut [f64], offset: f64) {
    // TODO: add `offset` to every reading, in place, with `par_iter_mut`
    todo!()
}

fn hottest(readings: &[f64]) -> f64 {
    // TODO: return the largest reading with a parallel `reduce`
    todo!()
}

fn main() {
    let mut readings: Vec<f64> = (0..1_000_000).map(|i| f64::from(i % 400) / 10.0).collect();

    calibrate(&mut readings, 0.5);

    println!("first readings: {:?}", &readings[..4]);
    println!("hottest:        {}", hottest(&readings));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibrate_adds_the_offset() {
        let mut readings = vec![20.0, 21.5, 19.25];
        calibrate(&mut readings, 0.5);
        assert_eq!(readings, vec![20.5, 22.0, 19.75]);
    }

    #[test]
    fn test_calibrate_with_negative_offset() {
        let mut readings = vec![1.0, 2.0];
        calibrate(&mut readings, -1.0);
        assert_eq!(readings, vec![0.0, 1.0]);
    }

    #[test]
    fn test_calibrate_large_input() {
        let mut readings = vec![0.0; 10_000];
        calibrate(&mut readings, 1.5);
        assert!(readings.iter().all(|&reading| reading == 1.5));
    }

    #[test]
    fn test_hottest() {
        assert_eq!(hottest(&[20.5, 22.0, 19.75]), 22.0);
    }

    #[test]
    fn test_hottest_below_zero() {
        assert_eq!(hottest(&[-3.0, -1.5, -7.0]), -1.5);
    }

    #[test]
    fn test_hottest_of_nothing() {
        assert_eq!(hottest(&[]), f64::NEG_INFINITY);
    }

    #[test]
    fn test_hottest_large_input() {
        let readings: Vec<f64> = (0..10_000).map(f64::from).collect();
        assert_eq!(hottest(&readings), 9999.0);
    }
}
