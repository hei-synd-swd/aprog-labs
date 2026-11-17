//! Basic MiniBus demo: one channel, two subscribers, fan-out, drop-unsubscribe.
//!
//! Run with: cargo run --example basic

use minibus::Channel;
use std::thread;

fn main() {
    // Step 1: create a typed channel.
    let ch = Channel::<String>::new("news");

    // Step 2a: first subscriber, consumed by a background thread.
    let worker_sub = ch.subscribe();
    let worker = thread::spawn(move || {
        // Iteration blocks per message and ends when all channel handles are gone.
        for msg in worker_sub {
            println!("[worker] got: {msg}");
        }
        println!("[worker] channel closed, exiting");
    });

    // Step 2b: second subscriber, consumed on the main thread.
    let mut main_sub = ch.subscribe();

    // Step 3: publish a few messages; each one is fanned out to both subscribers.
    for text in ["hello", "pub/sub", "fan-out"] {
        let delivered = ch.publish(text.to_string());
        println!("[main]   published {text:?} -> {delivered} subscriber(s)");
        // Read our own copy right away (blocking next()).
        let msg = main_sub.next().unwrap();
        println!("[main]   got: {msg}");
    }

    // A cloned handle publishes to the very same channel.
    let ch2 = ch.clone();
    ch2.publish("via a cloned handle".to_string());
    println!("[main]   got: {}", main_sub.next().unwrap());

    // Step 4: drop one subscriber; it is pruned on the next publish.
    drop(main_sub);
    let delivered = ch.publish("only the worker sees this".to_string());
    println!("[main]   after drop -> delivered to {delivered} subscriber(s)");

    // Step 5: drop every channel handle so the worker's loop ends...
    drop(ch2);
    drop(ch);
    // ...then wait for the worker to finish.
    worker.join().unwrap();
    println!("[main]   done");
}
