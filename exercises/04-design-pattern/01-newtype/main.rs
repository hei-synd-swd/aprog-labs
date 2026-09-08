use std::fmt::Display;

#[derive(Debug, PartialEq)]
// TODO: Add Feet struct holding a `f64`
#[derive(Debug, PartialEq)]
// TODO: Add Meters struct holding a `f64`

fn main() {
    // TODO: Create instance of `Feet`, convert it, print with Debug & Display Formatter
}

// TODO: Add the convertion function here
// Note that `From<Feet>` is implemented further down and think about what that means for you
// fn feet_to_meters(d: Feet) -> Meters {
// ..
// }

impl From<Feet> for Meters {
    fn from(v: Feet) -> Self {
        Meters(v.0 * 0.3048)
    }
}

impl Display for Meters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Meters!", self.0)
    }
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_meters_instance() {
        let _d = Meters(1234.0);
    }
    #[test]
    fn test_new_feet_instance() {
        let _d = Feet(1234.0);
    }
    #[test]
    fn test_feet_to_meters() {
        let d = Feet(10.0);
        let res = feet_to_meters(d);
        assert_eq!(res, Meters(3.048))
    }
}
