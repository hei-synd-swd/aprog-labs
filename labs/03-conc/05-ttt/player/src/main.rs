//! player — one person at a keyboard.
//!
//! This file wires the program together, top to bottom in the order the
//! data flows: the command line and the socket, the controller, the two
//! buses, the worker threads, then the dispatcher loop.
//!
//! The files beside it: `controller.rs` decides what to do, `display.rs`
//! draws, `input.rs` reads the keyboard, `net.rs` talks to the socket.
//!
//! Usage: `player <host:port>` — start a game with `server <port>`; the
//! first two players to connect get the two seats.

mod controller;
mod display;
mod input;
mod net;

use std::process;

use minibus::Channel;
use ttt_core::Request;

use controller::{Controller, Event};
use display::ConsoleDisplay;

fn main() {
    // -- the edge of the program: command line in, exit codes out --------
    let addr = parse_args();
    let Some(socket) = net::connect(&addr) else {
        process::exit(1);
    };

    // -- the controller, with the screen it draws on ---------------------
    let mut controller = Controller::new(ConsoleDisplay::new());

    // -- the two buses. Main only ever goes ONE way on each: it READS
    //    `events` (as `incoming`, below) and WRITES `outgoing` — never
    //    the other way round on either one. ------------------------------
    let events: Channel<Event> = Channel::new("events");
    let outgoing: Channel<Request> = Channel::new("outgoing");
    let incoming = events.subscribe(); // before ANYTHING can publish

    // Sit down: the session's own first event, published the exact same
    // way every worker below publishes theirs. Doing it here — before any
    // worker thread exists to publish anything else — makes it the
    // guaranteed first thing `incoming` ever yields, no race to reason
    // about.
    events.publish(Event::Start);

    // -- the workers: the keyboard, and the two directions of the socket.
    //    One clone of the socket for the reader; the writer gets the
    //    original, since nothing else needs it after this. ---------------
    input::spawn_stdin(&events);
    net::spawn_net_reader(&events, socket.try_clone().expect("cannot clone stream"));
    let writer = net::spawn_net_writer(&outgoing, socket);

    // -- THE DISPATCHER: ask the controller what to do with each event,
    //    send whatever it says — the SAME two steps for every event,
    //    `Start` included. The server runs the same loop. -----------------
    for event in incoming {
        for request in controller.handle(event) {
            outgoing.publish(request);
        }
        if controller.finished() {
            break;
        }
    }

    // Drop our handle on `outgoing` — it is the last one, so the writer
    // thread sends what is left in its queue and stops. Waiting for it is
    // what guarantees our last message really left.
    drop(outgoing);
    let _ = writer.join();
}

/// Exactly one address on the command line. Owns the usage exit code.
fn parse_args() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: player <host:port>");
        process::exit(2);
    }
    args[1].clone()
}
