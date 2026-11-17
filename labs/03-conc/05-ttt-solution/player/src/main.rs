//! player — one seat: a person at a keyboard, or a robot.
//!
//! This file wires the program together, top to bottom in the order the
//! data flows: the command line and the socket, the controller, the two
//! buses, the worker threads, then the dispatcher loop.
//!
//! The files beside it: `controller.rs` holds the trait both controllers
//! share, and `controller/` the two implementations (`human_controller.rs`,
//! the person's; `robot_controller.rs`, the machine's). `display.rs` draws for
//! the person, `input.rs` reads the keyboard, `net.rs` talks to the socket.
//!
//! Usage: `player <host:port> [human|robot]` — the optional second word
//! chooses which controller runs the session; without it the seat is a
//! human. Start a game with `server <port>`; the first two players to
//! connect get the two seats.

mod controller;
mod display;
mod input;
mod net;

use std::process;

use minibus::Channel;
use ttt_core::Request;

use controller::{Controller, Event, HumanController, RobotController};
use display::ConsoleDisplay;

/// Which controller runs this session, chosen when the program starts.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Human,
    Robot,
}

impl Kind {
    /// The word on the command line, as a kind.
    fn parse(kind: &str) -> Option<Kind> {
        match kind {
            "human" => Some(Kind::Human),
            "robot" => Some(Kind::Robot),
            _ => None,
        }
    }
}

fn main() {
    // -- the edge of the program: command line in, exit codes out --------
    let (addr, kind) = parse_args();
    let Some(socket) = net::connect(&addr) else {
        process::exit(1);
    };

    // -- the controller, decided at startup: a person (given a screen to
    //    draw on) or a robot (no screen, no keyboard) ----------------------
    let mut session: Box<dyn Controller> = match kind {
        Kind::Human => Box::new(HumanController::new(ConsoleDisplay::new())),
        Kind::Robot => Box::new(RobotController::new(rand::random(), ConsoleDisplay::new())),
    };

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

    // -- the workers: the keyboard (only a human has one), and the two
    //    directions of the socket. One clone of the socket for the reader;
    //    the writer gets the original, since nothing else needs it after
    //    this. ------------------------------------------------------------
    if kind == Kind::Human {
        input::spawn_stdin(&events);
    }
    net::spawn_net_reader(&events, socket.try_clone().expect("cannot clone stream"));
    let writer = net::spawn_net_writer(&outgoing, socket);

    // -- THE DISPATCHER: ask the controller what to do with each event,
    //    send whatever it says — the SAME two steps for every event,
    //    `Start` included. The server runs the same loop. -----------------
    for event in incoming {
        for request in session.handle(event) {
            outgoing.publish(request);
        }
        if session.finished() {
            break;
        }
    }

    // Drop our handle on `outgoing` — it is the last one, so the writer
    // thread sends what is left in its queue and stops. Waiting for it is
    // what guarantees our last message really left.
    drop(outgoing);
    let _ = writer.join();
}

/// One address, and an optional player kind, on the command line. Owns the
/// usage exit code.
fn parse_args() -> (String, Kind) {
    let args: Vec<String> = std::env::args().collect();
    if !(2..=3).contains(&args.len()) {
        eprintln!("usage: player <host:port> [human|robot]");
        process::exit(2);
    }
    let addr = args[1].clone();
    let kind = if args.len() == 3 {
        match Kind::parse(&args[2]) {
            Some(kind) => kind,
            None => {
                eprintln!("usage: player <host:port> [human|robot]");
                process::exit(2);
            }
        }
    } else {
        Kind::Human
    };
    (addr, kind)
}
