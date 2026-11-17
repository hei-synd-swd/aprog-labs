//! What the two sides can say to each other.
//!
//! Two directions, so two enums: a `Request` travels from a client to the
//! server, an `Update` travels back. These say WHAT is meant; turning one
//! into a line of text is `wire.rs`'s job.

use std::fmt;

use crate::game::{PlayError, Player};

/// A message a client sends to the server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Request {
    /// Sit down: asks for your seat and the current board.
    Hello,
    /// Play a cell (1..=9).
    Move(u8),
    /// Ask for the current board again. Plays nothing.
    GetBoard,
    /// Leaving.
    Bye,
}

/// A message the server sends back to a client.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Update {
    /// Your seat — the answer to your `Hello`, sent before your first board.
    Seat(Player),
    /// The board, as 9 characters (see `Game::wire_state`).
    Board(String),
    /// Your move was refused, and here is why.
    Refused(Refusal),
    /// The game is over, or someone left.
    Bye,
}

/// Why a move was refused: either it was not your turn, or the rules said
/// no. The second case reuses the game's own error rather than inventing a
/// second list of the same problems.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    WrongTurn,
    Rule(PlayError),
}

/// What a client prints for each refusal.
impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Refusal::WrongTurn => write!(f, "not your turn"),
            Refusal::Rule(e) => write!(f, "{e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The exact words of each refusal, pinned: the e2e script looks for
    /// them, and a port to another language must print the same.
    #[test]
    fn refusal_words_are_pinned() {
        assert_eq!(Refusal::WrongTurn.to_string(), "not your turn");
        assert_eq!(
            Refusal::Rule(PlayError::CellTaken).to_string(),
            "that cell is already taken"
        );
        assert_eq!(
            Refusal::Rule(PlayError::OutOfRange).to_string(),
            "pick a cell between 1 and 9"
        );
    }
}
