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
pub mod primitives;
pub mod revolve;
pub mod pattern;
pub mod sketch;
pub mod solver;
pub mod topology;

// Re-export commonly used types
pub use geometry::{
    BoundingBox3, Line, Plane, Point3, Ray, Segment, Transform3, Vector3, TOLERANCE,
};
pub use primitives::{
    make_box, make_box_at, make_cone, make_cone_at, make_cylinder, make_cylinder_at, make_sphere,
    make_sphere_at,
};
pub use topology::{
    CurveType, Edge, EdgeId, Face, FaceId, FaceOrientation, Loop, Shell, ShellId, Solid,
    SurfaceType, Vertex, VertexId,
};
pub use mesh::{TriangleMesh, solid_to_mesh};
pub use intersect::{Classification, plane_plane_intersect, ray_sphere_intersect, ray_cylinder_intersect, point_in_solid};
pub use boolean::{BooleanOp, BooleanError, union, difference, intersection};
pub use sketch::{Sketch, SketchPlane, Point2, SketchPoint, SketchPointId, SketchEntity, SketchEntityId, ConstraintId};
pub use constraints::{Constraint, GeometricConstraint, DimensionalConstraint};
pub use solver::{ConstraintSolver, SolverConfig, SolverResult};
pub use extrude::{extrude_sketch, ExtrudeParams, ExtrudeError};
pub use revolve::{revolve_sketch, RevolveParams, RevolveAxis, RevolveError};
pub use pattern::{linear_pattern, circular_pattern};
