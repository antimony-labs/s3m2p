//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: cartesian.rs | DNA/src/world/coordinates/cartesian.rs
//! PURPOSE: Coordinates module implementation
//! MODIFIED: 2025-12-09
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

//!
//! Note: Most code uses glam::Vec2, Vec3, Vec4 directly.
//! This module exists for completeness and for conversions FROM other systems.
//!
//! ═══════════════════════════════════════════════════════════════════════════════

// Re-export glam types as canonical Cartesian representation
pub use glam::{Vec2, Vec3, Vec4};
