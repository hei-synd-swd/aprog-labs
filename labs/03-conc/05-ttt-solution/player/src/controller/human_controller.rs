//! The person's controller.
//!
//! The human reads what you type ("5" plays cell 5, "q" leaves, anything
//! else gets a hint), and reacts to what the server pushes. It also owns
//! the display, and is the only thing here that decides when the session
//! is over.
//!
//! Whether a move is legal is NOT this controller's business: the server
//! answers every request, so a bad move comes back as a `Refused` and the
//! display prints it.
//!
//! Like the server's tests: hand it events, check the requests. Legality is
//! not tested here — it is not this file's job.

use super::{Controller, Event};
use crate::display::ConsoleDisplay;
use ttt_core::{Request, Update};

/// The person at the seat. Owns the display, remembers whether we typed
/// "q" (we are the one leaving) and whether the session is over.
pub(crate) struct HumanController {
    display: ConsoleDisplay,
    /// We typed "q": we are the one leaving, not the other player.
    leaving: bool,
    /// The session is over.
    done: bool,
}

impl HumanController {
    /// The display is passed in, so the wiring decides what the front end is.
    pub(crate) fn new(display: ConsoleDisplay) -> Self {
        HumanController { display, leaving: false, done: false }
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

impl Controller for HumanController {
    fn handle(&mut self, event: Event) -> Vec<Request> {
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

    fn finished(&self) -> bool {
        self.done
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn controller() -> HumanController {
        HumanController::new(ConsoleDisplay::new())
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
