//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/physics/solvers/pde/mod.rs
//! PURPOSE: Module exports: spectral
//! MODIFIED: 2025-12-09
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

/// FFT-based spectral methods (Cooley-Tukey)
pub mod spectral;
pub use spectral::FFT2D;

// pub mod fdm;       // TODO: Finite Difference Method
// pub mod fem;       // TODO: Finite Element Method
