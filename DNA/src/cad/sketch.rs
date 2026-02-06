//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: sketch.rs | DNA/src/cad/sketch.rs
//! PURPOSE: 2D parametric sketch for CAD modeling
//! MODIFIED: 2026-01-04
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

use super::geometry::{Point3, Vector3};
use super::topology::{Face, Solid};
use serde::{Deserialize, Serialize};

/// Coordinate frame for arbitrary sketch planes
///
/// Defines a local 2D coordinate system embedded in 3D space.
/// The U and V axes define the sketch plane, with the normal perpendicular to both.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SketchCoordinateFrame {
    /// Origin point in 3D world space
    pub origin: Point3,
    /// Normal vector (perpendicular to sketch plane)
    pub normal: Vector3,
    /// U axis (local X direction in sketch)
    pub u_axis: Vector3,
    /// V axis (local Y direction in sketch)
    pub v_axis: Vector3,
}

impl SketchCoordinateFrame {
    /// Create a coordinate frame from origin and normal
    ///
    /// Automatically computes orthogonal U and V axes.
    pub fn from_origin_normal(origin: Point3, normal: Vector3) -> Option<Self> {
        // Normalize the normal
        let len = normal.length();
        if len < 1e-8 {
            return None;
        }
        let normal = Vector3::new(normal.x / len, normal.y / len, normal.z / len);

        // Find a vector not parallel to normal to create U axis
        let reference = if normal.x.abs() < 0.9 {
            Vector3::new(1.0, 0.0, 0.0)
        } else {
            Vector3::new(0.0, 1.0, 0.0)
        };

        // U = reference × normal (normalized)
        let u = Vector3::new(
            reference.y * normal.z - reference.z * normal.y,
            reference.z * normal.x - reference.x * normal.z,
            reference.x * normal.y - reference.y * normal.x,
        );
        let u_len = u.length();
        if u_len < 1e-8 {
            return None;
        }
        let u_axis = Vector3::new(u.x / u_len, u.y / u_len, u.z / u_len);

        // V = normal × U (already normalized since both inputs are unit vectors)
        let v_axis = Vector3::new(
            normal.y * u_axis.z - normal.z * u_axis.y,
            normal.z * u_axis.x - normal.x * u_axis.z,
            normal.x * u_axis.y - normal.y * u_axis.x,
        );

        Some(Self {
            origin,
            normal,
            u_axis,
            v_axis,
        })
    }

    /// Create a coordinate frame from a face on a solid
    ///
    /// Uses the face's planar surface normal and centroid as origin.
    pub fn from_face(face: &Face, solid: &Solid) -> Option<Self> {
        // Get first 3 vertices to compute face normal
        let verts: Vec<Point3> = face
            .outer_loop
            .edges
            .iter()
            .take(3)
            .filter_map(|&edge_id| {
                solid
                    .edge(edge_id)
                    .and_then(|e| solid.vertex(e.start))
                    .map(|v| v.point)
            })
            .collect();

        if verts.len() < 3 {
            return None;
        }

        // Compute face normal from first 3 vertices
        let v0 = verts[0].to_vec3();
        let v1 = verts[1].to_vec3();
        let v2 = verts[2].to_vec3();

        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let cross = edge1.cross(edge2);

        if cross.length() < 1e-8 {
            return None; // Degenerate face
        }

        let normal = Vector3::from_vec3(cross.normalize());

        // Compute centroid as origin
        let all_verts: Vec<Point3> = face
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

        if all_verts.is_empty() {
            return None;
        }

        let mut cx = 0.0f32;
        let mut cy = 0.0f32;
        let mut cz = 0.0f32;
        for v in &all_verts {
            cx += v.x;
            cy += v.y;
            cz += v.z;
        }
        let n = all_verts.len() as f32;
        let origin = Point3::new(cx / n, cy / n, cz / n);

        Self::from_origin_normal(origin, normal)
    }

    /// Create an offset plane (parallel to this frame, shifted along normal)
    pub fn with_offset(&self, distance: f32) -> Self {
        Self {
            origin: Point3::new(
                self.origin.x + self.normal.x * distance,
                self.origin.y + self.normal.y * distance,
                self.origin.z + self.normal.z * distance,
            ),
            normal: self.normal,
            u_axis: self.u_axis,
            v_axis: self.v_axis,
        }
    }

    /// Transform a 2D sketch point to 3D world space
    pub fn to_3d(&self, p: Point2) -> Point3 {
        Point3::new(
            self.origin.x + self.u_axis.x * p.x + self.v_axis.x * p.y,
            self.origin.y + self.u_axis.y * p.x + self.v_axis.y * p.y,
            self.origin.z + self.u_axis.z * p.x + self.v_axis.z * p.y,
        )
    }

    /// Transform a 3D world point to 2D sketch coordinates
    ///
    /// Projects the point onto the sketch plane.
    pub fn from_3d(&self, p: Point3) -> Point2 {
        // Vector from origin to point
        let v = Vector3::new(
            p.x - self.origin.x,
            p.y - self.origin.y,
            p.z - self.origin.z,
        );

        // Project onto U and V axes
        let u = v.x * self.u_axis.x + v.y * self.u_axis.y + v.z * self.u_axis.z;
        let v_coord = v.x * self.v_axis.x + v.y * self.v_axis.y + v.z * self.v_axis.z;

        Point2::new(u, v_coord)
    }
}

impl PartialEq for SketchCoordinateFrame {
    fn eq(&self, other: &Self) -> bool {
        const TOL: f32 = 1e-6;
        (self.origin.x - other.origin.x).abs() < TOL
            && (self.origin.y - other.origin.y).abs() < TOL
            && (self.origin.z - other.origin.z).abs() < TOL
            && (self.normal.x - other.normal.x).abs() < TOL
            && (self.normal.y - other.normal.y).abs() < TOL
            && (self.normal.z - other.normal.z).abs() < TOL
    }
}

/// Sketch plane orientation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SketchPlane {
    XY, // Normal: +Z
    YZ, // Normal: +X
    XZ, // Normal: +Y
    /// Arbitrary plane defined by a coordinate frame
    Arbitrary(SketchCoordinateFrame),
}

/// 2D point in sketch coordinates
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Point2 {
    pub x: f32,
    pub y: f32,
}

impl Point2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Point2) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn distance_squared(&self, other: &Point2) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    pub fn lerp(&self, other: &Point2, t: f32) -> Point2 {
        Point2::new(
            self.x + (other.x - self.x) * t,
            self.y + (other.y - self.y) * t,
        )
    }

    pub fn midpoint(&self, other: &Point2) -> Point2 {
        self.lerp(other, 0.5)
    }
}

/// Handle for sketch points
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SketchPointId(pub u32);

/// Handle for sketch entities
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SketchEntityId(pub u32);

/// Handle for constraints
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConstraintId(pub u32);

/// A point in the sketch
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SketchPoint {
    pub id: SketchPointId,
    pub position: Point2,
    pub is_construction: bool, // Construction geometry (guides, not extruded)
}

/// Sketch entity types
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SketchEntity {
    Line {
        id: SketchEntityId,
        start: SketchPointId,
        end: SketchPointId,
    },
    Arc {
        id: SketchEntityId,
        center: SketchPointId,
        start: SketchPointId,
        end: SketchPointId,
        radius: f32,
        /// Arc direction in sketch coordinates (true = counter-clockwise from start→end).
        ccw: bool,
    },
    Circle {
        id: SketchEntityId,
        center: SketchPointId,
        radius: f32,
    },
    Point {
        id: SketchEntityId,
        point: SketchPointId,
    },
}

impl SketchEntity {
    pub fn id(&self) -> SketchEntityId {
        match self {
            Self::Line { id, .. } => *id,
            Self::Arc { id, .. } => *id,
            Self::Circle { id, .. } => *id,
            Self::Point { id, .. } => *id,
        }
    }
}

/// 2D parametric sketch
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sketch {
    pub plane: SketchPlane,
    pub points: Vec<SketchPoint>,
    pub entities: Vec<SketchEntity>,
    pub is_solved: bool,
}

impl Sketch {
    pub fn new(plane: SketchPlane) -> Self {
        Self {
            plane,
            points: Vec::new(),
            entities: Vec::new(),
            is_solved: false,
        }
    }

    /// Add a point to the sketch
    pub fn add_point(&mut self, position: Point2) -> SketchPointId {
        let id = SketchPointId(self.points.len() as u32);
        self.points.push(SketchPoint {
            id,
            position,
            is_construction: false,
        });
        id
    }

    /// Add an entity to the sketch
    pub fn add_entity(&mut self, entity: SketchEntity) -> SketchEntityId {
        let id = entity.id();
        self.entities.push(entity);
        id
    }

    /// Get point by ID
    pub fn point(&self, id: SketchPointId) -> Option<&SketchPoint> {
        self.points.get(id.0 as usize)
    }

    /// Get mutable point by ID
    pub fn point_mut(&mut self, id: SketchPointId) -> Option<&mut SketchPoint> {
        self.points.get_mut(id.0 as usize)
    }

    /// Get entity by ID
    pub fn entity(&self, id: SketchEntityId) -> Option<&SketchEntity> {
        self.entities.iter().find(|e| e.id() == id)
    }

    /// Transform 2D sketch point to 3D world space
    pub fn to_3d_point(&self, p: Point2) -> Point3 {
        match &self.plane {
            SketchPlane::XY => Point3::new(p.x, p.y, 0.0),
            SketchPlane::YZ => Point3::new(0.0, p.x, p.y),
            SketchPlane::XZ => Point3::new(p.x, 0.0, p.y),
            SketchPlane::Arbitrary(frame) => frame.to_3d(p),
        }
    }

    /// Transform 3D world point to 2D sketch space
    pub fn from_3d_point(&self, p: Point3) -> Point2 {
        match &self.plane {
            SketchPlane::XY => Point2::new(p.x, p.y),
            SketchPlane::YZ => Point2::new(p.y, p.z),
            SketchPlane::XZ => Point2::new(p.x, p.z),
            SketchPlane::Arbitrary(frame) => frame.from_3d(p),
        }
    }

    /// Get the normal vector for this sketch plane
    pub fn normal(&self) -> Vector3 {
        match &self.plane {
            SketchPlane::XY => Vector3::new(0.0, 0.0, 1.0),
            SketchPlane::YZ => Vector3::new(1.0, 0.0, 0.0),
            SketchPlane::XZ => Vector3::new(0.0, 1.0, 0.0),
            SketchPlane::Arbitrary(frame) => frame.normal,
        }
    }

    /// Get the origin point for this sketch plane
    pub fn origin(&self) -> Point3 {
        match &self.plane {
            SketchPlane::XY | SketchPlane::YZ | SketchPlane::XZ => Point3::new(0.0, 0.0, 0.0),
            SketchPlane::Arbitrary(frame) => frame.origin,
        }
    }

    /// Get all entities that reference a point
    pub fn entities_with_point(&self, point_id: SketchPointId) -> Vec<SketchEntityId> {
        self.entities
            .iter()
            .filter(|e| match e {
                SketchEntity::Line { start, end, .. } => *start == point_id || *end == point_id,
                SketchEntity::Arc {
                    center, start, end, ..
                } => *center == point_id || *start == point_id || *end == point_id,
                SketchEntity::Circle { center, .. } => *center == point_id,
                SketchEntity::Point { point, .. } => *point == point_id,
            })
            .map(|e| e.id())
            .collect()
    }
}

/// Compute circumcenter of three points (circle through 3 points).
///
/// Returns `None` if the points are collinear or numerically degenerate.
pub fn circumcenter(p1: Point2, p2: Point2, p3: Point2) -> Option<Point2> {
    // Based on standard determinant formula.
    let x1 = p1.x;
    let y1 = p1.y;
    let x2 = p2.x;
    let y2 = p2.y;
    let x3 = p3.x;
    let y3 = p3.y;

    let d = 2.0 * (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2));
    if d.abs() < 1e-8 {
        return None;
    }

    let x1sq_y1sq = x1 * x1 + y1 * y1;
    let x2sq_y2sq = x2 * x2 + y2 * y2;
    let x3sq_y3sq = x3 * x3 + y3 * y3;

    let cx = (x1sq_y1sq * (y2 - y3) + x2sq_y2sq * (y3 - y1) + x3sq_y3sq * (y1 - y2)) / d;
    let cy = (x1sq_y1sq * (x3 - x2) + x2sq_y2sq * (x1 - x3) + x3sq_y3sq * (x2 - x1)) / d;

    Some(Point2::new(cx, cy))
}

/// Signed area *2 of triangle (a,b,c). Positive => CCW turn.
pub fn orient2d(a: Point2, b: Point2, c: Point2) -> f32 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sketch_creation() {
        let sketch = Sketch::new(SketchPlane::XY);
        assert_eq!(sketch.plane, SketchPlane::XY);
        assert_eq!(sketch.points.len(), 0);
        assert_eq!(sketch.entities.len(), 0);
    }

    #[test]
    fn test_add_point() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(10.0, 20.0));
        let p2 = sketch.add_point(Point2::new(30.0, 40.0));

        assert_eq!(p1.0, 0);
        assert_eq!(p2.0, 1);
        assert_eq!(sketch.points.len(), 2);
    }

    #[test]
    fn test_coordinate_transform() {
        let sketch = Sketch::new(SketchPlane::XY);
        let p2 = Point2::new(5.0, 10.0);
        let p3 = sketch.to_3d_point(p2);

        assert_eq!(p3.x, 5.0);
        assert_eq!(p3.y, 10.0);
        assert_eq!(p3.z, 0.0);

        let back = sketch.from_3d_point(p3);
        assert_eq!(back.x, 5.0);
        assert_eq!(back.y, 10.0);
    }

    #[test]
    fn test_point_distance() {
        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(3.0, 4.0);
        assert!((p1.distance(&p2) - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_circumcenter_right_triangle() {
        // Triangle: (0,0), (2,0), (0,2) has circumcenter at (1,1).
        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(2.0, 0.0);
        let p3 = Point2::new(0.0, 2.0);
        let c = circumcenter(p1, p2, p3).unwrap();
        assert!((c.x - 1.0).abs() < 1e-5);
        assert!((c.y - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_circumcenter_collinear_none() {
        let p1 = Point2::new(0.0, 0.0);
        let p2 = Point2::new(1.0, 1.0);
        let p3 = Point2::new(2.0, 2.0);
        assert!(circumcenter(p1, p2, p3).is_none());
    }

    #[test]
    fn test_orient2d_sign() {
        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(1.0, 0.0);
        let c = Point2::new(1.0, 1.0);
        assert!(orient2d(a, b, c) > 0.0);
        assert!(orient2d(a, c, b) < 0.0);
    }

    #[test]
    fn test_coordinate_frame_from_origin_normal() {
        // Frame at origin with Z normal (like XY plane)
        let frame = SketchCoordinateFrame::from_origin_normal(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        )
        .unwrap();

        // U and V should be perpendicular to normal and each other
        let u_dot_n = frame.u_axis.x * frame.normal.x
            + frame.u_axis.y * frame.normal.y
            + frame.u_axis.z * frame.normal.z;
        let v_dot_n = frame.v_axis.x * frame.normal.x
            + frame.v_axis.y * frame.normal.y
            + frame.v_axis.z * frame.normal.z;
        let u_dot_v = frame.u_axis.x * frame.v_axis.x
            + frame.u_axis.y * frame.v_axis.y
            + frame.u_axis.z * frame.v_axis.z;

        assert!(u_dot_n.abs() < 1e-6);
        assert!(v_dot_n.abs() < 1e-6);
        assert!(u_dot_v.abs() < 1e-6);
    }

    #[test]
    fn test_coordinate_frame_roundtrip() {
        // Create a tilted plane
        let frame = SketchCoordinateFrame::from_origin_normal(
            Point3::new(10.0, 20.0, 30.0),
            Vector3::new(1.0, 1.0, 1.0), // Tilted normal
        )
        .unwrap();

        // Transform a 2D point to 3D and back
        let p2 = Point2::new(5.0, 7.0);
        let p3 = frame.to_3d(p2);
        let back = frame.from_3d(p3);

        assert!((back.x - p2.x).abs() < 1e-5);
        assert!((back.y - p2.y).abs() < 1e-5);
    }

    #[test]
    fn test_coordinate_frame_offset() {
        let frame = SketchCoordinateFrame::from_origin_normal(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        )
        .unwrap();

        let offset = frame.with_offset(10.0);

        assert!((offset.origin.z - 10.0).abs() < 1e-6);
        assert!(offset.normal == frame.normal);
        assert!(offset.u_axis == frame.u_axis);
        assert!(offset.v_axis == frame.v_axis);
    }

    #[test]
    fn test_arbitrary_sketch_plane() {
        let frame = SketchCoordinateFrame::from_origin_normal(
            Point3::new(5.0, 5.0, 5.0),
            Vector3::new(0.0, 1.0, 0.0), // Y normal (like XZ plane but shifted)
        )
        .unwrap();

        let sketch = Sketch::new(SketchPlane::Arbitrary(frame));

        // Point at origin of sketch plane
        let p3 = sketch.to_3d_point(Point2::new(0.0, 0.0));
        assert!((p3.x - 5.0).abs() < 1e-5);
        assert!((p3.y - 5.0).abs() < 1e-5);
        assert!((p3.z - 5.0).abs() < 1e-5);

        // Normal should be Y
        let n = sketch.normal();
        assert!((n.y - 1.0).abs() < 1e-5);
    }
}
