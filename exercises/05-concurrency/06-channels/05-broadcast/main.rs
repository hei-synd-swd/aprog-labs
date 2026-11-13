use tokio::sync::broadcast;

async fn announce(messages: Vec<String>, subscribers: usize) -> Vec<Vec<String>> {
    // TODO: start `subscribers` tasks that each collect every message from a
    //       broadcast channel, send all messages, close the channel and
    //       return the inbox of every subscriber
    todo!()
}

#[tokio::main]
async fn main() {
    let messages = vec![
        String::from("doors open"),
        String::from("show starts"),
        String::from("intermission"),
    ];

    let inboxes = announce(messages, 3).await;

    for (subscriber, inbox) in inboxes.iter().enumerate() {
        println!("subscriber {subscriber} received {inbox:?}");
    }
}

//==============================================================================
//                           EXERCISE UNIT TESTS
//                       DO NOT EDIT BELOW THIS LINE
//==============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    // Fails if `announce` is not done after 3 seconds of (paused) time.
    // An `announce` that waits forever would otherwise block the test.
    async fn announce_in_time(messages: Vec<String>, subscribers: usize) -> Vec<Vec<String>> {
        tokio::time::timeout(Duration::from_secs(3), announce(messages, subscribers))
            .await
            .expect("`announce` waits forever: is a `Sender` still alive?")
    }

    fn messages(list: &[&str]) -> Vec<String> {
        list.iter().map(|message| message.to_string()).collect()
    }

    #[tokio::test(start_paused = true)]
    async fn test_every_subscriber_gets_every_message() {
        let inboxes = announce_in_time(messages(&["a", "b", "c"]), 3).await;
        assert_eq!(inboxes.len(), 3);
        for inbox in inboxes {
            assert_eq!(inbox, vec!["a", "b", "c"]);
        }
    }

    #[tokio::test(start_paused = true)]
    async fn test_one_subscriber() {
        let inboxes = announce_in_time(messages(&["hello"]), 1).await;
        assert_eq!(inboxes, vec![vec!["hello"]]);
    }

    #[tokio::test(start_paused = true)]
    async fn test_no_subscriber() {
        let inboxes = announce_in_time(messages(&["a", "b"]), 0).await;
        assert!(inboxes.is_empty());
    }

    #[tokio::test(start_paused = true)]
    async fn test_no_message() {
        let inboxes = announce_in_time(Vec::new(), 2).await;
        assert_eq!(inboxes, vec![Vec::<String>::new(), Vec::new()]);
    }

    #[tokio::test(start_paused = true)]
    async fn test_many_messages_are_not_lost() {
        let all: Vec<String> = (0..200).map(|i| format!("message {i}")).collect();
        let inboxes = announce_in_time(all.clone(), 2).await;
        assert_eq!(inboxes, vec![all.clone(), all]);
    }
}
