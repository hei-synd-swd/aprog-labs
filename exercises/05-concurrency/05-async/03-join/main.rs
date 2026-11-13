use std::time::Duration;
use tokio::time::{Instant, sleep};

async fn read_sensor(id: u32, delay_ms: u64) -> u32 {
    sleep(Duration::from_millis(delay_ms)).await;
    id * 10
}

async fn read_sequential(delay_a: u64, delay_b: u64) -> u32 {
    let a = read_sensor(1, delay_a).await;
    let b = read_sensor(2, delay_b).await;
    a + b
}

async fn read_concurrent(delay_a: u64, delay_b: u64) -> u32 {
    // TODO: read sensor 1 and sensor 2 concurrently and return the sum
    todo!()
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let sum = read_sequential(300, 200).await;
    println!("sequential: {sum} after {:?}", start.elapsed());

    let start = Instant::now();
    let sum = read_concurrent(300, 200).await;
    println!("concurrent: {sum} after {:?}", start.elapsed());
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // `start_paused` freezes the clock: a sleep completes instantly and moves
    // the clock forward by exactly its duration, so the timings are exact.

    #[tokio::test(start_paused = true)]
    async fn test_returns_the_sum() {
        assert_eq!(read_concurrent(300, 200).await, 30);
    }

    #[tokio::test(start_paused = true)]
    async fn test_takes_as_long_as_the_slower_sensor() {
        let start = Instant::now();
        read_concurrent(300, 200).await;
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(300), "took {elapsed:?}");
        assert!(elapsed < Duration::from_millis(350), "took {elapsed:?}");
    }

    #[tokio::test(start_paused = true)]
    async fn test_slower_sensor_can_be_the_second_one() {
        let start = Instant::now();
        read_concurrent(100, 400).await;
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(400), "took {elapsed:?}");
        assert!(elapsed < Duration::from_millis(450), "took {elapsed:?}");
    }
}
