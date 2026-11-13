// TODO: implement `reverse`
// Swap arr[i] with arr[len - 1 - i] for i in 0..len/2
fn reverse(arr: &mut [i32]) {
    todo!()
}

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
    fn test_several_elements() {
        let mut v = vec![1, 2, 3, 4];
        reverse(&mut v);
        assert_eq!(v, vec![4, 3, 2, 1]);
    }

    #[test]
    fn test_single_element() {
        let mut v = vec![1];
        reverse(&mut v);
        assert_eq!(v, vec![1]);
    }

    #[test]
    fn test_empty() {
        let mut v: Vec<i32> = vec![];
        reverse(&mut v);
        assert_eq!(v, vec![]);
    }
}
