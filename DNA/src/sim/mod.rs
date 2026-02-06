pub mod types;
pub mod arena;
pub mod spatial_grid;
pub mod flocking;
pub mod state_machine;
pub mod interactions;
pub mod simulation;
pub mod world;
pub mod chladni;

pub use types::*;
pub use arena::BoidArena;
pub use spatial_grid::SpatialGrid;
pub use flocking::compute_flocking_forces;
pub use state_machine::update_states;
pub use interactions::{process_predation, process_scavenging};
pub use simulation::{SimConfig, simulation_step};
pub use world::*;
