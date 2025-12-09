//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: cartesian.rs | DNA/src/world/coordinates/cartesian.rs
//! PURPOSE: Cartesian coordinate system (x, y, z) - the default representation
//! LAYER: DNA → WORLD → COORDINATES
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Note: Most code uses glam::Vec2, Vec3, Vec4 directly.
//! This module exists for completeness and for conversions FROM other systems.
//!
//! ═══════════════════════════════════════════════════════════════════════════════

// Re-export glam types as canonical Cartesian representation
pub use glam::{Vec2, Vec3, Vec4};
