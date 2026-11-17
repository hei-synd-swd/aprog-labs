//! The robot: the machine at the seat. Same dispatcher, different brain.
//!
//! A person watches the board and decides; a robot does too, only its
//! eyes and hands are this file. It draws every `Board` the server pushes
//! exactly like a human's screen does, and when it is its turn it picks a
//! free cell at random and plays it. No keyboard: the other seat is where
//! typing means something.
//!
//! The move itself is chosen in a way that can be tested without a
//! network: everything the robot knows about a board is
//! `Game::from_wire_state` plus `Game::free_cells` — the two `core`
//! helpers that exist exactly for this.
//!
//! A robot never reasons about legality: like a human's keyboard, its
//! move goes to the authority and comes back as a `Refused` if it was
//! wrong. In practice it only ever plays on its own turn, so a refusal
//! would mean a bug — the robot answers by staying quiet and waiting for
//! the next board, which the no-clock protocol makes safe.

use rand::{RngExt, SeedableRng, rngs::StdRng};

use super::{Controller, Event};
use crate::display::ConsoleDisplay;
use ttt_core::{Game, Request, State, Update};

/// The machine at the seat. Owns a display too — same screen a human
/// would get, so a robot game is not a black box. It leans on the
/// display for the one fact it would otherwise have to track itself:
/// which seat we are (`display.seat()`).
pub(crate) struct RobotController {
    display: ConsoleDisplay,
    /// The session is over.
    done: bool,
    rng: StdRng,
}

impl RobotController {
    /// A robot seeded with `seed`. A fixed seed makes a failing test the
    /// code's fault, not Fortune's — a live game seeds from
    /// `rand::random()`, see `main.rs`.
    pub(crate) fn new(seed: u64, display: ConsoleDisplay) -> Self {
        RobotController { display, done: false, rng: StdRng::seed_from_u64(seed) }
    }

    /// A move, if this update is a `Board` on which it is our turn.
    /// Everything else the display already drew is none of this
    /// function's business.
    fn decide(&mut self, update: &Update) -> Option<Request> {
        let Update::Board(cells) = update else {
            return None;
        };
        // A board we cannot read is ignored rather than crashing.
        let Some(game) = Game::from_wire_state(cells) else {
            return None;
        };
        // Only act on a running game, and only on our own turn: any
        // other board is just the other player thinking.
        if game.state() != State::Playing || self.display.seat() != Some(game.turn()) {
            return None;
        }
        let free = game.free_cells();
        let pick = self.rng.random_range(0..free.len());
        Some(Request::Move(free[pick]))
    }
}

impl Controller for RobotController {
    fn handle(&mut self, event: Event) -> Vec<Request> {
        match event {
            Event::Start => vec![Request::Hello], // sit down, like anyone else
            // A machine has no keyboard; whatever reaches the bus typed
            // is not for us. (In fact the keyboard thread never existing is
            // the point — see `main.rs`.)
            Event::FromStdin(_) => vec![],
            Event::FromServer(update) => {
                // Draw first, exactly like `HumanController` does — the
                // 4-way breakdown of `update` happens inside this call.
                self.display.on_update(&update, false); // never the one leaving
                let request = self.decide(&update); // Refused: stays quiet
                if update == Update::Bye {
                    self.done = true;
                }
                match request {
                    Some(request) => vec![request],
                    None => vec![],
                }
            }
            Event::Closed => {
                // The connection dropped without a Bye: show it as one.
                if !self.done {
                    self.display.on_update(&Update::Bye, false);
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
    use ttt_core::Player;

    fn controller(seed: u64) -> RobotController {
        RobotController::new(seed, ConsoleDisplay::new())
    }

    /// A robot that has already been seated as O.
    fn seated_o(seed: u64) -> RobotController {
        let mut robot = controller(seed);
        robot.handle(Event::FromServer(Update::Seat(Player::O)));
        robot
    }

    #[test]
    fn start_says_hello() {
        assert_eq!(controller(1).handle(Event::Start), vec![Request::Hello]);
    }

    #[test]
    fn plays_into_a_free_cell_on_its_turn() {
        let mut robot = seated_o(7);
        // O to play: X has taken cell 1 only.
        let replies = robot.handle(Event::FromServer(Update::Board("X23456789".into())));
        assert_eq!(replies.len(), 1);
        let Request::Move(cell) = replies[0] else {
            panic!("expected a move, got {replies:?}");
        };
        assert!(
            (2..=9).contains(&cell),
            "cell {cell} is not free — X owns 1"
        );
    }

    #[test]
    fn stays_silent_out_of_turn() {
        let mut robot = seated_o(7);
        // A fresh board: it is X's turn, we are O — nothing to answer.
        assert!(
            robot
                .handle(Event::FromServer(Update::Board("123456789".into())))
                .is_empty()
        );
    }

    #[test]
    fn stays_silent_once_the_game_is_over() {
        let mut robot = seated_o(7);
        // X owns 1-2-3: finished, even though the counts say it is O's turn.
        assert!(
            robot
                .handle(Event::FromServer(Update::Board("XXXOO6789".into())))
                .is_empty()
        );
    }

    #[test]
    fn a_refusal_is_never_retried_immediately() {
        let mut robot = seated_o(7);
        assert!(
            robot
                .handle(Event::FromServer(Update::Refused(
                    ttt_core::Refusal::WrongTurn
                )))
                .is_empty()
        );
    }

    #[test]
    fn keyboard_lines_are_not_read() {
        let mut robot = controller(1);
        assert!(robot.handle(Event::FromStdin("q".into())).is_empty());
        assert!(robot.handle(Event::FromStdin("5".into())).is_empty());
    }

    #[test]
    fn a_bye_from_the_server_ends_the_session() {
        let mut robot = controller(1);
        assert!(robot.handle(Event::FromServer(Update::Bye)).is_empty());
        assert!(robot.finished());
    }

    #[test]
    fn a_lost_connection_ends_it_too() {
        let mut robot = controller(1);
        assert!(robot.handle(Event::Closed).is_empty());
        assert!(robot.finished());
    }
}
