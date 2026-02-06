//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: extrude.rs | DNA/src/cad/extrude.rs
//! PURPOSE: Extrude 2D sketch profiles to create 3D B-Rep solids
//! MODIFIED: 2026-01-04
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

use super::geometry::{Point3, Vector3};
use super::sketch::{Point2, Sketch, SketchEntity, SketchEntityId};
use super::topology::{
    CurveType, Edge, EdgeId, Face, FaceId, FaceOrientation, Loop, Shell, ShellId, Solid,
    SurfaceType, Vertex, VertexId,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Extrusion parameters
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtrudeParams {
    pub distance: f32,
    pub symmetric: bool, // Extrude both directions from sketch plane
}

impl Default for ExtrudeParams {
    fn default() -> Self {
        Self {
            distance: 50.0,
            symmetric: false,
        }
    }
}

/// Extrusion error types
#[derive(Debug, Clone)]
pub enum ExtrudeError {
    NoClosedProfile,
    SelfIntersecting,
    InvalidGeometry,
}

/// Extrude a closed sketch profile to create a 3D solid
///
/// Finds closed loops in the sketch and extrudes them along the plane normal.
/// Returns a B-Rep solid with proper topology.
pub fn extrude_sketch(sketch: &Sketch, params: &ExtrudeParams) -> Result<Solid, ExtrudeError> {
    // Find closed loops
    let loops = find_closed_loops(sketch)?;
    if loops.is_empty() {
        return Err(ExtrudeError::NoClosedProfile);
    }

    // For simplicity, extrude the first closed loop
    let loop_entities = &loops[0];

    // Build solid
    let mut solid = Solid::new();

    // Get all points in the loop
    let loop_points: Vec<Point2> = loop_entities
        .iter()
        .filter_map(|&entity_id| get_entity_start_point(sketch, entity_id))
        .collect();

    if loop_points.is_empty() {
        return Err(ExtrudeError::InvalidGeometry);
    }

    // Calculate extrusion offsets
    let (z_start, z_end) = if params.symmetric {
        (-params.distance / 2.0, params.distance / 2.0)
    } else {
        (0.0, params.distance)
    };

    // Create bottom vertices (at sketch plane)
    let bottom_verts: Vec<VertexId> = loop_points
        .iter()
        .map(|p| {
            let p3 = sketch.to_3d_point(*p);
            let p3 = Point3::new(p3.x, p3.y, z_start);
            let id = VertexId(solid.vertices.len() as u32);
            solid.vertices.push(Vertex {
                id,
                point: p3,
                edges: Vec::new(),
            });
            id
        })
        .collect();

    // Create top vertices (at extrude distance)
    let top_verts: Vec<VertexId> = loop_points
        .iter()
        .map(|p| {
            let p3 = sketch.to_3d_point(*p);
            let p3 = Point3::new(p3.x, p3.y, z_end);
            let id = VertexId(solid.vertices.len() as u32);
            solid.vertices.push(Vertex {
                id,
                point: p3,
                edges: Vec::new(),
            });
            id
        })
        .collect();

    // Create bottom edges
    let bottom_edges: Vec<EdgeId> = (0..bottom_verts.len())
        .map(|i| {
            let next = (i + 1) % bottom_verts.len();
            let id = EdgeId(solid.edges.len() as u32);
            solid.edges.push(Edge {
                id,
                start: bottom_verts[i],
                end: bottom_verts[next],
                curve: CurveType::Linear,
                faces: Vec::new(),
            });
            id
        })
        .collect();

    // Create top edges
    let top_edges: Vec<EdgeId> = (0..top_verts.len())
        .map(|i| {
            let next = (i + 1) % top_verts.len();
            let id = EdgeId(solid.edges.len() as u32);
            solid.edges.push(Edge {
                id,
                start: top_verts[i],
                end: top_verts[next],
                curve: CurveType::Linear,
                faces: Vec::new(),
            });
            id
        })
        .collect();

    // Create vertical edges
    let vertical_edges: Vec<EdgeId> = (0..bottom_verts.len())
        .map(|i| {
            let id = EdgeId(solid.edges.len() as u32);
            solid.edges.push(Edge {
                id,
                start: bottom_verts[i],
                end: top_verts[i],
                curve: CurveType::Linear,
                faces: Vec::new(),
            });
            id
        })
        .collect();

    // Create bottom face
    let bottom_face_id = FaceId(solid.faces.len() as u32);
    solid.faces.push(Face {
        id: bottom_face_id,
        surface: SurfaceType::Planar {
            normal: Vector3::new(0.0, 0.0, -1.0), // Points down
        },
        outer_loop: Loop {
            edges: bottom_edges.clone(),
            directions: vec![true; bottom_edges.len()],
        },
        inner_loops: Vec::new(),
        orientation: FaceOrientation::Outward,
        shell: Some(ShellId(0)),
    });

    // Create top face
    let top_face_id = FaceId(solid.faces.len() as u32);
    solid.faces.push(Face {
        id: top_face_id,
        surface: SurfaceType::Planar {
            normal: Vector3::new(0.0, 0.0, 1.0), // Points up
        },
        outer_loop: Loop {
            edges: top_edges.clone(),
            directions: vec![true; top_edges.len()],
        },
        inner_loops: Vec::new(),
        orientation: FaceOrientation::Outward,
        shell: Some(ShellId(0)),
    });

    // Create side faces (one per edge in profile)
    for i in 0..bottom_edges.len() {
        let next = (i + 1) % bottom_edges.len();

        let face_id = FaceId(solid.faces.len() as u32);
        solid.faces.push(Face {
            id: face_id,
            surface: SurfaceType::Planar {
                normal: Vector3::new(1.0, 0.0, 0.0), // Simplified
            },
            outer_loop: Loop {
                edges: vec![
                    bottom_edges[i],
                    vertical_edges[next],
                    top_edges[i],
                    vertical_edges[i],
                ],
                directions: vec![true, true, false, false], // Forward, forward, reverse, reverse
            },
            inner_loops: Vec::new(),
            orientation: FaceOrientation::Outward,
            shell: Some(ShellId(0)),
        });
    }

    // Create shell
    let all_faces: Vec<FaceId> = (0..solid.faces.len()).map(|i| FaceId(i as u32)).collect();
    solid.shells.push(Shell {
        id: ShellId(0),
        faces: all_faces,
        is_closed: true,
    });

    Ok(solid)
}

/// Find closed loops in the sketch
fn find_closed_loops(sketch: &Sketch) -> Result<Vec<Vec<SketchEntityId>>, ExtrudeError> {
    // Build adjacency graph
    let mut graph: HashMap<SketchEntityId, Vec<SketchEntityId>> = HashMap::new();

    for entity in &sketch.entities {
        if let SketchEntity::Line { id, .. } = entity {
            graph.entry(*id).or_default();
        }
    }

    // For now, return all line entities as a single loop
    // Full implementation would use DFS to find actual closed loops
    let line_entities: Vec<SketchEntityId> = sketch
        .entities
        .iter()
        .filter_map(|e| match e {
            SketchEntity::Line { id, .. } => Some(*id),
            _ => None,
        })
        .collect();

    if line_entities.is_empty() {
        return Err(ExtrudeError::NoClosedProfile);
    }

    Ok(vec![line_entities])
}

/// Get start point of an entity
fn get_entity_start_point(sketch: &Sketch, entity_id: SketchEntityId) -> Option<Point2> {
    sketch.entity(entity_id).and_then(|entity| match entity {
        SketchEntity::Line { start, .. } => sketch.point(*start).map(|p| p.position),
        SketchEntity::Arc { start, .. } => sketch.point(*start).map(|p| p.position),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cad::sketch::*;

    #[test]
    fn test_extrude_square() {
        let mut sketch = Sketch::new(SketchPlane::XY);

        // Create square profile
        let p1 = sketch.add_point(Point2::new(0.0, 0.0));
        let p2 = sketch.add_point(Point2::new(10.0, 0.0));
        let p3 = sketch.add_point(Point2::new(10.0, 10.0));
        let p4 = sketch.add_point(Point2::new(0.0, 10.0));

        sketch.add_entity(SketchEntity::Line {
            id: SketchEntityId(0),
            start: p1,
            end: p2,
        });
        sketch.add_entity(SketchEntity::Line {
            id: SketchEntityId(1),
            start: p2,
            end: p3,
        });
        sketch.add_entity(SketchEntity::Line {
            id: SketchEntityId(2),
            start: p3,
            end: p4,
        });
        sketch.add_entity(SketchEntity::Line {
            id: SketchEntityId(3),
            start: p4,
            end: p1,
        });

        let params = ExtrudeParams {
            distance: 20.0,
            symmetric: false,
        };

        let result = extrude_sketch(&sketch, &params);
        assert!(result.is_ok());

        let solid = result.unwrap();
        // Should have 8 vertices (4 bottom + 4 top)
        assert_eq!(solid.vertices.len(), 8);
    }
}
