// Allow warnings common during development (stubs with unused params, unfinished code).
// Remove this once all methods are implemented.
#![allow(unused_variables, dead_code)]

/// 2D position on the game grid.
///
/// ⚠️ **Student task** - this module is not pre-written.
/// You must write the struct, derives, and all methods from scratch.
pub mod position;

/// A human entity that holds a position on the grid.
pub mod human;

/// A zombie entity with a position and stamina that decays each turn.
pub mod zombie;

/// The game world: owns all entities and runs the game loop.
pub mod world;

/// Shared utility functions.
pub mod utils;
