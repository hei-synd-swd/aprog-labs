use std::ops::Add;

// TODO: derive `Debug`, `Clone`, `Copy`, `PartialEq`, `PartialOrd` and `Default`
struct Money {
    cents: u32,
}

// TODO: implement `Add` for `Money`, so that `a + b` adds the cents
// impl Add for Money { ... }

fn total(prices: &[Money]) -> Money {
    // TODO: start at `Money::default()` and add every price with `+`
    todo!()
}

fn main() {
    let coffee = Money { cents: 450 };
    let cake = Money { cents: 620 };

    println!("{:?}", coffee + cake);
    println!("coffee is cheaper: {}", coffee < cake);
    println!("{:?}", total(&[coffee, cake, coffee]));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_zero() {
        assert_eq!(Money::default(), Money { cents: 0 });
    }

    #[test]
    fn test_copy_keeps_the_original_usable() {
        let original = Money { cents: 5 };
        let copy = original;
        assert_eq!(original, copy);
    }

    #[test]
    fn test_debug_format() {
        assert_eq!(format!("{:?}", Money { cents: 5 }), "Money { cents: 5 }");
    }

    #[test]
    fn test_ordering() {
        assert!(Money { cents: 450 } < Money { cents: 620 });
        assert!(Money { cents: 7 } >= Money { cents: 7 });
    }

    #[test]
    fn test_add() {
        let sum = Money { cents: 450 } + Money { cents: 620 };
        assert_eq!(sum, Money { cents: 1070 });
    }

    #[test]
    fn test_total() {
        let prices = [
            Money { cents: 450 },
            Money { cents: 620 },
            Money { cents: 450 },
        ];
        assert_eq!(total(&prices), Money { cents: 1520 });
        assert_eq!(total(&[]), Money::default());
    }
}
