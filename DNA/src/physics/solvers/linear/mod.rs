//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/physics/solvers/linear/mod.rs
//! PURPOSE: Linear algebra solvers
//! LAYER: DNA → PHYSICS → SOLVERS → LINEAR
//! ═══════════════════════════════════════════════════════════════════════════════

/// Dense matrix operations (LU, QR)
pub mod dense;

// pub mod sparse;      // TODO: CSR format, sparse operations
// pub mod iterative;   // TODO: CG, GMRES, BiCGSTAB
// pub mod eigensolver; // TODO: Power iteration, QR algorithm
