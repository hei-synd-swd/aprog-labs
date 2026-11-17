//! The controllers: the one place that decides what this program does, for
//! whichever kind of player sits down.
//!
//! This module is the contract between the session and whoever sits at the
//! seat. It holds the shared types — the `Event` bus and the `Controller`
//! trait — and the two implementations:
//!   - `human_controller` is the person at the keyboard;
//!   - `robot_controller` is the machine with no screen.
//!
//! Both are the same `Controller` trait from `main`'s point of view: one
//! event in, the requests to send out. `main` talks to the trait, never to
//! a particular kind.
//!
//! Whether a move is legal is NOT any controller's business: the server
//! answers every request, so a bad move comes back as a `Refused` and the
//! display prints it.

mod human_controller;
mod robot_controller;

pub(crate) use human_controller::HumanController;
pub(crate) use robot_controller::RobotController;

use ttt_core::{Request, Update};

/// Everything the controller can react to, all on the one `events` bus.
/// Three of these arrive from two background threads; the fourth,
/// `Start`, is `main`'s own doing — asking "what do we do at the very
/// beginning?" is not a different question from asking it about anything
/// else, so it gets the same answer machinery as everything below.
#[derive(Clone)]
pub(crate) enum Event {
    /// The session begins. Published exactly once, by `main` itself,
    /// before any worker thread even exists — see `main.rs` for why that
    /// ordering is what makes this safe rather than a race.
    Start,
    /// One line typed on the keyboard. Published by `input.rs`'s stdin
    /// thread — the payload is exactly what was typed, unparsed.
    FromStdin(String),
    /// One message the server sent us. Published by `net.rs`'s reader.
    ///
    /// `Update` is `ttt_core`'s own enum, not this module's — reaching the
    /// actual news costs a SECOND match, on purpose. Each implementation
    /// draws these, and then `handle` asks one further question of
    /// `update`: was that the last one?
    ///   - `Seat(player)`    — your seat, the reply to `HELLO`
    ///   - `Board(cells)`    — the 9-character board, redrawn
    ///   - `Refused(reason)` — your last move was declined
    ///   - `Bye`             — the game is over
    FromServer(Update),
    /// The connection is gone. Also published by `net.rs`'s reader.
    Closed,
}

/// Whatever sits at the seat, person or machine, answers these two
/// questions. `main` drives a session through this trait, so the choice
/// made at startup is the only place the two kinds ever meet.
pub(crate) trait Controller {
    /// One event in, the requests to send out.
    fn handle(&mut self, event: Event) -> Vec<Request>;
    /// The session is over, so the dispatcher can stop.
    fn finished(&self) -> bool;
}
