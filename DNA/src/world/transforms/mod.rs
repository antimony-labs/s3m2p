//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/world/transforms/mod.rs
//! PURPOSE: Coordinate system transformations
//! LAYER: DNA → WORLD → TRANSFORMS
//! ═══════════════════════════════════════════════════════════════════════════════

/// Astronomical coordinate transforms (J2000, ecliptic, galactic)
pub mod astronomical;
pub use astronomical::*;

// pub mod geodetic;      // TODO: Future
// pub mod projection;    // TODO: Future
// pub mod rotation;      // TODO: Future
