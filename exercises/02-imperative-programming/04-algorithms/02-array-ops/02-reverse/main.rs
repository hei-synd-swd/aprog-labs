// TODO: `reverse(arr: &mut [i32])`
// Swap arr[i] with arr[len - 1 - i] for i in 0..len/2
// fn reverse(arr: &mut [i32]) { ... }

fn main() {
    let mut v = vec![1, 2, 3, 4];
    reverse(&mut v);
    println!("{:?}", v);
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        let mut v = vec![1, 2, 3, 4];
        reverse(&mut v);
        assert_eq!(v, vec![4, 3, 2, 1]);

        let mut v = vec![1];
        reverse(&mut v);
        assert_eq!(v, vec![1]);

        let mut v: Vec<i32> = vec![];
        reverse(&mut v);
        assert_eq!(v, vec![]);
    }
}
