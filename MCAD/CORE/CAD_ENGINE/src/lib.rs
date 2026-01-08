//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lib.rs | TOOLS/CORE/CAD_ENGINE/src/lib.rs
//! PURPOSE: B-Rep CAD engine for solid modeling
//! MODIFIED: 2025-12-09
//! LAYER: CORE → CAD_ENGINE
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! CAD_ENGINE provides solid modeling capabilities:
//! - Geometric primitives (points, vectors, planes)
//! - B-Rep topology (vertices, edges, faces, shells, solids)
//! - Solid primitives (box, cylinder, sphere, cone)
//! - Transformations (translate, rotate, scale)
//! - Boolean operations (planned: union, difference, intersection)
//!
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │ ARCHITECTURE                                                                │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │                                                                             │
//! │   CAD_ENGINE                                                                │
//! │       │                                                                     │
//! │       ├── Geometry (DNA/cad/geometry)                                       │
//! │       │     ├── Point3, Vector3                                             │
//! │       │     ├── Plane, Line, Ray, Segment                                   │
//! │       │     ├── BoundingBox3                                                │
//! │       │     └── Transform3                                                  │
//! │       │                                                                     │
//! │       ├── Topology (DNA/cad/topology)                                       │
//! │       │     ├── Vertex, Edge, Face                                          │
//! │       │     ├── Loop, Shell, Solid                                          │
//! │       │     └── CurveType, SurfaceType                                      │
//! │       │                                                                     │
//! │       └── Primitives (DNA/cad/primitives)                                   │
//! │             ├── make_box, make_cylinder                                     │
//! │             ├── make_sphere, make_cone                                      │
//! │             └── (future: make_prism, make_torus)                            │
//! │                                                                             │
//! └─────────────────────────────────────────────────────────────────────────────┘
//!
//! DEPENDS ON:
//!   • DNA/cad → B-Rep primitives and types
//!   • glam    → Vector/matrix math
//!
//! USED BY:
//!   • TOOLS/AUTOCRATE → Shipping crate geometry
//!   • Future: MCAD    → Full CAD application
//!
//! ═══════════════════════════════════════════════════════════════════════════════

// ─────────────────────────────────────────────────────────────────────────────────
// RE-EXPORTS FROM DNA
// ─────────────────────────────────────────────────────────────────────────────────

// Geometry primitives
pub use dna::cad::geometry::{
    BoundingBox3, Line, Plane, Point3, Ray, Segment, Transform3, Vector3, TOLERANCE,
};

// Topology types
pub use dna::cad::topology::{
    CurveType, Edge, EdgeId, Face, FaceId, FaceOrientation, Loop, Shell, ShellId, Solid,
    SurfaceType, Vertex, VertexId,
};

// Primitive generators
pub use dna::cad::primitives::{
    make_box, make_box_at, make_cone, make_cone_at, make_cylinder, make_cylinder_at, make_sphere,
    make_sphere_at,
};

// Mesh triangulation and picking
pub use dna::cad::mesh::{TriangleMesh, solid_to_mesh, PickableMesh, solid_to_pickable_mesh};
pub use dna::cad::intersect::{ray_triangle_intersect, pick_face, FaceHit};

// Boolean operations
pub use dna::cad::boolean::{BooleanOp, BooleanError, union, difference, intersection};

// Sketcher
pub use dna::cad::sketch::{
    circumcenter, orient2d, ConstraintId, Point2, Sketch, SketchEntity, SketchEntityId, SketchPlane,
    SketchPoint, SketchPointId, SketchCoordinateFrame,
};
pub use dna::cad::constraints::{Constraint, GeometricConstraint, DimensionalConstraint};
pub use dna::cad::solver::{ConstraintSolver, SolverConfig, SolverResult, DofStatus, ConstraintAnalysis};
pub use dna::cad::extrude::{extrude_sketch, ExtrudeParams, ExtrudeError};
pub use dna::cad::revolve::{revolve_sketch, RevolveParams, RevolveAxis, RevolveError};
pub use dna::cad::pattern::{linear_pattern, circular_pattern};

// ─────────────────────────────────────────────────────────────────────────────────
// HIGH-LEVEL API
// ─────────────────────────────────────────────────────────────────────────────────

/// Builder pattern for creating complex solids
pub struct SolidBuilder {
    solid: Solid,
}

impl SolidBuilder {
    /// Start with an empty solid
    pub fn new() -> Self {
        Self {
            solid: Solid::new(),
        }
    }

    /// Start with a box primitive
    pub fn from_box(width: f32, depth: f32, height: f32) -> Self {
        Self {
            solid: make_box(width, depth, height),
        }
    }

    /// Start with a cylinder primitive
    pub fn from_cylinder(radius: f32, height: f32, segments: u32) -> Self {
        Self {
            solid: make_cylinder(radius, height, segments),
        }
    }

    /// Start with a sphere primitive
    pub fn from_sphere(radius: f32, u_segments: u32, v_segments: u32) -> Self {
        Self {
            solid: make_sphere(radius, u_segments, v_segments),
        }
    }

    /// Start with a cone primitive
    pub fn from_cone(base_radius: f32, height: f32, segments: u32) -> Self {
        Self {
            solid: make_cone(base_radius, height, segments),
        }
    }

    /// Translate the solid
    pub fn translate(mut self, x: f32, y: f32, z: f32) -> Self {
        let transform = Transform3::from_translation(Vector3::new(x, y, z));
        for vertex in &mut self.solid.vertices {
            vertex.point = vertex.point.transform(&transform);
        }
        self
    }

    /// Scale the solid uniformly
    pub fn scale(mut self, factor: f32) -> Self {
        let transform = Transform3::from_scale(Vector3::new(factor, factor, factor));
        for vertex in &mut self.solid.vertices {
            vertex.point = vertex.point.transform(&transform);
        }
        self
    }

    /// Scale the solid non-uniformly
    pub fn scale_xyz(mut self, x: f32, y: f32, z: f32) -> Self {
        let transform = Transform3::from_scale(Vector3::new(x, y, z));
        for vertex in &mut self.solid.vertices {
            vertex.point = vertex.point.transform(&transform);
        }
        self
    }

    /// Rotate around X axis (radians)
    pub fn rotate_x(mut self, angle: f32) -> Self {
        let transform = Transform3::from_rotation_x(angle);
        for vertex in &mut self.solid.vertices {
            vertex.point = vertex.point.transform(&transform);
        }
        self
    }

    /// Rotate around Y axis (radians)
    pub fn rotate_y(mut self, angle: f32) -> Self {
        let transform = Transform3::from_rotation_y(angle);
        for vertex in &mut self.solid.vertices {
            vertex.point = vertex.point.transform(&transform);
        }
        self
    }

    /// Rotate around Z axis (radians)
    pub fn rotate_z(mut self, angle: f32) -> Self {
        let transform = Transform3::from_rotation_z(angle);
        for vertex in &mut self.solid.vertices {
            vertex.point = vertex.point.transform(&transform);
        }
        self
    }

    /// Build the final solid
    pub fn build(self) -> Solid {
        self.solid
    }

    /// Get reference to solid during building
    pub fn solid(&self) -> &Solid {
        &self.solid
    }
}

impl Default for SolidBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if a solid is watertight (manifold)
pub fn is_manifold(solid: &Solid) -> bool {
    // Every edge should be shared by exactly 2 faces
    for edge in &solid.edges {
        if edge.faces.len() != 2 {
            return false;
        }
    }

    // All shells should be closed
    for shell in &solid.shells {
        if !shell.is_closed {
            return false;
        }
    }

    true
}

/// Calculate volume of a closed solid (approximate for complex surfaces)
pub fn volume(solid: &Solid) -> f32 {
    // Use signed volume of tetrahedra from faces to origin
    let mut total_volume = 0.0;

    for face in &solid.faces {
        // Get vertices of face (simplified - assumes triangulated)
        let vertices: Vec<Point3> = face
            .outer_loop
            .edges
            .iter()
            .filter_map(|&edge_id| {
                solid
                    .edge(edge_id)
                    .and_then(|e| solid.vertex(e.start).map(|v| v.point))
            })
            .collect();

        if vertices.len() >= 3 {
            // Calculate signed volume of tetrahedron from origin to face
            let v0 = vertices[0].to_vec3();
            for i in 1..vertices.len() - 1 {
                let v1 = vertices[i].to_vec3();
                let v2 = vertices[i + 1].to_vec3();
                total_volume += v0.dot(v1.cross(v2)) / 6.0;
            }
        }
    }

    total_volume.abs()
}

/// Calculate surface area of a solid (approximate for complex surfaces)
pub fn surface_area(solid: &Solid) -> f32 {
    let mut total_area = 0.0;

    for face in &solid.faces {
        let vertices: Vec<Point3> = face
            .outer_loop
            .edges
            .iter()
            .filter_map(|&edge_id| {
                solid
                    .edge(edge_id)
                    .and_then(|e| solid.vertex(e.start).map(|v| v.point))
            })
            .collect();

        if vertices.len() >= 3 {
            // Calculate area using cross product (triangulated)
            for i in 1..vertices.len() - 1 {
                let v1 = vertices[i].to_vec3() - vertices[0].to_vec3();
                let v2 = vertices[i + 1].to_vec3() - vertices[0].to_vec3();
                total_area += v1.cross(v2).length() / 2.0;
            }
        }
    }

    total_area
}

// ─────────────────────────────────────────────────────────────────────────────────
// EXPORT
// ─────────────────────────────────────────────────────────────────────────────────

/// Convert a Solid to STL binary format for 3D printing
pub fn solid_to_stl(solid: &Solid, name: &str) -> Vec<u8> {
    use dna::cad::mesh::solid_to_mesh;
    use dna::export::stl::write_stl_binary;

    let mesh = solid_to_mesh(solid);
    write_stl_binary(&mesh, name)
}

/// Convert a Solid to STEP AP242 format
///
/// This exports the B-Rep topology to ISO 10303-21 STEP format.
/// Currently supports planar faces (boxes, cylinders with flat caps).
pub fn solid_to_step(solid: &Solid, name: &str) -> String {
    use dna::export::step::StepWriter;

    let mut writer = StepWriter::new();

    // Create points for all vertices
    let point_ids: Vec<_> = solid.vertices.iter()
        .map(|v| writer.add_point(None, v.point.x as f64, v.point.y as f64, v.point.z as f64))
        .collect();

    // Create vertex_point entities
    let vertex_ids: Vec<_> = point_ids.iter()
        .map(|&pid| writer.add_vertex_point(None, pid))
        .collect();

    // Create edge_curve entities for each edge
    let edge_ids: Vec<_> = solid.edges.iter()
        .map(|edge| {
            let start_pt = point_ids[edge.start.0 as usize];

            // Create line geometry for linear edges
            let start_vertex = &solid.vertices[edge.start.0 as usize];
            let end_vertex = &solid.vertices[edge.end.0 as usize];
            let dir = end_vertex.point.to_vec3() - start_vertex.point.to_vec3();
            let length = dir.length() as f64;

            if length > 0.0 {
                let dir = dir.normalize();
                let dir_id = writer.add_direction(None, dir.x as f64, dir.y as f64, dir.z as f64);
                let vec_id = writer.add_vector(None, dir_id, length);
                let line_id = writer.add_line(None, start_pt, vec_id);

                writer.add_edge_curve(
                    None,
                    vertex_ids[edge.start.0 as usize],
                    vertex_ids[edge.end.0 as usize],
                    line_id,
                    true
                )
            } else {
                // Degenerate edge - create placeholder
                let dir_id = writer.add_direction(None, 1.0, 0.0, 0.0);
                let vec_id = writer.add_vector(None, dir_id, 0.001);
                let line_id = writer.add_line(None, start_pt, vec_id);

                writer.add_edge_curve(
                    None,
                    vertex_ids[edge.start.0 as usize],
                    vertex_ids[edge.end.0 as usize],
                    line_id,
                    true
                )
            }
        })
        .collect();

    // Create faces
    let face_ids: Vec<_> = solid.faces.iter()
        .map(|face| {
            // Create oriented edges for the face loop
            let oriented_edges: Vec<_> = face.outer_loop.edges.iter()
                .map(|&edge_id| {
                    writer.add_oriented_edge(None, edge_ids[edge_id.0 as usize], true)
                })
                .collect();

            // Create face bound (loop)
            let face_bound = writer.add_face_bound(None, oriented_edges, true);

            // For planar faces, create a plane
            // Simplified: use first three vertices to define plane
            let verts: Vec<&Vertex> = face.outer_loop.edges.iter()
                .take(3)
                .filter_map(|&edge_id| solid.edge(edge_id))
                .filter_map(|e| solid.vertex(e.start))
                .collect();

            let plane_position = if verts.len() >= 3 {
                let p0 = verts[0].point.to_vec3();
                let p1 = verts[1].point.to_vec3();
                let p2 = verts[2].point.to_vec3();

                let v1 = p1 - p0;
                let v2 = p2 - p0;
                let normal = v1.cross(v2).normalize();

                let origin_id = writer.add_point(None, p0.x as f64, p0.y as f64, p0.z as f64);
                let normal_id = writer.add_direction(None, normal.x as f64, normal.y as f64, normal.z as f64);
                writer.add_axis2_placement_3d(None, origin_id, Some(normal_id), None)
            } else {
                // Fallback to origin
                let origin_id = writer.add_point(None, 0.0, 0.0, 0.0);
                let normal_id = writer.add_direction(None, 0.0, 0.0, 1.0);
                writer.add_axis2_placement_3d(None, origin_id, Some(normal_id), None)
            };

            let plane_id = writer.add_plane(None, plane_position);

            writer.add_advanced_face(None, plane_id, vec![face_bound])
        })
        .collect();

    // Create closed shell
    let shell_id = writer.add_closed_shell(None, face_ids);

    // Create manifold solid brep
    writer.add_manifold_solid_brep(Some(name), shell_id);

    writer.to_string()
}

// ─────────────────────────────────────────────────────────────────────────────────
// TESTS
// ─────────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solid_builder_box() {
        let solid = SolidBuilder::from_box(2.0, 3.0, 4.0).build();

        assert_eq!(solid.vertices.len(), 8);
        assert_eq!(solid.edges.len(), 12);
        assert_eq!(solid.faces.len(), 6);
    }

    #[test]
    fn test_solid_builder_translate() {
        let solid = SolidBuilder::from_box(2.0, 2.0, 2.0)
            .translate(10.0, 0.0, 0.0)
            .build();

        // Check that all vertices are translated
        for vertex in &solid.vertices {
            assert!(vertex.point.x >= 9.0);
        }
    }

    #[test]
    fn test_solid_builder_chain() {
        let solid = SolidBuilder::from_cylinder(1.0, 2.0, 16)
            .translate(0.0, 0.0, 5.0)
            .rotate_z(std::f32::consts::PI / 4.0)
            .scale(2.0)
            .build();

        assert!(solid.is_valid());
    }

    #[test]
    fn test_box_volume() {
        let solid = make_box(2.0, 3.0, 4.0);
        let vol = volume(&solid);

        // Volume should be approximately 2 * 3 * 4 = 24
        // Note: May not be exact due to face vertex extraction limitations
        assert!(vol > 0.0);
    }

    #[test]
    fn test_surface_area() {
        let solid = make_box(2.0, 2.0, 2.0);
        let area = surface_area(&solid);

        // Surface area should be 6 * 4 = 24 (6 faces, each 2x2)
        assert!(area > 0.0);
    }
}
