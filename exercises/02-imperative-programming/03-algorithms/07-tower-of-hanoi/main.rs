// TODO: `hanoi(n: u32, from: &str, to: &str, aux: &str)`
// prints each move like "Move disk from A to C"
// fn hanoi(n: u32, from: &str, to: &str, aux: &str) { ... }

// TODO: `hanoi_count(n: u32) -> u32` — return number of moves (2ⁿ − 1)
// fn hanoi_count(n: u32) -> u32 { ... }

fn main() {
    println!("Moving 4 disks:");
    hanoi(4, "A", "C", "B");
    println!("Total moves: {}", hanoi_count(4));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hanoi_count_1() {
        assert_eq!(hanoi_count(1), 1);
    }

    #[test]
    fn test_hanoi_count_3() {
        assert_eq!(hanoi_count(3), 7);
    }

    #[test]
    fn test_hanoi_count_5() {
        assert_eq!(hanoi_count(5), 31);
    }

    #[test]
    fn test_hanoi_count_0() {
        assert_eq!(hanoi_count(0), 0);
    }
}
