//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/physics/mod.rs
//! PURPOSE: The Rules - Physics simulation algorithms organized by domain
//! LAYER: DNA → PHYSICS
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! PHYSICS defines HOW things behave:
//! - core/             - Units, constants, physical quantities
//! - mechanics/        - Particle, rigid body, collision, constraints
//! - fields/           - Scalar, vector, tensor, wave fields
//! - electromagnetics/ - Maxwell, FDTD, lumped circuits
//! - fluids/           - Euler, Navier-Stokes, SPH, Lattice-Boltzmann
//! - thermal/          - Conduction, convection, radiation
//! - orbital/          - Kepler, N-body, perturbation
//! - solvers/          - ODE, PDE, linear, nonlinear solvers
//!
//! ═══════════════════════════════════════════════════════════════════════════════

// ─────────────────────────────────────────────────────────────────────────────────
// SUBMODULES
// ─────────────────────────────────────────────────────────────────────────────────

pub mod core;
pub mod mechanics;
pub mod fields;
pub mod electromagnetics;
pub mod fluids;
pub mod thermal;
pub mod orbital;
pub mod solvers;
