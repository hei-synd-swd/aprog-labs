/// 2D position on the game grid.
pub mod position;

/// A human entity that holds a position on the grid.
pub mod human;

/// A zombie entity with a position and stamina that decays each turn.
pub mod zombie;

/// The game world: owns all entities and runs the game loop.
pub mod world;

/// Shared utility functions.
pub mod utils;
