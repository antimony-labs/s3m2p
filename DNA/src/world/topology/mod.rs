//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/world/topology/mod.rs
//! PURPOSE: Module exports: toroidal
//! MODIFIED: 2025-12-09
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

/// Toroidal (wrap-around) boundaries
pub mod toroidal;
pub use toroidal::{wrap_position, wrapped_distance};

// pub mod bounded;   // TODO: Future
// pub mod infinite;  // TODO: Future
