//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/math/mod.rs
//! PURPOSE: The Language - Pure mathematics primitives
//! LAYER: DNA → MATH
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! MATH provides the language for describing physics and geometry:
//! - mat.rs           - Mat2 (2x2 matrix operations)
//! - random.rs        - Random number generation utilities
//! - vec.rs           - Vec2, Vec3, Vec4 (future: or glam re-exports)
//! - quaternion.rs    - Rotation representation (future)
//! - complex.rs       - Complex number arithmetic (future)
//! - polynomial.rs    - Polynomial evaluation, roots (future)
//! - interpolation.rs - Linear, cubic, spline (future)
//!
//! ═══════════════════════════════════════════════════════════════════════════════

// ─────────────────────────────────────────────────────────────────────────────────
// ACTIVE SUBMODULES
// ─────────────────────────────────────────────────────────────────────────────────

/// 2x2 Matrix operations for 2D transforms and filters
pub mod mat;
pub use mat::Mat2;

/// Random number generation utilities
pub mod random;
pub use random::*;

// ─────────────────────────────────────────────────────────────────────────────────
// FUTURE SUBMODULES
// ─────────────────────────────────────────────────────────────────────────────────

// pub mod vec;           // TODO: Vec2, Vec3, Vec4 wrapper or re-export glam
// pub mod quaternion;    // TODO: Rotation representation
// pub mod complex;       // TODO: Extract from spice/ac.rs
// pub mod polynomial;    // TODO: Polynomial evaluation
// pub mod interpolation; // TODO: Linear, cubic, spline
