//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: mod.rs | DNA/src/cad/mod.rs
//! PURPOSE: B-Rep (Boundary Representation) CAD kernel for solid modeling
//! MODIFIED: 2025-12-09
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

//!
//! PURPOSE: B-Rep (Boundary Representation) CAD kernel for solid modeling
//!
//! LAYER: DNA → CAD
//!
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │ MODULE STRUCTURE                                                            │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │                                                                             │
//! │   cad/                                                                      │
//! │   ├── geometry.rs    3D geometry primitives (Point3, Vector3, Plane, etc.) │
//! │   ├── topology.rs    B-Rep topology (Vertex, Edge, Face, Shell, Solid)     │
//! │   ├── primitives.rs  Solid generators (box, cylinder, sphere, cone)        │
//! │   ├── mesh.rs        Mesh triangulation for export/rendering               │
//! │   ├── intersect.rs   Geometric intersection algorithms                     │
//! │   ├── boolean.rs     Boolean operations (union, difference, intersection)  │
//! │   ├── sketch.rs      2D parametric sketch (Point2, SketchEntity)           │
//! │   ├── constraints.rs Sketch constraints (geometric, dimensional)           │
//! │   ├── solver.rs      Constraint solver (Newton-Raphson)                    │
//! │   └── extrude.rs     Sketch extrusion (2D → 3D Solid)                      │
//! │                                                                             │
//! └─────────────────────────────────────────────────────────────────────────────┘
//!
//! DEPENDS ON:
//!   • glam → Vector/matrix math
//!
//! USED BY:
//!   • CORE/CAD_ENGINE   → High-level CAD operations
//!   • TOOLS/AUTOCRATE   → Shipping crate geometry
//!   • MCAD              → Professional CAD application
//!
//! ═══════════════════════════════════════════════════════════════════════════════

pub mod boolean;
pub mod constraints;
pub mod extrude;
pub mod geometry;
pub mod intersect;
pub mod mesh;
pub mod pattern;
pub mod primitives;
pub mod revolve;
pub mod sketch;
pub mod solver;
pub mod topology;

// Re-export commonly used types
pub use boolean::{difference, intersection, union, BooleanError, BooleanOp};
pub use constraints::{Constraint, DimensionalConstraint, GeometricConstraint};
pub use extrude::{extrude_sketch, ExtrudeError, ExtrudeParams};
pub use geometry::{
    BoundingBox3, Line, Plane, Point3, Ray, Segment, Transform3, Vector3, TOLERANCE,
};
pub use intersect::{
    pick_face, plane_plane_intersect, point_in_solid, ray_cylinder_intersect, ray_sphere_intersect,
    ray_triangle_intersect, Classification, FaceHit,
};
pub use mesh::{solid_to_mesh, solid_to_pickable_mesh, PickableMesh, TriangleMesh};
pub use pattern::{circular_pattern, linear_pattern};
pub use primitives::{
    make_box, make_box_at, make_cone, make_cone_at, make_cylinder, make_cylinder_at, make_sphere,
    make_sphere_at,
};
pub use revolve::{revolve_sketch, RevolveAxis, RevolveError, RevolveParams};
pub use sketch::{
    ConstraintId, Point2, Sketch, SketchCoordinateFrame, SketchEntity, SketchEntityId, SketchPlane,
    SketchPoint, SketchPointId,
};
pub use solver::{ConstraintAnalysis, ConstraintSolver, DofStatus, SolverConfig, SolverResult};
pub use topology::{
    CurveType, Edge, EdgeId, Face, FaceId, FaceOrientation, Loop, Shell, ShellId, Solid,
    SurfaceType, Vertex, VertexId,
};
