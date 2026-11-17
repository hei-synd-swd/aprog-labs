//! The controller: the one place that decides what this program does.
//!
//! `main` feeds it one event at a time and sends out whatever it returns.
//! It reads what you type ("5" plays cell 5, "q" leaves, anything else
//! gets a hint), and it reacts to what the server pushes.
//!
//! Whether a move is legal is NOT its business: the server answers every
//! request, so a bad move comes back as a `Refused` and the display prints
//! it. The controller also owns the display, and is the only thing here
//! that decides when the session is over.

use ttt_core::{Request, Update};

use crate::display::ConsoleDisplay;

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
    /// `Update` is `ttt_core`'s own enum, not this file's — reaching the
    /// actual news costs a SECOND match, on purpose (see `handle`, below):
    ///   - `Seat(player)`    — your seat, the reply to `HELLO`
    ///   - `Board(cells)`    — the 9-character board, redrawn
    ///   - `Refused(reason)` — your last move was declined
    ///   - `Bye`             — the game is over
    ///
    /// All four are drawn in exactly one place, `ConsoleDisplay::on_update`
    /// (`display.rs`) — this file itself asks only one further question of
    /// `update`: was that the last one?
    FromServer(Update),
    /// The connection is gone. Also published by `net.rs`'s reader.
    Closed,
}

pub(crate) struct Controller {
    display: ConsoleDisplay,
    /// We typed "q": we are the one leaving, not the other player.
    leaving: bool,
    /// The session is over.
    done: bool,
}

impl Controller {
    /// The display is passed in, so the wiring decides what the front end is.
    pub(crate) fn new(display: ConsoleDisplay) -> Self {
        Controller { display, leaving: false, done: false }
    }

    /// One event in, the requests to send out.
    pub(crate) fn handle(&mut self, event: Event) -> Vec<Request> {
        match event {
            Event::Start => vec![Request::Hello], // sit down
            Event::FromStdin(line) => {
                if self.leaving {
                    return vec![]; // lines typed after "q" are not read
                }
                if line == "q" {
                    self.leaving = true;
                    return vec![Request::Bye];
                }
                match Self::decode(&line) {
                    Some(request) => vec![request],
                    None => vec![],
                }
            }
            Event::FromServer(update) => {
                // The 4-way breakdown of `update` happens inside this call
                // (display.rs) — this file only asks ONE further question
                // of it, below: was that the last word?
                self.display.on_update(&update, self.leaving);
                if update == Update::Bye {
                    self.done = true; // Bye is the last thing anyone hears
                }
                vec![]
            }
            Event::Closed => {
                // The connection dropped without a Bye: show it as one.
                if !self.done {
                    self.display.on_update(&Update::Bye, self.leaving);
                    self.done = true;
                }
                vec![]
            }
        }
    }

    /// The session is over, so the dispatcher can stop.
    pub(crate) fn finished(&self) -> bool {
        self.done
    }

    /// Turn one typed line into a request. `None` means "nothing to send".
    fn decode(line: &str) -> Option<Request> {
        match line.parse::<u8>() {
            Ok(n) => Some(Request::Move(n)),
            Err(_) => {
                println!("(type a cell number 1-9, or q to quit)");
                None
            }
        }
    }
}

// Like the server's tests: hand it events, check the requests. Legality is
// not tested here — it is not this file's job.
#[cfg(test)]
mod tests {
    use super::*;

    fn controller() -> Controller {
        Controller::new(ConsoleDisplay::new())
    }

    #[test]
    fn start_says_hello() {
        assert_eq!(controller().handle(Event::Start), vec![Request::Hello]);
    }

    #[test]
    fn a_number_becomes_a_move() {
        assert_eq!(
            controller().handle(Event::FromStdin("5".into())),
            vec![Request::Move(5)]
        );
    }

    #[test]
    fn anything_else_only_prints_a_hint() {
        assert!(
            controller()
                .handle(Event::FromStdin("abc".into()))
                .is_empty()
        );
    }

    #[test]
    fn q_says_bye_and_later_lines_are_not_read() {
        let mut c = controller();
        assert_eq!(c.handle(Event::FromStdin("q".into())), vec![Request::Bye]);
        assert!(c.handle(Event::FromStdin("5".into())).is_empty());
    }

    #[test]
    fn a_bye_from_the_server_ends_the_session() {
        let mut c = controller();
        assert!(c.handle(Event::FromServer(Update::Bye)).is_empty());
        assert!(c.finished());
    }

    #[test]
    fn a_lost_connection_ends_it_too() {
        let mut c = controller();
        assert!(c.handle(Event::Closed).is_empty());
        assert!(c.finished());
    }
}
