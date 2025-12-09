//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: toroidal.rs | DNA/src/world/topology/toroidal.rs
//! PURPOSE: Toroidal topology (wrap-around boundaries)
//! MODIFIED: 2025-12-09
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

//!
//! PURPOSE: Toroidal topology (wrap-around boundaries)
//!
//! LAYER: DNA → WORLD → TOPOLOGY
//!
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │ DATA FLOW                                                                   │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │ CONSUMES:  Vec2 (position), f32 (world width/height)                        │
//! │ PRODUCES:  Vec2 (wrapped position)                                          │
//! └─────────────────────────────────────────────────────────────────────────────┘
//!
//! DEPENDS ON:
//!   • glam::Vec2 → Vector type
//!
//! USED BY:
//!   • DNA/src/lib.rs → Boid simulation
//!   • Future: Wrapped worlds, periodic boundary conditions
//!
//! ═══════════════════════════════════════════════════════════════════════════════

use glam::Vec2;

/// Wrap position to toroidal world boundaries
#[inline]
pub fn wrap_position(pos: Vec2, width: f32, height: f32) -> Vec2 {
    Vec2::new(
        if pos.x < 0.0 {
            pos.x + width
        } else if pos.x >= width {
            pos.x - width
        } else {
            pos.x
        },
        if pos.y < 0.0 {
            pos.y + height
        } else if pos.y >= height {
            pos.y - height
        } else {
            pos.y
        },
    )
}

/// Compute wrapped distance between two points
pub fn wrapped_distance(a: Vec2, b: Vec2, width: f32, height: f32) -> f32 {
    let dx = (a.x - b.x).abs();
    let dy = (a.y - b.y).abs();

    let dx = dx.min(width - dx);
    let dy = dy.min(height - dy);

    (dx * dx + dy * dy).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_position() {
        let pos = Vec2::new(-5.0, 105.0);
        let wrapped = wrap_position(pos, 100.0, 100.0);
        assert_eq!(wrapped, Vec2::new(95.0, 5.0));
    }

    #[test]
    fn test_wrapped_distance() {
        let a = Vec2::new(5.0, 50.0);
        let b = Vec2::new(95.0, 50.0);

        // Direct distance: 90
        // Wrapped distance: 10 (across boundary)
        let dist = wrapped_distance(a, b, 100.0, 100.0);
        assert!((dist - 10.0).abs() < 1e-5);
    }
}
