//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: snap.rs | MCAD/src/snap.rs
//! PURPOSE: Smart snap system for sketch geometry - point, midpoint, center,
//!          intersection, perpendicular, and grid snapping
//! MODIFIED: 2026-01-08
//! LAYER: MCAD (L1 Bubble)
//! ═══════════════════════════════════════════════════════════════════════════════

use cad_engine::{Point2, Sketch, SketchEntity, SketchEntityId, SketchPointId};
use std::f32::consts::PI;

/// Snap type for enhanced snapping
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapType {
    None,
    Point,         // Existing endpoint
    Midpoint,      // Midpoint of line/arc
    Center,        // Center of circle/arc
    Intersection,  // Intersection of two entities
    Perpendicular, // Perpendicular to line
    Grid,          // Grid snap
}

/// Result of snap position calculation
#[derive(Clone, Debug)]
pub struct SnapResult {
    pub position: Point2,
    pub snap_type: SnapType,
    pub source_entity: Option<SketchEntityId>,
}

impl SnapResult {
    /// Create a snap result indicating no snap occurred
    pub fn none(pos: Point2) -> Self {
        Self {
            position: pos,
            snap_type: SnapType::None,
            source_entity: None,
        }
    }

    /// Create a snap result for grid snapping
    pub fn grid(pos: Point2, grid_size: f32) -> Self {
        Self {
            position: snap_to_grid(pos, grid_size),
            snap_type: SnapType::Grid,
            source_entity: None,
        }
    }
}

/// Snap position to grid
pub fn snap_to_grid(pos: Point2, grid_size: f32) -> Point2 {
    Point2::new(
        (pos.x / grid_size).round() * grid_size,
        (pos.y / grid_size).round() * grid_size,
    )
}

/// Find the closest point to a position within tolerance
pub fn find_point_at_position(
    sketch: &Sketch,
    pos: Point2,
    tolerance: f32,
) -> Option<SketchPointId> {
    let mut best_id: Option<SketchPointId> = None;
    let mut best_dist = tolerance;

    for point in &sketch.points {
        let dist = pos.distance(&point.position);
        if dist < best_dist {
            best_dist = dist;
            best_id = Some(point.id);
        }
    }

    best_id
}

/// Snap to existing point if close enough, otherwise snap to grid
pub fn snap_position(
    sketch: &Sketch,
    pos: Point2,
    point_tolerance: f32,
    grid_size: f32,
) -> (Point2, Option<SketchPointId>) {
    // Priority 1: Snap to existing points
    if let Some(point_id) = find_point_at_position(sketch, pos, point_tolerance) {
        if let Some(point) = sketch.point(point_id) {
            return (point.position, Some(point_id));
        }
    }

    // Priority 2: Snap to grid
    (snap_to_grid(pos, grid_size), None)
}

/// Enhanced snap with multiple snap types
/// Priority: Point > Midpoint > Center > Intersection > Perpendicular > Grid
pub fn snap_position_enhanced(
    sketch: &Sketch,
    pos: Point2,
    tolerance: f32,
    grid_size: f32,
) -> SnapResult {
    let tol_sq = tolerance * tolerance;

    // Priority 1: Snap to existing points (endpoints)
    for point in &sketch.points {
        if point.position.distance_squared(&pos) < tol_sq {
            return SnapResult {
                position: point.position,
                snap_type: SnapType::Point,
                source_entity: None,
            };
        }
    }

    // Priority 2: Snap to midpoints
    for entity in &sketch.entities {
        if let Some((mid, entity_id)) = entity_midpoint(sketch, entity) {
            if mid.distance_squared(&pos) < tol_sq {
                return SnapResult {
                    position: mid,
                    snap_type: SnapType::Midpoint,
                    source_entity: Some(entity_id),
                };
            }
        }
    }

    // Priority 3: Snap to centers (circles and arcs)
    for entity in &sketch.entities {
        match entity {
            SketchEntity::Circle { id, center, .. } | SketchEntity::Arc { id, center, .. } => {
                if let Some(center_pt) = sketch.point(*center) {
                    if center_pt.position.distance_squared(&pos) < tol_sq {
                        return SnapResult {
                            position: center_pt.position,
                            snap_type: SnapType::Center,
                            source_entity: Some(*id),
                        };
                    }
                }
            }
            _ => {}
        }
    }

    // Priority 4: Snap to intersections
    if let Some(intersection) = find_nearest_intersection(sketch, pos, tolerance) {
        return intersection;
    }

    // Priority 5: Snap to perpendicular foot (when near a line)
    for entity in &sketch.entities {
        if let SketchEntity::Line { id, start, end, .. } = entity {
            if let (Some(a), Some(b)) = (sketch.point(*start), sketch.point(*end)) {
                if let Some(foot) = perpendicular_foot(pos, a.position, b.position) {
                    if foot.distance_squared(&pos) < tol_sq {
                        return SnapResult {
                            position: foot,
                            snap_type: SnapType::Perpendicular,
                            source_entity: Some(*id),
                        };
                    }
                }
            }
        }
    }

    // Priority 6: Snap to grid
    SnapResult::grid(pos, grid_size)
}

/// Get midpoint of an entity
pub fn entity_midpoint(sketch: &Sketch, entity: &SketchEntity) -> Option<(Point2, SketchEntityId)> {
    match entity {
        SketchEntity::Line { id, start, end, .. } => {
            let a = sketch.point(*start)?.position;
            let b = sketch.point(*end)?.position;
            Some((a.midpoint(&b), *id))
        }
        SketchEntity::Arc {
            id,
            center,
            start,
            end,
            radius,
            ccw,
            ..
        } => {
            // Arc midpoint is the point on the arc at the middle angle
            let c = sketch.point(*center)?.position;
            let s = sketch.point(*start)?.position;
            let e = sketch.point(*end)?.position;

            let start_angle = (s.y - c.y).atan2(s.x - c.x);
            let end_angle = (e.y - c.y).atan2(e.x - c.x);

            let mut sweep = if *ccw {
                end_angle - start_angle
            } else {
                start_angle - end_angle
            };
            if sweep < 0.0 {
                sweep += 2.0 * PI;
            }

            let mid_angle = if *ccw {
                start_angle + sweep / 2.0
            } else {
                start_angle - sweep / 2.0
            };

            let mid = Point2::new(
                c.x + radius * mid_angle.cos(),
                c.y + radius * mid_angle.sin(),
            );
            Some((mid, *id))
        }
        _ => None,
    }
}

/// Find the closest point on a line segment from a given point (perpendicular foot)
/// Returns None if the foot is outside the segment (within 5%-95% range)
pub fn perpendicular_foot(p: Point2, a: Point2, b: Point2) -> Option<Point2> {
    let ab = Point2::new(b.x - a.x, b.y - a.y);
    let ap = Point2::new(p.x - a.x, p.y - a.y);

    let ab_len_sq = ab.x * ab.x + ab.y * ab.y;
    if ab_len_sq < 1e-10 {
        return None;
    }

    // Project p onto ab
    let t = (ap.x * ab.x + ap.y * ab.y) / ab_len_sq;

    // Only return if foot is within segment (with small margin)
    if (0.05..=0.95).contains(&t) {
        Some(Point2::new(a.x + t * ab.x, a.y + t * ab.y))
    } else {
        None
    }
}

/// Find intersection of two line segments
/// Returns None if segments are parallel or don't intersect
pub fn line_line_intersection(a1: Point2, a2: Point2, b1: Point2, b2: Point2) -> Option<Point2> {
    let d1 = Point2::new(a2.x - a1.x, a2.y - a1.y);
    let d2 = Point2::new(b2.x - b1.x, b2.y - b1.y);

    let cross = d1.x * d2.y - d1.y * d2.x;
    if cross.abs() < 1e-10 {
        return None; // Parallel
    }

    let dx = b1.x - a1.x;
    let dy = b1.y - a1.y;

    let t = (dx * d2.y - dy * d2.x) / cross;
    let u = (dx * d1.y - dy * d1.x) / cross;

    // Check if intersection is within both segments
    if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
        Some(Point2::new(a1.x + t * d1.x, a1.y + t * d1.y))
    } else {
        None
    }
}

/// Find the nearest intersection point to the given position
pub fn find_nearest_intersection(
    sketch: &Sketch,
    pos: Point2,
    tolerance: f32,
) -> Option<SnapResult> {
    let mut best: Option<(Point2, f32)> = None;
    let tol_sq = tolerance * tolerance;

    // Collect all lines for pairwise intersection testing
    let lines: Vec<_> = sketch
        .entities
        .iter()
        .filter_map(|e| {
            if let SketchEntity::Line { start, end, .. } = e {
                let a = sketch.point(*start)?.position;
                let b = sketch.point(*end)?.position;
                Some((a, b))
            } else {
                None
            }
        })
        .collect();

    // Test all pairs of lines
    for i in 0..lines.len() {
        for j in (i + 1)..lines.len() {
            if let Some(intersection) =
                line_line_intersection(lines[i].0, lines[i].1, lines[j].0, lines[j].1)
            {
                let dist_sq = intersection.distance_squared(&pos);
                if dist_sq < tol_sq && best.is_none_or(|(_, d)| dist_sq < d) {
                    best = Some((intersection, dist_sq));
                }
            }
        }
    }

    best.map(|(position, _)| SnapResult {
        position,
        snap_type: SnapType::Intersection,
        source_entity: None,
    })
}

/// Calculate distance from point to line segment
pub fn point_to_segment_distance(p: Point2, a: Point2, b: Point2) -> f32 {
    let ab = Point2::new(b.x - a.x, b.y - a.y);
    let ap = Point2::new(p.x - a.x, p.y - a.y);

    let ab_len_sq = ab.x * ab.x + ab.y * ab.y;
    if ab_len_sq < 1e-10 {
        return ap.x.hypot(ap.y);
    }

    // Project p onto ab, clamping to segment
    let t = ((ap.x * ab.x + ap.y * ab.y) / ab_len_sq).clamp(0.0, 1.0);
    let closest = Point2::new(a.x + t * ab.x, a.y + t * ab.y);

    p.distance(&closest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cad_engine::SketchPlane;

    fn new_sketch() -> Sketch {
        Sketch::new(SketchPlane::XY)
    }

    #[test]
    fn test_snap_to_grid() {
        let pos = Point2::new(23.7, 48.2);
        let snapped = snap_to_grid(pos, 10.0);
        assert_eq!(snapped.x, 20.0);
        assert_eq!(snapped.y, 50.0);
    }

    #[test]
    fn test_snap_to_grid_exact() {
        let pos = Point2::new(20.0, 50.0);
        let snapped = snap_to_grid(pos, 10.0);
        assert_eq!(snapped.x, 20.0);
        assert_eq!(snapped.y, 50.0);
    }

    #[test]
    fn test_snap_to_grid_negative() {
        let pos = Point2::new(-23.7, -48.2);
        let snapped = snap_to_grid(pos, 10.0);
        assert_eq!(snapped.x, -20.0);
        assert_eq!(snapped.y, -50.0);
    }

    #[test]
    fn test_perpendicular_foot_middle() {
        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(100.0, 0.0);
        let p = Point2::new(50.0, 30.0);

        let foot = perpendicular_foot(p, a, b).unwrap();
        assert!((foot.x - 50.0).abs() < 1e-6);
        assert!((foot.y - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_perpendicular_foot_near_endpoint() {
        // Point close to endpoint - should return None (within 5% margin)
        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(100.0, 0.0);
        let p = Point2::new(2.0, 10.0);

        assert!(perpendicular_foot(p, a, b).is_none());
    }

    #[test]
    fn test_perpendicular_foot_zero_length() {
        let a = Point2::new(50.0, 50.0);
        let b = Point2::new(50.0, 50.0);
        let p = Point2::new(60.0, 60.0);

        assert!(perpendicular_foot(p, a, b).is_none());
    }

    #[test]
    fn test_line_line_intersection_cross() {
        let a1 = Point2::new(0.0, 0.0);
        let a2 = Point2::new(100.0, 100.0);
        let b1 = Point2::new(0.0, 100.0);
        let b2 = Point2::new(100.0, 0.0);

        let int = line_line_intersection(a1, a2, b1, b2).unwrap();
        assert!((int.x - 50.0).abs() < 1e-6);
        assert!((int.y - 50.0).abs() < 1e-6);
    }

    #[test]
    fn test_line_line_intersection_parallel() {
        let a1 = Point2::new(0.0, 0.0);
        let a2 = Point2::new(100.0, 0.0);
        let b1 = Point2::new(0.0, 10.0);
        let b2 = Point2::new(100.0, 10.0);

        assert!(line_line_intersection(a1, a2, b1, b2).is_none());
    }

    #[test]
    fn test_line_line_intersection_no_overlap() {
        let a1 = Point2::new(0.0, 0.0);
        let a2 = Point2::new(10.0, 10.0);
        let b1 = Point2::new(50.0, 0.0);
        let b2 = Point2::new(50.0, 10.0);

        // These lines would intersect if extended, but segments don't overlap
        assert!(line_line_intersection(a1, a2, b1, b2).is_none());
    }

    #[test]
    fn test_point_to_segment_distance_on_segment() {
        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(100.0, 0.0);
        let p = Point2::new(50.0, 0.0);

        let dist = point_to_segment_distance(p, a, b);
        assert!(dist < 1e-6);
    }

    #[test]
    fn test_point_to_segment_distance_perpendicular() {
        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(100.0, 0.0);
        let p = Point2::new(50.0, 30.0);

        let dist = point_to_segment_distance(p, a, b);
        assert!((dist - 30.0).abs() < 1e-6);
    }

    #[test]
    fn test_point_to_segment_distance_to_endpoint() {
        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(100.0, 0.0);
        let p = Point2::new(-30.0, 40.0); // Beyond a

        let dist = point_to_segment_distance(p, a, b);
        let expected = (30.0_f32.powi(2) + 40.0_f32.powi(2)).sqrt(); // 50
        assert!((dist - expected).abs() < 1e-6);
    }

    #[test]
    fn test_snap_result_none() {
        let pos = Point2::new(10.0, 20.0);
        let result = SnapResult::none(pos);

        assert_eq!(result.snap_type, SnapType::None);
        assert_eq!(result.position.x, 10.0);
        assert_eq!(result.position.y, 20.0);
        assert!(result.source_entity.is_none());
    }

    #[test]
    fn test_snap_result_grid() {
        let pos = Point2::new(23.7, 48.2);
        let result = SnapResult::grid(pos, 10.0);

        assert_eq!(result.snap_type, SnapType::Grid);
        assert_eq!(result.position.x, 20.0);
        assert_eq!(result.position.y, 50.0);
        assert!(result.source_entity.is_none());
    }

    #[test]
    fn test_find_point_at_position_empty_sketch() {
        let sketch = new_sketch();
        let pos = Point2::new(50.0, 50.0);

        assert!(find_point_at_position(&sketch, pos, 10.0).is_none());
    }

    #[test]
    fn test_find_point_at_position_within_tolerance() {
        let mut sketch = new_sketch();
        let pt_id = sketch.add_point(Point2::new(50.0, 50.0));

        // Point within tolerance
        let pos = Point2::new(52.0, 48.0);
        let result = find_point_at_position(&sketch, pos, 10.0);
        assert_eq!(result, Some(pt_id));
    }

    #[test]
    fn test_find_point_at_position_outside_tolerance() {
        let mut sketch = new_sketch();
        sketch.add_point(Point2::new(50.0, 50.0));

        // Point outside tolerance
        let pos = Point2::new(100.0, 100.0);
        assert!(find_point_at_position(&sketch, pos, 10.0).is_none());
    }

    #[test]
    fn test_find_point_at_position_nearest() {
        let mut sketch = new_sketch();
        let pt1 = sketch.add_point(Point2::new(40.0, 50.0));
        let _pt2 = sketch.add_point(Point2::new(60.0, 50.0));

        // Should find nearest point
        let pos = Point2::new(42.0, 50.0);
        let result = find_point_at_position(&sketch, pos, 10.0);
        assert_eq!(result, Some(pt1));
    }

    #[test]
    fn test_snap_position_to_point() {
        let mut sketch = new_sketch();
        let pt_id = sketch.add_point(Point2::new(50.0, 50.0));

        let pos = Point2::new(52.0, 48.0);
        let (snapped, id) = snap_position(&sketch, pos, 10.0, 25.0);

        assert_eq!(snapped.x, 50.0);
        assert_eq!(snapped.y, 50.0);
        assert_eq!(id, Some(pt_id));
    }

    #[test]
    fn test_snap_position_to_grid() {
        let sketch = new_sketch();

        let pos = Point2::new(23.7, 48.2);
        let (snapped, id) = snap_position(&sketch, pos, 10.0, 25.0);

        assert_eq!(snapped.x, 25.0);
        assert_eq!(snapped.y, 50.0);
        assert!(id.is_none());
    }

    #[test]
    fn test_entity_midpoint_line() {
        let mut sketch = new_sketch();
        let p1 = sketch.add_point(Point2::new(0.0, 0.0));
        let p2 = sketch.add_point(Point2::new(100.0, 0.0));
        let line_id = sketch.add_entity(SketchEntity::Line {
            id: SketchEntityId(0),
            start: p1,
            end: p2,
        });

        let entity = &sketch.entities[0];
        let (mid, id) = entity_midpoint(&sketch, entity).unwrap();

        assert!((mid.x - 50.0).abs() < 1e-6);
        assert!((mid.y - 0.0).abs() < 1e-6);
        assert_eq!(id, line_id);
    }
}
