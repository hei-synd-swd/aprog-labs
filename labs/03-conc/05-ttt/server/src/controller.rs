//! The referee: it owns THE game and answers every request.
//!
//! `handle` takes one request, tagged with the seat it came from, and
//! returns the updates to send back. That is all it does — no channels, no
//! sockets, no printing, no waiting. `server.rs` feeds it and sends out
//! whatever comes back.
//!
//! The rule it follows is always the same:
//!   - a QUESTION is answered to the asker alone: `Hello` gets your seat
//!     and your board, `GetBoard` gets your board, a move that is refused
//!     gets your refusal;
//!   - a CHANGE is sent to everyone: an accepted move gets the new board,
//!     and the end of the game (or someone leaving) gets a `Bye`.
//!
//! Because the whole contract is one method over plain values, the tests
//! at the bottom need no threads: call `handle`, compare the answers.

use ttt_core::{Game, Player, Refusal, Request, State, Update};

/// Who an update is for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum To {
    /// Both players.
    Everyone,
    /// This seat only — the other player never sees it.
    Seat(Player),
}

/// Owns the game and answers requests, until someone leaves or the game
/// ends. After that it is `finished` and says nothing more.
pub(crate) struct SessionController {
    game: Game,
    finished: bool,
}

impl SessionController {
    pub(crate) fn new() -> Self {
        SessionController { game: Game::new(), finished: false }
    }

    /// The session is over, so the dispatcher can stop.
    pub(crate) fn finished(&self) -> bool {
        self.finished
    }

    /// Answer one request: the updates to send, each with its address.
    pub(crate) fn handle(&mut self, seat: Player, request: Request) -> Vec<(To, Update)> {
        if self.finished {
            return vec![]; // anything arriving late is not read
        }
        match request {
            // Sitting down: first who you are, then what the board looks like.
            Request::Hello => vec![
                (To::Seat(seat), Update::Seat(seat)),
                (To::Seat(seat), Update::Board(self.game.wire_state())),
            ],
            // Just asking for the board again: nothing changes.
            Request::GetBoard => vec![(To::Seat(seat), Update::Board(self.game.wire_state()))],
            Request::Move(n) => {
                // Not your turn, or against the rules: the refusal goes back
                // to you alone, and the other player hears nothing about it.
                if seat != self.game.turn() {
                    return vec![(To::Seat(seat), Update::Refused(Refusal::WrongTurn))];
                }
                if let Err(e) = self.game.play(n) {
                    return vec![(To::Seat(seat), Update::Refused(Refusal::Rule(e)))];
                }
                // Accepted: the board changed, so everyone gets it. If that
                // move ended the game, the last board is followed by a Bye.
                let mut replies = vec![(To::Everyone, Update::Board(self.game.wire_state()))];
                if self.game.state() != State::Playing {
                    replies.push((To::Everyone, Update::Bye));
                    self.finished = true;
                }
                replies
            }
            Request::Bye => {
                self.finished = true;
                vec![(To::Everyone, Update::Bye)]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ttt_core::PlayError;

    #[test]
    fn hello_answers_the_asker_with_seat_then_board() {
        let mut controller = SessionController::new();
        // Both answers go to the asker, not to everyone.
        assert_eq!(
            controller.handle(Player::O, Request::Hello),
            vec![
                (To::Seat(Player::O), Update::Seat(Player::O)),
                (To::Seat(Player::O), Update::Board("123456789".into())),
            ]
        );
    }

    #[test]
    fn a_legal_move_goes_to_everyone() {
        let mut controller = SessionController::new();
        assert_eq!(
            controller.handle(Player::X, Request::Move(5)),
            vec![(To::Everyone, Update::Board("1234X6789".into()))]
        );
    }

    #[test]
    fn refusals_are_explicit_and_private() {
        let mut controller = SessionController::new();
        // O tries to move first: not O's turn, so the refusal goes to O alone.
        assert_eq!(
            controller.handle(Player::O, Request::Move(1)),
            vec![(To::Seat(Player::O), Update::Refused(Refusal::WrongTurn))]
        );
        // Out of range from X, whose turn it is: refused with the game's
        // own error.
        assert_eq!(
            controller.handle(Player::X, Request::Move(0)),
            vec![(
                To::Seat(Player::X),
                Update::Refused(Refusal::Rule(PlayError::OutOfRange))
            )]
        );
        // Neither refusal changed anything: the board is still fresh.
        assert_eq!(
            controller.handle(Player::X, Request::GetBoard),
            vec![(To::Seat(Player::X), Update::Board("123456789".into()))]
        );
    }

    #[test]
    fn a_win_sends_the_last_board_then_bye_and_finishes() {
        let mut controller = SessionController::new();
        // X and O take turns; X takes 1-2-3.
        for (seat, n) in [
            (Player::X, 1),
            (Player::O, 4),
            (Player::X, 2),
            (Player::O, 5),
        ] {
            controller.handle(seat, Request::Move(n));
        }
        assert_eq!(
            controller.handle(Player::X, Request::Move(3)),
            vec![
                (To::Everyone, Update::Board("XXXOO6789".into())),
                (To::Everyone, Update::Bye),
            ]
        );
        // Finished: anything arriving after that is not read.
        assert!(controller.finished());
        assert_eq!(controller.handle(Player::O, Request::Hello), vec![]);
    }

    #[test]
    fn bye_from_anyone_ends_the_session() {
        let mut controller = SessionController::new();
        assert_eq!(
            controller.handle(Player::O, Request::Bye),
            vec![(To::Everyone, Update::Bye)]
        );
        assert!(controller.finished());
    }
}
