ouse std::sync::mpsc::{self, Receiver};
use std::thread;

fn start_worker(n: u64) -> Receiver<u64> {
    // TODO: create a channel, spawn a thread that sends `n * n`,
    //       and return the receiver
    todo!()
}

fn main() {
    let rx = start_worker(12);

    println!("main waits for the worker...");
    match rx.recv() {
        Ok(square) => println!("the worker sent {square}"),
        Err(_) => println!("the worker hung up without sending"),
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
    fn test_worker_sends_the_square() {
        let rx = start_worker(7);
        assert_eq!(rx.recv(), Ok(49));
    }

    #[test]
    fn test_square_of_zero() {
        let rx = start_worker(0);
        assert_eq!(rx.recv(), Ok(0));
    }

    #[test]
    fn test_channel_closes_after_one_value() {
        let rx = start_worker(3);
        assert_eq!(rx.recv(), Ok(9));
        assert!(rx.recv().is_err());
    }

    #[test]
    fn test_every_worker_has_its_own_channel() {
        let first = start_worker(2);
        let second = start_worker(5);
        assert_eq!(second.recv(), Ok(25));
        assert_eq!(first.recv(), Ok(4));
    }
}
