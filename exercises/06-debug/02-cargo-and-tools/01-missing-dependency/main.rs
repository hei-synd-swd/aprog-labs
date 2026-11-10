// A tiny dice roller. This file is correct - do not edit it.

use rand::random_range;

fn main() {
    let roll: u32 = random_range(1..=6);
    println!("rolled: {roll}");
}

//==============================================================================
//                      EXERCISE VERIFICATION - DO NOT EDIT
//==============================================================================
#[cfg(test)]
mod verify {
    #[test]
    fn manifest_declares_rand() {
        let roll: u32 = rand::random_range(1..=6);
        assert!((1..=6).contains(&roll));
    }
}
