//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/world/transforms/mod.rs
//! PURPOSE: Module exports: astronomical
//! MODIFIED: 2025-12-09
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

/// Astronomical coordinate transforms (J2000, ecliptic, galactic)
pub mod astronomical;
pub use astronomical::*;

// pub mod geodetic;      // TODO: Future
// pub mod projection;    // TODO: Future
// pub mod rotation;      // TODO: Future
