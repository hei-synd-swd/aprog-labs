struct Squares {
    n: u32,
    max: u32,
}

impl Squares {
    fn up_to(max: u32) -> Squares {
        Squares { n: 1, max }
    }
}

// TODO: implement `Iterator` for `Squares`
// impl Iterator for Squares { ... }

fn squares_above(max: u32, min: u32) -> Vec<u32> {
    // TODO: collect the squares of 1..=max that are greater than `min`
    todo!()
}

fn main() {
    for square in Squares::up_to(5) {
        println!("{square}");
    }

    let total: u32 = Squares::up_to(3).sum();
    println!("sum = {total}");

    println!("{:?}", squares_above(6, 10));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yields_squares_in_order() {
        let squares: Vec<u32> = Squares::up_to(4).collect();
        assert_eq!(squares, vec![1, 4, 9, 16]);
    }

    #[test]
    fn test_ends_with_none() {
        let mut squares = Squares::up_to(2);
        assert_eq!(squares.next(), Some(1));
        assert_eq!(squares.next(), Some(4));
        assert_eq!(squares.next(), None);
        assert_eq!(squares.next(), None);
    }

    #[test]
    fn test_empty_range() {
        assert_eq!(Squares::up_to(0).count(), 0);
    }

    #[test]
    fn test_adaptors_work() {
        let total: u32 = Squares::up_to(3).sum();
        assert_eq!(total, 14);
        let first_two: Vec<u32> = Squares::up_to(10)
            .map(|square| square + 1)
            .take(2)
            .collect();
        assert_eq!(first_two, vec![2, 5]);
    }

    #[test]
    fn test_squares_above() {
        assert_eq!(squares_above(6, 10), vec![16, 25, 36]);
        assert_eq!(squares_above(3, 100), Vec::<u32>::new());
    }
}
