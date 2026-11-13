use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

struct Account {
    id: u32,
    balance: Mutex<i64>,
}

impl Account {
    fn new(id: u32, balance: i64) -> Account {
        Account {
            id,
            balance: Mutex::new(balance),
        }
    }

    fn lock(&self) -> MutexGuard<'_, i64> {
        self.balance
            .lock()
            .expect("no thread panicked while holding the lock")
    }

    fn balance(&self) -> i64 {
        *self.lock()
    }
}

fn simulate_work() {
    thread::sleep(Duration::from_millis(10));
}

fn transfer(from: &Account, to: &Account, amount: i64) {
    // TODO: this always locks `from` first. Two transfers in opposite
    //       directions deadlock. Lock the account with the lower id first.
    let mut from_balance = from.lock();
    simulate_work();
    let mut to_balance = to.lock();

    *from_balance -= amount;
    *to_balance += amount;
}

fn main() {
    let alice = Arc::new(Account::new(1, 100));
    let bob = Arc::new(Account::new(2, 100));

    let (a, b) = (Arc::clone(&alice), Arc::clone(&bob));
    let to_bob = thread::spawn(move || transfer(&a, &b, 30));

    let (a, b) = (Arc::clone(&alice), Arc::clone(&bob));
    let to_alice = thread::spawn(move || transfer(&b, &a, 50));

    to_bob.join().expect("the transfer does not panic");
    to_alice.join().expect("the transfer does not panic");

    println!("alice: {}, bob: {}", alice.balance(), bob.balance());
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    // Runs `job` in a thread and fails if it is not done after 3 seconds.
    // A deadlocked job would otherwise block the test forever.
    fn finishes_in_time(job: impl FnOnce() + Send + 'static) -> bool {
        let (done_tx, done_rx) = mpsc::channel();
        thread::spawn(move || {
            job();
            let _ = done_tx.send(());
        });
        done_rx.recv_timeout(Duration::from_secs(3)).is_ok()
    }

    #[test]
    fn test_transfer_moves_money() {
        let alice = Account::new(1, 100);
        let bob = Account::new(2, 100);
        transfer(&alice, &bob, 30);
        assert_eq!(alice.balance(), 70);
        assert_eq!(bob.balance(), 130);
    }

    #[test]
    fn test_transfer_in_the_other_direction() {
        let alice = Account::new(1, 100);
        let bob = Account::new(2, 100);
        transfer(&bob, &alice, 50);
        assert_eq!(alice.balance(), 150);
        assert_eq!(bob.balance(), 50);
    }

    #[test]
    fn test_opposite_transfers_do_not_deadlock() {
        let alice = Arc::new(Account::new(1, 100));
        let bob = Arc::new(Account::new(2, 100));

        let (a, b) = (Arc::clone(&alice), Arc::clone(&bob));
        let done = finishes_in_time(move || {
            let (a1, b1) = (Arc::clone(&a), Arc::clone(&b));
            let to_bob = thread::spawn(move || {
                for _ in 0..5 {
                    transfer(&a1, &b1, 10);
                }
            });
            let to_alice = thread::spawn(move || {
                for _ in 0..5 {
                    transfer(&b, &a, 20);
                }
            });
            to_bob.join().unwrap();
            to_alice.join().unwrap();
        });

        assert!(done, "deadlock: the transfers did not finish in 3 seconds");
        assert_eq!(alice.balance(), 150);
        assert_eq!(bob.balance(), 50);
    }

    #[test]
    fn test_ring_of_transfers_does_not_deadlock() {
        let alice = Arc::new(Account::new(1, 100));
        let bob = Arc::new(Account::new(2, 100));
        let carol = Arc::new(Account::new(3, 100));

        let (a, b, c) = (Arc::clone(&alice), Arc::clone(&bob), Arc::clone(&carol));
        let done = finishes_in_time(move || {
            let pairs = [
                (Arc::clone(&a), Arc::clone(&b)),
                (Arc::clone(&b), Arc::clone(&c)),
                (Arc::clone(&c), Arc::clone(&a)),
            ];
            let mut handles = Vec::new();
            for (from, to) in pairs {
                handles.push(thread::spawn(move || {
                    for _ in 0..3 {
                        transfer(&from, &to, 10);
                    }
                }));
            }
            for handle in handles {
                handle.join().unwrap();
            }
        });

        assert!(done, "deadlock: the transfers did not finish in 3 seconds");
        assert_eq!(alice.balance() + bob.balance() + carol.balance(), 300);
    }
}
