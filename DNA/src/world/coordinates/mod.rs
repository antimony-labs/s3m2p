//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/world/coordinates/mod.rs
//! PURPOSE: Coordinate system representations (Cartesian, spherical, etc.)
//! LAYER: DNA → WORLD → COORDINATES
//! ═══════════════════════════════════════════════════════════════════════════════

/// Cartesian coordinates (x, y, z) - default representation
pub mod cartesian;

/// Spherical coordinates (r, θ, φ)
pub mod spherical;
pub use spherical::Spherical;

// pub mod cylindrical;  // TODO: Future
// pub mod polar;        // TODO: Future
