//! ttt-core — the rules of the game, the words the two sides trade, and
//! the format those words travel in.
//!
//! No threads, no sockets, no bus: this crate is the part both the server
//! and the player need, and it can be tested on its own. Cells are
//! numbered 1..=9 like a phone keypad:
//!
//! ```text
//!  1 | 2 | 3
//! ---+---+---
//!  4 | 5 | 6
//! ---+---+---
//!  7 | 8 | 9
//! ```
//!
//! Three files: `game.rs` has the rules, `frame.rs` the two message enums
//! (what is said), `wire.rs` the text format (how it crosses the wire).

mod frame;
mod game;
pub mod wire;

pub use frame::{Refusal, Request, Update};
pub use game::{Game, PlayError, Player, State};
