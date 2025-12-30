//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/physics/solvers/pde/mod.rs
//! PURPOSE: Module exports: spectral
//! MODIFIED: 2025-12-09
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

/// FFT-based spectral methods (Cooley-Tukey)
pub mod spectral;
pub use spectral::FFT2D;

/// Finite Difference Method solvers (explicit time-stepping)
pub mod fdm;
pub use fdm::DrivenWaveSolver2D;

// pub mod fem;       // TODO: Finite Element Method
