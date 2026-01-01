//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/physics/orbital/mod.rs
//! PURPOSE: Module exports: kepler
//! MODIFIED: 2025-12-09
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

/// Two-body Kepler problem
pub mod kepler;
pub use kepler::OrbitalElements;

// pub mod n_body;        // TODO: N-body gravitational dynamics
// pub mod perturbation;  // TODO: Orbital perturbations
