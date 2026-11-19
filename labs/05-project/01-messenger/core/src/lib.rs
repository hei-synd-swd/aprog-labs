//! messenger-core — the words the two sides trade, and the format those
//! words travel in.
//!
//! No threads, no sockets, no bus: this crate is the part both the server
//! and the player need, and it can be tested on its own.
//!
//! Two files: `frame.rs` has the two message enums (what is said),
//! `wire.rs` the text format (how it crosses the wire).

mod frame;
pub mod wire;

pub use frame::{Request, Update, is_valid_name};
