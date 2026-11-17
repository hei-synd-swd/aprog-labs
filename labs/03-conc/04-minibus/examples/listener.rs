//! Listener vs subscriber: two ways to observe the same channel.
//!
//! - listener: sync callback, runs in the PUBLISHER's thread, no queue.
//!   Cheap, but a slow listener blocks the publisher.
//! - subscriber: own queue, consumed in its OWN thread.
//!   The publisher never waits for it.
//!
//! Run: cargo run --example listener

use minibus::Channel;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let ch: Channel<String> = Channel::new("events");
    println!("[main]       thread {:?}", thread::current().id());

    // A sync listener: note it reports the PUBLISHER's thread id,
    // and it is slow on purpose (10 ms) to show that it blocks publish.
    ch.listen(|msg: &String| {
        thread::sleep(Duration::from_millis(10));
        println!(
            "[listener]   got {msg:?} in thread {:?} (publisher's!)",
            thread::current().id()
        );
    });

    // A subscriber: its own queue, drained in its own thread.
    let sub = ch.subscribe();
    let worker = thread::spawn(move || {
        for msg in sub {
            println!(
                "[subscriber] got {msg:?} in thread {:?} (its own)",
                thread::current().id()
            );
        }
    });

    // Publish and time it: the listener's 10 ms is paid BY the publisher,
    // the subscriber costs it (almost) nothing.
    for i in 1..=3 {
        let t = Instant::now();
        ch.publish(format!("event {i}"));
        println!("[main]       publish took {:?}", t.elapsed());
    }

    drop(ch); // close the channel so the subscriber's loop ends
    worker.join().unwrap();
    println!("[main]       done — same messages, two very different costs.");
}