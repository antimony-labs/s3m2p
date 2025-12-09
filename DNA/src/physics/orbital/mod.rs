//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/physics/orbital/mod.rs
//! PURPOSE: Orbital mechanics - Kepler, N-body, perturbations
//! LAYER: DNA → PHYSICS → ORBITAL
//! ═══════════════════════════════════════════════════════════════════════════════

/// Two-body Kepler problem
pub mod kepler;
pub use kepler::OrbitalElements;

// pub mod n_body;        // TODO: N-body gravitational dynamics
// pub mod perturbation;  // TODO: Orbital perturbations
