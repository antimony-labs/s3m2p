//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/world/coordinates/mod.rs
//! PURPOSE: Module exports: cartesian, spherical
//! MODIFIED: 2025-12-09
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

/// Cartesian coordinates (x, y, z) - default representation
pub mod cartesian;

/// Spherical coordinates (r, θ, φ)
pub mod spherical;
pub use spherical::Spherical;

// pub mod cylindrical;  // TODO: Future
// pub mod polar;        // TODO: Future
