use std::time::Duration;
use tokio::sync::oneshot;
use tokio::time::sleep;

async fn slow_square(n: u64) -> u64 {
    sleep(Duration::from_millis(100)).await;
    n * n
}

fn start_job(n: u64) -> oneshot::Receiver<u64> {
    // TODO: spawn a task that computes `slow_square(n)` and sends the result
    //       through a oneshot channel. Return the receiver.
    todo!()
}

#[tokio::main]
async fn main() {
    let reply = start_job(12);
    println!("job started, main is free to do other things");

    match reply.await {
        Ok(square) => println!("the job answered {square}"),
        Err(_) => println!("the job ended without an answer"),
    }
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Instant;

    // `start_paused` freezes the clock: a sleep completes instantly and moves
    // the clock forward by exactly its duration.

    #[tokio::test(start_paused = true)]
    async fn test_reply_carries_the_square() {
        assert_eq!(start_job(7).await, Ok(49));
    }

    #[tokio::test(start_paused = true)]
    async fn test_start_job_does_not_wait() {
        let start = Instant::now();
        let _reply = start_job(7);
        assert!(start.elapsed() < Duration::from_millis(100));
    }

    #[tokio::test(start_paused = true)]
    async fn test_reply_arrives_after_the_computation() {
        let start = Instant::now();
        let reply = start_job(3);
        assert_eq!(reply.await, Ok(9));
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(100), "took {elapsed:?}");
        assert!(elapsed < Duration::from_millis(150), "took {elapsed:?}");
    }

    #[tokio::test(start_paused = true)]
    async fn test_jobs_run_concurrently() {
        let start = Instant::now();
        let first = start_job(2);
        let second = start_job(5);
        assert_eq!(first.await, Ok(4));
        assert_eq!(second.await, Ok(25));
        let elapsed = start.elapsed();
        assert!(elapsed < Duration::from_millis(150), "took {elapsed:?}");
    }
}
