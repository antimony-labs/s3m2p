//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mesh.rs | DNA/src/cad/mesh.rs
//! PURPOSE: Mesh triangulation for B-Rep solids
//! MODIFIED: 2026-01-04
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

use super::geometry::{Point3, Vector3};
use super::topology::{EdgeId, Face, FaceId, Solid};

/// Triangle mesh representation
#[derive(Clone, Debug)]
pub struct TriangleMesh {
    pub vertices: Vec<Point3>,
    pub triangles: Vec<[usize; 3]>,
    pub normals: Vec<Vector3>,
}

impl TriangleMesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
            normals: Vec::new(),
        }
    }

    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }
}

impl Default for TriangleMesh {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert a solid to a triangle mesh
///
/// This triangulates all faces using simple fan triangulation from the first vertex.
/// Works well for convex polygons and simple shapes.
pub fn solid_to_mesh(solid: &Solid) -> TriangleMesh {
    let mut mesh = TriangleMesh::new();

    for face in &solid.faces {
        triangulate_face(face, solid, &mut mesh);
    }

    mesh
}

/// Triangulate a single face and add to mesh
fn triangulate_face(face: &Face, solid: &Solid, mesh: &mut TriangleMesh) {
    // Get all vertices in the face's outer loop
    let face_vertices: Vec<Point3> = face
        .outer_loop
        .edges
        .iter()
        .filter_map(|&edge_id| {
            solid
                .edge(edge_id)
                .and_then(|e| solid.vertex(e.start))
                .map(|v| v.point)
        })
        .collect();

    if face_vertices.len() < 3 {
        return; // Degenerate face
    }

    // Calculate face normal from first three vertices
    let v0 = face_vertices[0].to_vec3();
    let v1 = face_vertices[1].to_vec3();
    let v2 = face_vertices[2].to_vec3();

    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let normal = edge1.cross(edge2);

    let normal = if normal.length() > 1e-8 {
        Vector3::from_vec3(normal.normalize())
    } else {
        Vector3::new(0.0, 0.0, 1.0)
    };

    // Add vertices to mesh and remember base index
    let base_idx = mesh.vertices.len();
    mesh.vertices.extend_from_slice(&face_vertices);

    // Fan triangulation from first vertex
    for i in 1..face_vertices.len() - 1 {
        mesh.triangles
            .push([base_idx, base_idx + i, base_idx + i + 1]);
        mesh.normals.push(normal);
    }
}

/// Triangle mesh with face provenance for picking
#[derive(Clone, Debug)]
pub struct PickableMesh {
    /// The underlying triangle mesh
    pub mesh: TriangleMesh,
    /// For each triangle, the source face ID
    pub triangle_to_face: Vec<FaceId>,
    /// Edge segments for edge picking (start, end, edge_id)
    pub edge_segments: Vec<(Point3, Point3, EdgeId)>,
}

impl PickableMesh {
    pub fn new() -> Self {
        Self {
            mesh: TriangleMesh::new(),
            triangle_to_face: Vec::new(),
            edge_segments: Vec::new(),
        }
    }
}

impl Default for PickableMesh {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert a solid to a pickable mesh with face provenance
pub fn solid_to_pickable_mesh(solid: &Solid) -> PickableMesh {
    let mut pickable = PickableMesh::new();

    // Triangulate each face and track which face each triangle belongs to
    for face in &solid.faces {
        let start_tri_idx = pickable.mesh.triangles.len();
        triangulate_face(face, solid, &mut pickable.mesh);
        let end_tri_idx = pickable.mesh.triangles.len();

        // All triangles from start_tri_idx to end_tri_idx belong to this face
        for _ in start_tri_idx..end_tri_idx {
            pickable.triangle_to_face.push(face.id);
        }
    }

    // Extract edge segments for edge picking
    for edge in &solid.edges {
        if let (Some(start_v), Some(end_v)) = (solid.vertex(edge.start), solid.vertex(edge.end)) {
            pickable
                .edge_segments
                .push((start_v.point, end_v.point, edge.id));
        }
    }

    pickable
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cad::primitives::make_box;

    #[test]
    fn test_box_to_mesh() {
        let solid = make_box(2.0, 3.0, 4.0);
        let mesh = solid_to_mesh(&solid);

        // A box has 6 faces, each triangulated to 2 triangles = 12 triangles
        assert!(mesh.triangle_count() >= 6);
        assert!(mesh.vertex_count() > 0);
        assert_eq!(mesh.triangles.len(), mesh.normals.len());
    }

    #[test]
    fn test_mesh_validity() {
        let solid = make_box(1.0, 1.0, 1.0);
        let mesh = solid_to_mesh(&solid);

        // All triangle indices should be valid
        for tri in &mesh.triangles {
            assert!(tri[0] < mesh.vertices.len());
            assert!(tri[1] < mesh.vertices.len());
            assert!(tri[2] < mesh.vertices.len());
        }
    }
}
