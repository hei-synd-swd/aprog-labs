use std::time::Duration;
use tokio::time::Instant;

async fn download(id: u32) -> String {
    // TODO: this blocks the whole thread. Wait without blocking instead.
    std::thread::sleep(Duration::from_millis(100));
    format!("file-{id}")
}

async fn download_all(ids: Vec<u32>) -> Vec<String> {
    // TODO: spawn one task per id, then await all tasks and return the
    //       file names in input order
    todo!()
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let files = download_all(vec![1, 2, 3, 4, 5]).await;
    println!("{files:?} after {:?}", start.elapsed());
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // `start_paused` freezes the clock: a `tokio::time::sleep` completes
    // instantly and moves the clock forward by exactly its duration.
    // A blocking `std::thread::sleep` does not move this clock at all.

    #[tokio::test(start_paused = true)]
    async fn test_download_returns_the_file_name() {
        assert_eq!(download(7).await, "file-7");
    }

    #[tokio::test(start_paused = true)]
    async fn test_download_does_not_block() {
        let start = Instant::now();
        download(7).await;
        let elapsed = start.elapsed();
        assert!(
            elapsed >= Duration::from_millis(100),
            "`download` must wait with `tokio::time::sleep`"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn test_results_are_in_input_order() {
        let files = download_all(vec![3, 1, 2]).await;
        assert_eq!(files, vec!["file-3", "file-1", "file-2"]);
    }

    #[tokio::test(start_paused = true)]
    async fn test_empty_input() {
        assert!(download_all(Vec::new()).await.is_empty());
    }

    #[tokio::test(start_paused = true)]
    async fn test_downloads_run_concurrently() {
        let start = Instant::now();
        download_all(vec![1, 2, 3, 4, 5]).await;
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(100), "took {elapsed:?}");
        assert!(elapsed < Duration::from_millis(150), "took {elapsed:?}");
    }
}
