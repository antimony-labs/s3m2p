//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/world/topology/mod.rs
//! PURPOSE: World boundary behaviors (wrap-around, walls, infinite)
//! LAYER: DNA → WORLD → TOPOLOGY
//! ═══════════════════════════════════════════════════════════════════════════════

/// Toroidal (wrap-around) boundaries
pub mod toroidal;
pub use toroidal::{wrap_position, wrapped_distance};

// pub mod bounded;   // TODO: Future
// pub mod infinite;  // TODO: Future
