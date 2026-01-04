//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: revolve.rs | DNA/src/cad/revolve.rs
//! PURPOSE: Revolve a 2D sketch profile around an axis to create a 3D solid
//! MODIFIED: 2026-01-04
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

use super::geometry::{Point3, Vector3};
use super::sketch::{Point2, Sketch, SketchEntity};
use super::topology::{
    CurveType, Edge, EdgeId, Face, FaceId, FaceOrientation, Loop, Shell, ShellId, Solid, SurfaceType,
    Vertex, VertexId,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RevolveParams {
    /// Degrees to revolve (360 for full).
    pub angle_degrees: f32,
    /// Axis of revolution.
    pub axis: RevolveAxis,
    /// Number of angular segments for tessellation (>= 3).
    pub segments: u32,
}

impl Default for RevolveParams {
    fn default() -> Self {
        Self {
            angle_degrees: 360.0,
            axis: RevolveAxis::Y,
            segments: 32,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum RevolveAxis {
    X,
    Y,
}

#[derive(Debug, Clone)]
pub enum RevolveError {
    NoProfile,
    InvalidParams,
}

pub fn revolve_sketch(sketch: &Sketch, params: &RevolveParams) -> Result<Solid, RevolveError> {
    if params.segments < 3 || params.angle_degrees.abs() < 1e-6 {
        return Err(RevolveError::InvalidParams);
    }

    let profile = extract_profile_polyline(sketch)?;
    if profile.len() < 2 {
        return Err(RevolveError::NoProfile);
    }

    let angle_rad = params.angle_degrees.to_radians();
    let full = (params.angle_degrees - 360.0).abs() < 1e-3;
    let segs = if full { params.segments } else { params.segments.max(1) };
    let steps = if full { segs } else { segs + 1 };

    let mut solid = Solid::new();

    // Create all vertices: [step][i]
    let mut vid: Vec<Vec<VertexId>> = vec![vec![VertexId(0); profile.len()]; steps as usize];
    for s in 0..steps {
        let t = s as f32 / (segs as f32);
        let theta = t * angle_rad;
        for (i, p) in profile.iter().enumerate() {
            let pt = revolve_point_2d(*p, theta, params.axis);
            let id = VertexId(solid.vertices.len() as u32);
            solid.vertices.push(Vertex {
                id,
                point: pt,
                edges: Vec::new(),
            });
            vid[s as usize][i] = id;
        }
    }

    // Helper to add edge and return id.
    let mut add_edge = |start: VertexId, end: VertexId| -> EdgeId {
        let id = EdgeId(solid.edges.len() as u32);
        solid.edges.push(Edge {
            id,
            start,
            end,
            curve: CurveType::Linear,
            faces: Vec::new(),
        });
        id
    };

    // Build side faces between each angular slice and along each segment in the profile.
    // Each quad -> face with 4 edges (duplicated edges are fine for now).
    for s in 0..segs {
        let s0 = s as usize;
        let s1 = if full {
            ((s + 1) % segs) as usize
        } else {
            (s + 1) as usize
        };
        for i in 0..(profile.len() - 1) {
            let a = vid[s0][i];
            let b = vid[s0][i + 1];
            let c = vid[s1][i + 1];
            let d = vid[s1][i];

            let e0 = add_edge(a, b);
            let e1 = add_edge(b, c);
            let e2 = add_edge(c, d);
            let e3 = add_edge(d, a);

            let fid = FaceId(solid.faces.len() as u32);
            solid.faces.push(Face {
                id: fid,
                surface: SurfaceType::Planar {
                    normal: Vector3::Z, // Simplified placeholder
                },
                outer_loop: Loop {
                    edges: vec![e0, e1, e2, e3],
                    directions: vec![true, true, true, true],
                },
                inner_loops: Vec::new(),
                orientation: FaceOrientation::Outward,
                shell: Some(ShellId(0)),
            });
        }
    }

    // Caps for partial revolve
    if !full {
        // Start cap at theta = 0
        add_cap_face(&mut solid, &vid[0], true);
        // End cap at theta = angle
        add_cap_face(&mut solid, &vid[steps as usize - 1], false);
    }

    // Single shell containing all faces
    let all_faces: Vec<FaceId> = (0..solid.faces.len()).map(|i| FaceId(i as u32)).collect();
    solid.shells.push(Shell {
        id: ShellId(0),
        faces: all_faces,
        is_closed: full, // partial might still be closed if profile hits axis; keep conservative
    });

    Ok(solid)
}

fn extract_profile_polyline(sketch: &Sketch) -> Result<Vec<Point2>, RevolveError> {
    // Minimal v1: take all line entities in order of appearance and use their start points.
    let mut pts: Vec<Point2> = Vec::new();
    for entity in &sketch.entities {
        if let SketchEntity::Line { start, .. } = entity {
            if let Some(p) = sketch.point(*start) {
                pts.push(p.position);
            }
        }
    }
    if pts.is_empty() {
        return Err(RevolveError::NoProfile);
    }
    // Add last end point if possible to make a polyline
    if let Some(SketchEntity::Line { end, .. }) = sketch.entities.iter().rev().find(|e| matches!(e, SketchEntity::Line { .. })) {
        if let Some(p) = sketch.point(*end) {
            pts.push(p.position);
        }
    }
    Ok(pts)
}

fn revolve_point_2d(p: Point2, theta: f32, axis: RevolveAxis) -> Point3 {
    let (s, c) = theta.sin_cos();
    match axis {
        RevolveAxis::Y => {
            // Revolve around Y: (x, y) in sketch becomes radius=x, height=y
            Point3::new(p.x * c, p.y, p.x * s)
        }
        RevolveAxis::X => {
            // Revolve around X: radius=y, height=x
            Point3::new(p.x, p.y * c, p.y * s)
        }
    }
}

fn add_cap_face(solid: &mut Solid, ring: &[VertexId], start_cap: bool) {
    if ring.len() < 3 {
        return;
    }

    // Create a fan from the first vertex (not robust, but enough for visualization/tests).
    let mut edges: Vec<EdgeId> = Vec::new();
    for i in 1..(ring.len() - 1) {
        let a = ring[0];
        let b = ring[i];
        let c = ring[i + 1];
        let e0 = EdgeId(solid.edges.len() as u32);
        solid.edges.push(Edge { id: e0, start: a, end: b, curve: CurveType::Linear, faces: Vec::new() });
        let e1 = EdgeId(solid.edges.len() as u32);
        solid.edges.push(Edge { id: e1, start: b, end: c, curve: CurveType::Linear, faces: Vec::new() });
        let e2 = EdgeId(solid.edges.len() as u32);
        solid.edges.push(Edge { id: e2, start: c, end: a, curve: CurveType::Linear, faces: Vec::new() });

        edges.push(e0);
        edges.push(e1);
        edges.push(e2);
    }

    let edge_count = edges.len();
    let fid = FaceId(solid.faces.len() as u32);
    solid.faces.push(Face {
        id: fid,
        surface: SurfaceType::Planar { normal: Vector3::Z },
        outer_loop: Loop { edges, directions: vec![true; edge_count] },
        inner_loops: Vec::new(),
        // v1: keep orientation simple for now; topology validity checks don't inspect normals.
        orientation: if start_cap { FaceOrientation::Outward } else { FaceOrientation::Outward },
        shell: Some(ShellId(0)),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cad::sketch::{SketchPlane, SketchEntityId};

    #[test]
    fn test_revolve_simple_profile() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p0 = sketch.add_point(Point2::new(10.0, 0.0));
        let p1 = sketch.add_point(Point2::new(10.0, 20.0));
        sketch.add_entity(SketchEntity::Line { id: SketchEntityId(0), start: p0, end: p1 });

        let solid = revolve_sketch(&sketch, &RevolveParams { segments: 8, ..RevolveParams::default() }).unwrap();
        assert!(solid.vertices.len() > 0);
        assert!(solid.faces.len() > 0);
        assert!(solid.is_valid());
    }
}


