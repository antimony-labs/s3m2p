//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: dense.rs
//! PATH: DNA/src/physics/solvers/linear/dense.rs
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! PURPOSE: Dense matrix linear algebra (LU, QR decomposition)
//!
//! LAYER: DNA → PHYSICS → SOLVERS → LINEAR
//!
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │ ALGORITHM                                                                   │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │ LU Decomposition: A = L·U                                                   │
//! │   L = lower triangular                                                      │
//! │   U = upper triangular                                                      │
//! │   Solve Ax = b via L(Ux) = b                                                │
//! │   Complexity: O(n³)                                                         │
//! │                                                                             │
//! │ Partial pivoting for numerical stability                                    │
//! └─────────────────────────────────────────────────────────────────────────────┘
//!
//! TODO: Migrate from DNA/src/spice/matrix.rs
//!
//! REFERENCE: https://en.wikipedia.org/wiki/LU_decomposition
//!
//! ═══════════════════════════════════════════════════════════════════════════════

// TODO: Implement LU decomposition with partial pivoting
// TODO: Forward/backward substitution
// TODO: QR decomposition (Gram-Schmidt or Householder)
