use std::sync::mpsc;
use std::thread;

fn gather(workers: u32, per_worker: u32) -> Vec<u32> {
    // TODO: every worker `w` in `0..workers` sends `w * 100 + i` for each `i`
    //       in `0..per_worker` into one shared channel. Receive all numbers
    //       until the channel is closed and return them sorted.
    todo!()
}

fn main() {
    println!("{:?}", gather(3, 2));
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    // Runs `gather` in a thread and fails if no result arrives in 3 seconds.
    // A `gather` that waits forever would otherwise block the test.
    fn gather_in_time(workers: u32, per_worker: u32) -> Vec<u32> {
        let (done_tx, done_rx) = mpsc::channel();
        thread::spawn(move || {
            let _ = done_tx.send(gather(workers, per_worker));
        });
        done_rx
            .recv_timeout(Duration::from_secs(3))
            .expect("`gather` returned no result: it panicked, or it waits forever because a `Sender` is still alive")
    }

    #[test]
    fn test_three_workers() {
        assert_eq!(gather_in_time(3, 2), vec![0, 1, 100, 101, 200, 201]);
    }

    #[test]
    fn test_one_worker() {
        assert_eq!(gather_in_time(1, 3), vec![0, 1, 2]);
    }

    #[test]
    fn test_no_worker() {
        assert!(gather_in_time(0, 5).is_empty());
    }

    #[test]
    fn test_workers_that_send_nothing() {
        assert!(gather_in_time(4, 0).is_empty());
    }

    #[test]
    fn test_many_workers() {
        let numbers = gather_in_time(20, 50);
        assert_eq!(numbers.len(), 1000);
        assert!(numbers.windows(2).all(|pair| pair[0] < pair[1]));
    }
}
