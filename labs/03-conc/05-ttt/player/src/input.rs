//! The keyboard: every line typed becomes an event on the `events` bus.
//!
//! Swap this file and `display.rs` for another front end — a window, say —
//! and nothing else changes: it would publish the same `Event::FromStdin`.

use std::io::BufRead;
use std::thread;

use minibus::Channel;

use crate::controller::Event;

/// stdin -> events. This thread is the ONLY source of `Event::FromStdin` —
/// every line typed, one per publish. When stdin ends (Ctrl-D, or a script
/// running out of input) we pretend the user typed "q", so the program
/// leaves properly instead of hanging.
pub(crate) fn spawn_stdin(events: &Channel<Event>) {
    let bus = events.clone();
    thread::spawn(move || {
        for line in std::io::stdin().lock().lines() {
            let Ok(line) = line else { break };
            bus.publish(Event::FromStdin(line.trim().to_string()));
        }
        bus.publish(Event::FromStdin("q".to_string()));
    });
}
