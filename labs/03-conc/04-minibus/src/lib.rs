//! MiniBus — a tiny typed publish/subscribe bus for threads.
//!
//! ⚠️ **Student task** — two pieces are left for you to write:
//!
//! - `Channel::publish` (how a message *gets out* to everyone)
//! - `Subscriber::next`, the blocking receive (how a message *gets in*)
//!
//! Everything else — `new`, `name`, `subscribe`, `listen`, `try_next` and the
//! hand-written `Clone` — is provided: read it, don't rewrite it.

use std::sync::{Arc, Mutex, mpsc};

/// A sync callback, run in the publisher's thread.
/// (`type` here declares a reusable ALIAS — same keyword as `type Item`
/// in Iterator, but at module level.)
type Listener<T> = Arc<dyn Fn(&T) + Send + Sync>;

/// A named, typed broadcast channel.
///
/// Cloning a `Channel` gives another handle to the same channel,
/// so it can be shared across threads.
pub struct Channel<T> {
    name: String,
    subs: Arc<Mutex<Vec<mpsc::Sender<T>>>>,
    listeners: Arc<Mutex<Vec<Listener<T>>>>,
}

// Written by hand instead of #[derive(Clone)]: deriving would demand
// T: Clone, but cloning a handle only clones the Arcs, never a T.
impl<T> Clone for Channel<T> {
    fn clone(&self) -> Self {
        Channel {
            name: self.name.clone(),
            subs: Arc::clone(&self.subs),
            listeners: Arc::clone(&self.listeners),
        }
    }
}

// No bound here: creating, naming, subscribing and listening never
// copy a T. Only `publish` does — so only `publish` asks for Clone.
impl<T> Channel<T> {
    /// Create a new channel. The name is only used for debugging.
    pub fn new(name: impl Into<String>) -> Self {
        Channel {
            name: name.into(),
            subs: Arc::new(Mutex::new(Vec::new())),
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Channel name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Register a new subscriber with its own private, UNBOUNDED queue:
    /// a slow subscriber buffers indefinitely (see the README's design
    /// caveat) — it never blocks the publisher, it costs memory instead.
    pub fn subscribe(&self) -> Subscriber<T> {
        let (tx, rx) = mpsc::channel();
        self.subs.lock().unwrap().push(tx);
        Subscriber { rx }
    }

    /// Register a sync listener: a callback run in the publisher's thread
    /// on every publish. Cheap (no thread, no queue) but it blocks the
    /// publisher. Listeners live as long as the channel.
    pub fn listen(&self, f: impl Fn(&T) + Send + Sync + 'static) {
        self.listeners.lock().unwrap().push(Arc::new(f));
    }
}

// The one capability that costs a copy: fan-out gives every subscriber
// its own owned T.
impl<T: Clone> Channel<T> {
    /// Deliver `msg`: run the sync listeners (in THIS thread), then send a
    /// clone to every subscriber and drop the ones that have gone.
    /// Returns the number of subscribers that received the message.
    ///
    /// STUDENT TASK — the full walkthrough is in "Step 1 - publish" of the
    /// lab handout. Fields you need: `self.listeners` and `self.subs`.
    pub fn publish(&self, msg: T) -> usize {
        // 1. Listeners: lock `self.listeners` and call each one with a
        //    reference to the message. Do this before touching `subs`.

        // 2. Deliver: lock `self.subs`. Send a clone of the message to
        //    every subscriber, and record for each one whether the send
        //    succeeded — a send fails once that subscriber has gone away.

        // 3. Rebuild `subs` with only the subscribers whose send worked,
        //    and drop the rest. A `Sender` can be cloned (it is just a
        //    handle), so copying the survivors into a new list is cheap.

        // 4. Return how many subscribers are left.
        todo!()
    }
}

/// A subscription to a `Channel`. Consume messages by iterating.
///
/// Iteration blocks until a message arrives and ends when every
/// handle of the channel has been dropped. Dropping the subscriber
/// unsubscribes it (it is pruned on the next `publish`).
#[must_use = "dropping the subscriber unsubscribes it"]
pub struct Subscriber<T> {
    rx: mpsc::Receiver<T>,
}

impl<T> Subscriber<T> {
    /// Non-blocking receive. `None` if no message is waiting — or,
    /// indistinguishably, once the channel is gone. The blocking `next()`
    /// is the one that tells the two apart (its `None` MEANS gone).
    pub fn try_next(&self) -> Option<T> {
        self.rx.try_recv().ok()
    }
}

impl<T> Iterator for Subscriber<T> {
    type Item = T;

    /// Blocking receive. `None` when the channel is gone.
    ///
    /// STUDENT TASK
    fn next(&mut self) -> Option<T> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_01_one_subscriber_receives() {
        let ch = Channel::new("test");
        let mut sub = ch.subscribe();
        assert_eq!(ch.publish("hello"), 1);
        assert_eq!(sub.next(), Some("hello"));
    }

    #[test]
    fn test_02_broadcast_to_many() {
        let ch = Channel::new("test");
        let mut a = ch.subscribe();
        let mut b = ch.subscribe();
        assert_eq!(ch.publish(42), 2);
        assert_eq!(a.next(), Some(42));
        assert_eq!(b.next(), Some(42));
    }

    #[test]
    fn test_03_drop_unsubscribes() {
        let ch = Channel::new("test");
        let sub = ch.subscribe();
        drop(sub);
        // The dead subscriber is pruned on publish.
        assert_eq!(ch.publish(1), 0);
    }

    #[test]
    fn test_04_works_across_threads() {
        let ch = Channel::new("test");
        let sub = ch.subscribe();
        // The cloned handle IS the behavior under test: one handle per
        // worker, all driving the same channel.
        let publisher = ch.clone();
        drop(ch); // the publisher thread now owns the LAST handle
        let handle = thread::spawn(move || {
            for i in 0..3 {
                publisher.publish(i);
            }
        });
        // collect() drains until the publisher drops its handle at thread
        // exit — so this cannot hang on a regression, it just ends short.
        let got: Vec<i32> = sub.collect();
        handle.join().unwrap();
        assert_eq!(got, vec![0, 1, 2]);
    }

    #[test]
    fn test_05_listener_runs_in_publisher_thread() {
        let ch = Channel::new("test");
        // A shared counter, with the same tools the bus itself uses.
        let count = Arc::new(Mutex::new(0));
        let seen = Arc::clone(&count);
        let publisher_thread = thread::current().id();
        ch.listen(move |_msg: &i32| {
            assert_eq!(thread::current().id(), publisher_thread);
            *seen.lock().unwrap() += 1;
        });
        ch.publish(1);
        ch.publish(2);
        // Synchronous: already counted when publish returns.
        assert_eq!(*count.lock().unwrap(), 2);
    }

    #[test]
    fn test_06_listeners_and_subscribers_coexist() {
        let ch = Channel::new("test");
        let count = Arc::new(Mutex::new(0));
        let seen = Arc::clone(&count);
        ch.listen(move |_| {
            *seen.lock().unwrap() += 1;
        });
        let mut sub = ch.subscribe();
        assert_eq!(ch.publish("both"), 1); // count = subscribers only
        assert_eq!(*count.lock().unwrap(), 1);
        assert_eq!(sub.next(), Some("both"));
    }

    #[test]
    fn test_07_try_next_never_blocks() {
        let ch = Channel::new("test");
        let sub = ch.subscribe();
        assert_eq!(sub.try_next(), None); // nothing waiting yet
        ch.publish(7);
        assert_eq!(sub.try_next(), Some(7));
        assert_eq!(sub.try_next(), None); // drained
    }

    #[test]
    fn test_08_iteration_ends_when_channel_is_gone() {
        let ch = Channel::new("test");
        let mut sub = ch.subscribe();
        ch.publish("last");
        drop(ch);
        assert_eq!(sub.next(), Some("last"));
        assert_eq!(sub.next(), None);
    }
}
