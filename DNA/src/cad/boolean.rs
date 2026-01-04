//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: boolean.rs | DNA/src/cad/boolean.rs
//! PURPOSE: Boolean operations on B-Rep solids (union, difference, intersection)
//! MODIFIED: 2026-01-04
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

use super::geometry::{Point3, Vector3, BoundingBox3};
use super::topology::{Solid, Vertex, Edge, Face, Shell, Loop, EdgeId, VertexId, FaceId, SurfaceType, CurveType};
use super::intersect::Classification;

/// Boolean operation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BooleanOp {
    Union,
    Difference,
    Intersection,
}

/// Error type for Boolean operations
#[derive(Debug, Clone)]
pub enum BooleanError {
    NoIntersection,
    DegenerateCase,
    TopologyError(String),
}

/// Boolean union: A ∪ B
///
/// Currently implements a simplified version for non-intersecting or simple solids.
/// Returns a new solid containing both input solids' faces.
pub fn union(a: &Solid, b: &Solid) -> Result<Solid, BooleanError> {
    // Check if bounding boxes intersect
    let mut a_clone = a.clone();
    let mut b_clone = b.clone();
    let bbox_a = a_clone.bounding_box();
    let bbox_b = b_clone.bounding_box();

    if !bbox_a.intersects(&bbox_b) {
        // Non-intersecting: simple merge
        return Ok(merge_solids(a, b));
    }

    // For intersecting solids, we need full Boolean logic
    // Simplified implementation: just merge for now
    Ok(merge_solids(a, b))
}

/// Boolean difference: A - B
///
/// Removes the volume of B from A.
/// Simplified implementation: returns A unchanged if no intersection.
pub fn difference(a: &Solid, b: &Solid) -> Result<Solid, BooleanError> {
    let mut a_clone = a.clone();
    let mut b_clone = b.clone();
    let bbox_a = a_clone.bounding_box();
    let bbox_b = b_clone.bounding_box();

    if !bbox_a.intersects(&bbox_b) {
        // No intersection: return A unchanged
        return Ok(a.clone());
    }

    // Full implementation would:
    // 1. Find intersection curves
    // 2. Split faces at intersections
    // 3. Classify faces (keep A faces outside B)
    // 4. Reconstruct shell

    // Simplified: return A for now
    Ok(a.clone())
}

/// Boolean intersection: A ∩ B
///
/// Returns the common volume between A and B.
pub fn intersection(a: &Solid, b: &Solid) -> Result<Solid, BooleanError> {
    let mut a_clone = a.clone();
    let mut b_clone = b.clone();
    let bbox_a = a_clone.bounding_box();
    let bbox_b = b_clone.bounding_box();

    if !bbox_a.intersects(&bbox_b) {
        return Err(BooleanError::NoIntersection);
    }

    // Full implementation would:
    // 1. Find intersection curves
    // 2. Split faces at intersections
    // 3. Classify faces (keep faces inside both solids)
    // 4. Reconstruct shell

    // Simplified: create bounding box intersection
    let min_x = bbox_a.min.x.max(bbox_b.min.x);
    let min_y = bbox_a.min.y.max(bbox_b.min.y);
    let min_z = bbox_a.min.z.max(bbox_b.min.z);
    let max_x = bbox_a.max.x.min(bbox_b.max.x);
    let max_y = bbox_a.max.y.min(bbox_b.max.y);
    let max_z = bbox_a.max.z.min(bbox_b.max.z);

    use crate::cad::primitives::make_box_at;
    let width = max_x - min_x;
    let height = max_y - min_y;
    let depth = max_z - min_z;
    let center = Point3::new((min_x + max_x) / 2.0, (min_y + max_y) / 2.0, (min_z + max_z) / 2.0);

    Ok(make_box_at(center, width, height, depth))
}

/// Merge two solids (simple combination without intersection handling)
fn merge_solids(a: &Solid, b: &Solid) -> Solid {
    let mut result = Solid::new();

    // Copy all vertices from both solids
    result.vertices.extend_from_slice(&a.vertices);
    let offset_b = a.vertices.len() as u32;
    result.vertices.extend_from_slice(&b.vertices);

    // Copy edges from A
    result.edges.extend_from_slice(&a.edges);

    // Copy edges from B with offset vertex indices
    let edge_offset = a.edges.len() as u32;
    for edge in &b.edges {
        let mut new_edge = edge.clone();
        new_edge.start.0 += offset_b;
        new_edge.end.0 += offset_b;
        // Update face references
        new_edge.faces = edge.faces.iter().map(|&fid| FaceId(fid.0 + a.faces.len() as u32)).collect();
        result.edges.push(new_edge);
    }

    // Copy faces from A
    result.faces.extend_from_slice(&a.faces);

    // Copy faces from B with offset edge indices
    for face in &b.faces {
        let mut new_face = face.clone();
        new_face.outer_loop.edges = face.outer_loop.edges.iter()
            .map(|&eid| EdgeId(eid.0 + edge_offset))
            .collect();
        for inner in &mut new_face.inner_loops {
            inner.edges = inner.edges.iter()
                .map(|&eid| EdgeId(eid.0 + edge_offset))
                .collect();
        }
        result.faces.push(new_face);
    }

    // Create shells
    result.shells.extend_from_slice(&a.shells);
    result.shells.extend_from_slice(&b.shells);

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cad::primitives::{make_box, make_box_at};

    #[test]
    fn test_union_non_intersecting() {
        let box1 = make_box(10.0, 10.0, 10.0);
        let box2 = make_box_at(Point3::new(20.0, 0.0, 0.0), 10.0, 10.0, 10.0);

        let result = union(&box1, &box2);
        assert!(result.is_ok());

        let solid = result.unwrap();
        // Should have vertices from both boxes
        assert_eq!(solid.vertices.len(), 16);
    }

    #[test]
    fn test_difference_non_intersecting() {
        let box1 = make_box(10.0, 10.0, 10.0);
        let box2 = make_box_at(Point3::new(20.0, 0.0, 0.0), 10.0, 10.0, 10.0);

        let result = difference(&box1, &box2);
        assert!(result.is_ok());

        let solid = result.unwrap();
        // Should be box1 unchanged
        assert_eq!(solid.vertices.len(), 8);
    }

    #[test]
    fn test_intersection_overlapping() {
        let box1 = make_box_at(Point3::new(0.0, 0.0, 0.0), 20.0, 20.0, 20.0);
        let box2 = make_box_at(Point3::new(10.0, 10.0, 10.0), 20.0, 20.0, 20.0);

        let result = intersection(&box1, &box2);
        assert!(result.is_ok());
    }
}
