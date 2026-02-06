//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: pattern.rs | DNA/src/cad/pattern.rs
//! PURPOSE: Simple linear/circular pattern utilities for solids
//! MODIFIED: 2026-01-04
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

use super::geometry::{Point3, Transform3, Vector3};
use super::topology::Solid;

/// Create `count` translated copies of `solid` along `direction` spaced by `spacing`.
///
/// The first element is always an unmodified clone of the input solid.
pub fn linear_pattern(solid: &Solid, direction: Vector3, count: u32, spacing: f32) -> Vec<Solid> {
    let count = count.max(1);
    let mut out = Vec::with_capacity(count as usize);
    let dir = direction.normalize_or_z();

    for i in 0..count {
        let mut s = solid.clone();
        let offset = dir * (spacing * i as f32);
        let t = Transform3::from_translation(offset);
        for v in &mut s.vertices {
            v.point = v.point.transform(&t);
        }
        out.push(s);
    }

    out
}

/// Create `count` rotated copies of `solid` around an axis passing through `center`.
///
/// The first element is always an unmodified clone of the input solid.
pub fn circular_pattern(solid: &Solid, axis: Vector3, center: Point3, count: u32) -> Vec<Solid> {
    let count = count.max(1);
    let mut out = Vec::with_capacity(count as usize);
    let axis = axis.normalize_or_z();

    for i in 0..count {
        let mut s = solid.clone();
        let theta = (i as f32) * (2.0 * std::f32::consts::PI / count as f32);
        let rot = Transform3::from_axis_angle(axis, theta);
        let to_origin = Transform3::from_translation(Vector3::new(-center.x, -center.y, -center.z));
        let back = Transform3::from_translation(Vector3::new(center.x, center.y, center.z));
        let t = to_origin.then(rot).then(back);

        for v in &mut s.vertices {
            v.point = v.point.transform(&t);
        }

        out.push(s);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cad::primitives::make_box;

    #[test]
    fn test_linear_pattern_count() {
        let solid = make_box(1.0, 1.0, 1.0);
        let copies = linear_pattern(&solid, Vector3::X, 5, 10.0);
        assert_eq!(copies.len(), 5);
    }

    #[test]
    fn test_circular_pattern_count() {
        let solid = make_box(1.0, 1.0, 1.0);
        let copies = circular_pattern(&solid, Vector3::Z, Point3::ORIGIN, 8);
        assert_eq!(copies.len(), 8);
    }
}
