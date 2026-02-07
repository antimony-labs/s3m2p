pub mod arena;
pub mod chladni;
pub mod flocking;
pub mod interactions;
pub mod simulation;
pub mod spatial_grid;
pub mod state_machine;
pub mod types;
pub mod world;

pub use arena::BoidArena;
pub use flocking::compute_flocking_forces;
pub use interactions::{process_predation, process_scavenging};
pub use simulation::{simulation_step, SimConfig};
pub use spatial_grid::SpatialGrid;
pub use state_machine::update_states;
pub use types::*;
pub use world::*;
