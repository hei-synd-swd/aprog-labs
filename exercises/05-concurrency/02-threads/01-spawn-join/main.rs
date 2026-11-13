use std::thread::{self, ThreadId};

fn sum_in_thread(numbers: Vec<u64>) -> (u64, ThreadId) {
    // TODO: spawn a thread that sums `numbers`, then join it and return its value
    todo!()
}

fn main() {
    let (sum, worker_id) = sum_in_thread(vec![1, 2, 3, 4]);
    println!("sum = {sum}, computed by {worker_id:?}");
    println!("main runs on {:?}", thread::current().id());
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_is_correct() {
        let (sum, _) = sum_in_thread(vec![1, 2, 3, 4]);
        assert_eq!(sum, 10);
    }

    #[test]
    fn test_empty_input() {
        let (sum, _) = sum_in_thread(vec![]);
        assert_eq!(sum, 0);
    }

    #[test]
    fn test_runs_in_another_thread() {
        let (_, worker_id) = sum_in_thread(vec![5, 5]);
        assert_ne!(worker_id, thread::current().id());
    }

    #[test]
    fn test_each_call_spawns_a_new_thread() {
        let (_, first) = sum_in_thread(vec![1]);
        let (_, second) = sum_in_thread(vec![2]);
        assert_ne!(first, second);
    }
}
