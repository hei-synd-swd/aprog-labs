use std::time::Duration;
use tokio::time::sleep;

async fn load_user(id: u32) -> String {
    sleep(Duration::from_millis(50)).await;
    format!("user-{id}")
}

// TODO: make this function asynchronous
fn greeting(id: u32) -> String {
    // TODO: load the name with `load_user` and wait for it without blocking
    let name = String::from("?");
    format!("Hello, {name}!")
}

#[tokio::main]
async fn main() {
    println!("{}", greeting(1).await);
    println!("{}", greeting(2).await);
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
    async fn test_greets_the_user() {
        assert_eq!(greeting(7).await, "Hello, user-7!");
    }

    #[tokio::test(start_paused = true)]
    async fn test_greets_another_user() {
        assert_eq!(greeting(42).await, "Hello, user-42!");
    }

    #[tokio::test(start_paused = true)]
    async fn test_waits_for_the_user_to_load() {
        let start = Instant::now();
        greeting(1).await;
        assert!(start.elapsed() >= Duration::from_millis(50));
    }
}
