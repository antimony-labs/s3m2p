//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: geometry.rs | DNA/src/cad/geometry.rs
//! PURPOSE: Geometric primitives for CAD/B-Rep modeling
//! MODIFIED: 2025-12-09
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

//!
//! PURPOSE: Geometric primitives for CAD/B-Rep modeling
//!
//! LAYER: DNA → CAD
//!
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │ TYPES DEFINED                                                               │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │ Point3        3D point in space                                             │
//! │ Vector3       3D direction vector                                           │
//! │ Plane         Infinite plane (point + normal)                               │
//! │ Line          Infinite line (point + direction)                             │
//! │ Ray           Half-infinite ray (origin + direction)                        │
//! │ Segment       Line segment (two endpoints)                                  │
//! │ BoundingBox3  Axis-aligned bounding box                                     │
//! │ Transform3    Affine transformation matrix                                  │
//! └─────────────────────────────────────────────────────────────────────────────┘
//!
//! DEPENDS ON:
//!   • glam - Vector math
//!
//! USED BY:
//!   • DNA/src/cad/topology.rs → Geometric backing for topology
//!   • CORE/CAD_ENGINE         → Solid modeling
//!
//! ═══════════════════════════════════════════════════════════════════════════════

use glam::{Mat4, Vec3};
use serde::{Serialize, Deserialize};

/// Tolerance for geometric comparisons (1 micrometer)
pub const TOLERANCE: f32 = 1e-6;

/// 3D point in space
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    pub const ORIGIN: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn from_vec3(v: Vec3) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }

    #[inline]
    pub fn to_vec3(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    /// Distance to another point
    #[inline]
    pub fn distance(self, other: Point3) -> f32 {
        (self.to_vec3() - other.to_vec3()).length()
    }

    /// Squared distance (faster, no sqrt)
    #[inline]
    pub fn distance_squared(self, other: Point3) -> f32 {
        (self.to_vec3() - other.to_vec3()).length_squared()
    }

    /// Check if two points are approximately equal
    #[inline]
    pub fn approx_eq(self, other: Point3, tolerance: f32) -> bool {
        self.distance_squared(other) < tolerance * tolerance
    }

    /// Linear interpolation between two points
    #[inline]
    pub fn lerp(self, other: Point3, t: f32) -> Point3 {
        Point3::from_vec3(self.to_vec3().lerp(other.to_vec3(), t))
    }

    /// Midpoint between two points
    #[inline]
    pub fn midpoint(self, other: Point3) -> Point3 {
        self.lerp(other, 0.5)
    }

    /// Transform point by matrix
    #[inline]
    pub fn transform(self, matrix: &Transform3) -> Point3 {
        let v = matrix.0.transform_point3(self.to_vec3());
        Point3::from_vec3(v)
    }
}

impl From<Vec3> for Point3 {
    fn from(v: Vec3) -> Self {
        Self::from_vec3(v)
    }
}

impl From<Point3> for Vec3 {
    fn from(p: Point3) -> Self {
        p.to_vec3()
    }
}

impl std::ops::Add<Vector3> for Point3 {
    type Output = Point3;
    fn add(self, v: Vector3) -> Point3 {
        Point3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl std::ops::Sub for Point3 {
    type Output = Vector3;
    fn sub(self, other: Point3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

/// 3D direction vector
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const X: Self = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const Y: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    pub const Z: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };
    pub const NEG_X: Self = Self {
        x: -1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const NEG_Y: Self = Self {
        x: 0.0,
        y: -1.0,
        z: 0.0,
    };
    pub const NEG_Z: Self = Self {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };

    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn from_vec3(v: Vec3) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }

    #[inline]
    pub fn to_vec3(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    #[inline]
    pub fn length(self) -> f32 {
        self.to_vec3().length()
    }

    #[inline]
    pub fn length_squared(self) -> f32 {
        self.to_vec3().length_squared()
    }

    /// Normalize to unit vector (returns None if zero-length)
    pub fn normalize(self) -> Option<Self> {
        let len = self.length();
        if len < TOLERANCE {
            None
        } else {
            Some(Self::new(self.x / len, self.y / len, self.z / len))
        }
    }

    /// Normalize, defaulting to Z if zero-length
    #[inline]
    pub fn normalize_or_z(self) -> Self {
        self.normalize().unwrap_or(Self::Z)
    }

    #[inline]
    pub fn dot(self, other: Vector3) -> f32 {
        self.to_vec3().dot(other.to_vec3())
    }

    #[inline]
    pub fn cross(self, other: Vector3) -> Vector3 {
        Vector3::from_vec3(self.to_vec3().cross(other.to_vec3()))
    }

    /// Angle between two vectors in radians
    pub fn angle(self, other: Vector3) -> f32 {
        let dot = self.dot(other);
        let len_product = self.length() * other.length();
        if len_product < TOLERANCE {
            0.0
        } else {
            (dot / len_product).clamp(-1.0, 1.0).acos()
        }
    }

    /// Project this vector onto another
    pub fn project_onto(self, other: Vector3) -> Vector3 {
        let other_len_sq = other.length_squared();
        if other_len_sq < TOLERANCE {
            Vector3::ZERO
        } else {
            other * (self.dot(other) / other_len_sq)
        }
    }

    /// Transform vector by matrix (direction only, no translation)
    #[inline]
    pub fn transform(self, matrix: &Transform3) -> Vector3 {
        let v = matrix.0.transform_vector3(self.to_vec3());
        Vector3::from_vec3(v)
    }
}

impl From<Vec3> for Vector3 {
    fn from(v: Vec3) -> Self {
        Self::from_vec3(v)
    }
}

impl std::ops::Add for Vector3 {
    type Output = Vector3;
    fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, scalar: f32) -> Vector3 {
        Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

/// Infinite plane defined by point and normal
#[derive(Clone, Copy, Debug)]
pub struct Plane {
    pub origin: Point3,
    pub normal: Vector3,
}

impl Plane {
    pub const XY: Self = Self {
        origin: Point3::ORIGIN,
        normal: Vector3::Z,
    };
    pub const XZ: Self = Self {
        origin: Point3::ORIGIN,
        normal: Vector3::Y,
    };
    pub const YZ: Self = Self {
        origin: Point3::ORIGIN,
        normal: Vector3::X,
    };

    pub fn new(origin: Point3, normal: Vector3) -> Option<Self> {
        let normal = normal.normalize()?;
        Some(Self { origin, normal })
    }

    /// Create plane from three non-collinear points
    pub fn from_points(p1: Point3, p2: Point3, p3: Point3) -> Option<Self> {
        let v1 = p2 - p1;
        let v2 = p3 - p1;
        let normal = v1.cross(v2).normalize()?;
        Some(Self { origin: p1, normal })
    }

    /// Signed distance from point to plane (positive = same side as normal)
    pub fn signed_distance(&self, point: Point3) -> f32 {
        (point - self.origin).dot(self.normal)
    }

    /// Project point onto plane
    pub fn project_point(&self, point: Point3) -> Point3 {
        let dist = self.signed_distance(point);
        point + self.normal * (-dist)
    }
}

/// Infinite line defined by point and direction
#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Line {
    pub fn new(origin: Point3, direction: Vector3) -> Option<Self> {
        let direction = direction.normalize()?;
        Some(Self { origin, direction })
    }

    /// Create line through two points
    pub fn from_points(p1: Point3, p2: Point3) -> Option<Self> {
        Self::new(p1, p2 - p1)
    }

    /// Point at parameter t along line
    #[inline]
    pub fn point_at(&self, t: f32) -> Point3 {
        self.origin + self.direction * t
    }

    /// Project point onto line, returning parameter t
    pub fn project_param(&self, point: Point3) -> f32 {
        (point - self.origin).dot(self.direction)
    }

    /// Project point onto line
    pub fn project_point(&self, point: Point3) -> Point3 {
        self.point_at(self.project_param(point))
    }

    /// Distance from point to line
    pub fn distance_to_point(&self, point: Point3) -> f32 {
        let projected = self.project_point(point);
        point.distance(projected)
    }
}

/// Half-infinite ray (origin + direction)
#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Option<Self> {
        let direction = direction.normalize()?;
        Some(Self { origin, direction })
    }

    #[inline]
    pub fn point_at(&self, t: f32) -> Point3 {
        self.origin + self.direction * t
    }

    /// Intersect ray with plane, returning parameter t (None if parallel)
    pub fn intersect_plane(&self, plane: &Plane) -> Option<f32> {
        let denom = self.direction.dot(plane.normal);
        if denom.abs() < TOLERANCE {
            return None; // Parallel
        }
        let t = (plane.origin - self.origin).dot(plane.normal) / denom;
        if t >= 0.0 {
            Some(t)
        } else {
            None
        }
    }
}

/// Line segment (two endpoints)
#[derive(Clone, Copy, Debug)]
pub struct Segment {
    pub start: Point3,
    pub end: Point3,
}

impl Segment {
    pub fn new(start: Point3, end: Point3) -> Self {
        Self { start, end }
    }

    #[inline]
    pub fn length(&self) -> f32 {
        self.start.distance(self.end)
    }

    #[inline]
    pub fn midpoint(&self) -> Point3 {
        self.start.midpoint(self.end)
    }

    #[inline]
    pub fn direction(&self) -> Vector3 {
        self.end - self.start
    }

    /// Point at parameter t (0 = start, 1 = end)
    #[inline]
    pub fn point_at(&self, t: f32) -> Point3 {
        self.start.lerp(self.end, t)
    }

    /// Convert to infinite line
    pub fn to_line(&self) -> Option<Line> {
        Line::from_points(self.start, self.end)
    }
}

/// Axis-aligned bounding box in 3D
#[derive(Clone, Copy, Debug)]
pub struct BoundingBox3 {
    pub min: Point3,
    pub max: Point3,
}

impl BoundingBox3 {
    pub const EMPTY: Self = Self {
        min: Point3 {
            x: f32::INFINITY,
            y: f32::INFINITY,
            z: f32::INFINITY,
        },
        max: Point3 {
            x: f32::NEG_INFINITY,
            y: f32::NEG_INFINITY,
            z: f32::NEG_INFINITY,
        },
    };

    pub fn new(min: Point3, max: Point3) -> Self {
        Self {
            min: Point3::new(min.x.min(max.x), min.y.min(max.y), min.z.min(max.z)),
            max: Point3::new(min.x.max(max.x), min.y.max(max.y), min.z.max(max.z)),
        }
    }

    pub fn from_points(points: &[Point3]) -> Self {
        let mut bbox = Self::EMPTY;
        for &p in points {
            bbox = bbox.expand_by_point(p);
        }
        bbox
    }

    #[inline]
    pub fn center(&self) -> Point3 {
        self.min.midpoint(self.max)
    }

    #[inline]
    pub fn size(&self) -> Vector3 {
        self.max - self.min
    }

    #[inline]
    pub fn diagonal(&self) -> f32 {
        self.size().length()
    }

    pub fn expand_by_point(self, point: Point3) -> Self {
        Self {
            min: Point3::new(
                self.min.x.min(point.x),
                self.min.y.min(point.y),
                self.min.z.min(point.z),
            ),
            max: Point3::new(
                self.max.x.max(point.x),
                self.max.y.max(point.y),
                self.max.z.max(point.z),
            ),
        }
    }

    pub fn union(self, other: Self) -> Self {
        Self::new(
            Point3::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            Point3::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        )
    }

    pub fn contains(&self, point: Point3) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
            && point.z >= self.min.z
            && point.z <= self.max.z
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    /// Expand box by margin on all sides
    pub fn expand(&self, margin: f32) -> Self {
        Self {
            min: Point3::new(
                self.min.x - margin,
                self.min.y - margin,
                self.min.z - margin,
            ),
            max: Point3::new(
                self.max.x + margin,
                self.max.y + margin,
                self.max.z + margin,
            ),
        }
    }
}

/// Affine transformation matrix (4x4)
#[derive(Clone, Copy, Debug)]
pub struct Transform3(pub Mat4);

impl Transform3 {
    pub const IDENTITY: Self = Self(Mat4::IDENTITY);

    #[inline]
    pub fn from_translation(v: Vector3) -> Self {
        Self(Mat4::from_translation(v.to_vec3()))
    }

    #[inline]
    pub fn from_scale(v: Vector3) -> Self {
        Self(Mat4::from_scale(v.to_vec3()))
    }

    #[inline]
    pub fn from_rotation_x(angle: f32) -> Self {
        Self(Mat4::from_rotation_x(angle))
    }

    #[inline]
    pub fn from_rotation_y(angle: f32) -> Self {
        Self(Mat4::from_rotation_y(angle))
    }

    #[inline]
    pub fn from_rotation_z(angle: f32) -> Self {
        Self(Mat4::from_rotation_z(angle))
    }

    #[inline]
    pub fn from_axis_angle(axis: Vector3, angle: f32) -> Self {
        Self(Mat4::from_axis_angle(axis.to_vec3(), angle))
    }

    /// Combine transformations (self applied first, then other)
    #[inline]
    pub fn then(self, other: Self) -> Self {
        Self(other.0 * self.0)
    }

    /// Inverse transformation
    pub fn inverse(self) -> Self {
        Self(self.0.inverse())
    }
}

impl Default for Transform3 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_distance() {
        let p1 = Point3::new(0.0, 0.0, 0.0);
        let p2 = Point3::new(3.0, 4.0, 0.0);
        assert!((p1.distance(p2) - 5.0).abs() < TOLERANCE);
    }

    #[test]
    fn test_vector_normalize() {
        let v = Vector3::new(3.0, 4.0, 0.0);
        let n = v.normalize().unwrap();
        assert!((n.length() - 1.0).abs() < TOLERANCE);
    }

    #[test]
    fn test_vector_normalize_zero() {
        let v = Vector3::ZERO;
        assert!(v.normalize().is_none());
    }

    #[test]
    fn test_vector_normalize_near_zero() {
        let v = Vector3::new(1e-8, 0.0, 0.0);
        assert!(v.normalize().is_none());
    }

    #[test]
    fn test_vector_cross() {
        let v1 = Vector3::X;
        let v2 = Vector3::Y;
        let cross = v1.cross(v2);
        assert!((cross.x - Vector3::Z.x).abs() < TOLERANCE);
        assert!((cross.y - Vector3::Z.y).abs() < TOLERANCE);
        assert!((cross.z - Vector3::Z.z).abs() < TOLERANCE);
    }

    #[test]
    fn test_plane_distance() {
        let plane = Plane::XY;
        let point = Point3::new(1.0, 2.0, 5.0);
        assert!((plane.signed_distance(point) - 5.0).abs() < TOLERANCE);
    }

    #[test]
    fn test_plane_from_collinear_points() {
        let p1 = Point3::new(0.0, 0.0, 0.0);
        let p2 = Point3::new(1.0, 1.0, 1.0);
        let p3 = Point3::new(2.0, 2.0, 2.0);
        assert!(Plane::from_points(p1, p2, p3).is_none());
    }

    #[test]
    fn test_ray_plane_intersection() {
        let ray = Ray::new(Point3::new(0.0, 0.0, 5.0), Vector3::NEG_Z).unwrap();
        let plane = Plane::XY;
        let t = ray.intersect_plane(&plane).unwrap();
        assert!((t - 5.0).abs() < TOLERANCE);
    }

    #[test]
    fn test_ray_parallel_to_plane() {
        let ray = Ray::new(Point3::new(0.0, 0.0, 5.0), Vector3::X).unwrap();
        let plane = Plane::XY;
        assert!(ray.intersect_plane(&plane).is_none());
    }

    #[test]
    fn test_ray_behind_origin() {
        let ray = Ray::new(Point3::new(0.0, 0.0, 5.0), Vector3::Z).unwrap();
        let plane = Plane::XY;
        assert!(ray.intersect_plane(&plane).is_none());
    }

    #[test]
    fn test_bounding_box() {
        let points = vec![
            Point3::new(1.0, 2.0, 3.0),
            Point3::new(-1.0, -2.0, -3.0),
            Point3::new(0.0, 0.0, 0.0),
        ];
        let bbox = BoundingBox3::from_points(&points);
        assert!(bbox.contains(Point3::ORIGIN));
        assert!(!bbox.contains(Point3::new(10.0, 0.0, 0.0)));
    }

    #[test]
    fn test_bounding_box_empty_diagonal() {
        let bbox = BoundingBox3::EMPTY;
        assert!(bbox.diagonal().is_infinite());
    }

    #[test]
    fn test_transform() {
        let p = Point3::new(1.0, 0.0, 0.0);
        let t = Transform3::from_translation(Vector3::new(0.0, 5.0, 0.0));
        let transformed = p.transform(&t);
        assert!((transformed.y - 5.0).abs() < TOLERANCE);
    }

    #[test]
    fn test_transform_inverse() {
        let t = Transform3::from_rotation_z(0.5)
            .then(Transform3::from_translation(Vector3::new(10.0, 20.0, 30.0)));
        let t_inv = t.inverse();

        let p = Point3::new(1.0, 2.0, 3.0);
        let transformed = p.transform(&t);
        let back = transformed.transform(&t_inv);
        assert!(p.approx_eq(back, 1e-4));
    }

    #[test]
    fn test_lerp_midpoint() {
        let p1 = Point3::new(0.0, 0.0, 0.0);
        let p2 = Point3::new(10.0, 20.0, 30.0);
        let mid = p1.lerp(p2, 0.5);
        assert!((mid.x - 5.0).abs() < TOLERANCE);
        assert!((mid.y - 10.0).abs() < TOLERANCE);
        assert!((mid.z - 15.0).abs() < TOLERANCE);
    }

    #[test]
    fn test_vector_project_onto() {
        let v = Vector3::new(3.0, 4.0, 0.0);
        let onto = Vector3::X;
        let proj = v.project_onto(onto);
        assert!((proj.x - 3.0).abs() < TOLERANCE);
        assert!(proj.y.abs() < TOLERANCE);
        assert!(proj.z.abs() < TOLERANCE);
    }

    #[test]
    fn test_vector_angle() {
        let v1 = Vector3::X;
        let v2 = Vector3::Y;
        let angle = v1.angle(v2);
        assert!((angle - std::f32::consts::FRAC_PI_2).abs() < TOLERANCE);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PROPERTY-BASED TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Vector normalization produces unit length (when normalizable)
        #[test]
        fn prop_normalize_produces_unit_vector(
            x in -1000.0f32..1000.0,
            y in -1000.0f32..1000.0,
            z in -1000.0f32..1000.0
        ) {
            let v = Vector3::new(x, y, z);
            if let Some(n) = v.normalize() {
                prop_assert!((n.length() - 1.0).abs() < 1e-5,
                    "Normalized vector length should be 1.0, got {}", n.length());
            }
        }

        /// Point distance is symmetric: d(a,b) == d(b,a)
        #[test]
        fn prop_distance_symmetric(
            x1 in -1000.0f32..1000.0, y1 in -1000.0f32..1000.0, z1 in -1000.0f32..1000.0,
            x2 in -1000.0f32..1000.0, y2 in -1000.0f32..1000.0, z2 in -1000.0f32..1000.0
        ) {
            let p1 = Point3::new(x1, y1, z1);
            let p2 = Point3::new(x2, y2, z2);
            let d1 = p1.distance(p2);
            let d2 = p2.distance(p1);
            prop_assert!((d1 - d2).abs() < TOLERANCE,
                "Distance should be symmetric: {} vs {}", d1, d2);
        }

        /// Distance is non-negative
        #[test]
        fn prop_distance_non_negative(
            x1 in -1000.0f32..1000.0, y1 in -1000.0f32..1000.0, z1 in -1000.0f32..1000.0,
            x2 in -1000.0f32..1000.0, y2 in -1000.0f32..1000.0, z2 in -1000.0f32..1000.0
        ) {
            let p1 = Point3::new(x1, y1, z1);
            let p2 = Point3::new(x2, y2, z2);
            prop_assert!(p1.distance(p2) >= 0.0);
        }

        /// Dot product is commutative: a·b == b·a
        #[test]
        fn prop_dot_commutative(
            x1 in -100.0f32..100.0, y1 in -100.0f32..100.0, z1 in -100.0f32..100.0,
            x2 in -100.0f32..100.0, y2 in -100.0f32..100.0, z2 in -100.0f32..100.0
        ) {
            let v1 = Vector3::new(x1, y1, z1);
            let v2 = Vector3::new(x2, y2, z2);
            prop_assert!((v1.dot(v2) - v2.dot(v1)).abs() < TOLERANCE,
                "Dot product should be commutative");
        }

        /// Cross product is anti-commutative: a×b == -(b×a)
        #[test]
        fn prop_cross_anticommutative(
            x1 in -100.0f32..100.0, y1 in -100.0f32..100.0, z1 in -100.0f32..100.0,
            x2 in -100.0f32..100.0, y2 in -100.0f32..100.0, z2 in -100.0f32..100.0
        ) {
            let v1 = Vector3::new(x1, y1, z1);
            let v2 = Vector3::new(x2, y2, z2);
            let c1 = v1.cross(v2);
            let c2 = v2.cross(v1);
            prop_assert!((c1.x + c2.x).abs() < TOLERANCE);
            prop_assert!((c1.y + c2.y).abs() < TOLERANCE);
            prop_assert!((c1.z + c2.z).abs() < TOLERANCE);
        }

        /// Cross product is perpendicular to both inputs
        #[test]
        fn prop_cross_perpendicular(
            x1 in -100.0f32..100.0, y1 in -100.0f32..100.0, z1 in -100.0f32..100.0,
            x2 in -100.0f32..100.0, y2 in -100.0f32..100.0, z2 in -100.0f32..100.0
        ) {
            let v1 = Vector3::new(x1, y1, z1);
            let v2 = Vector3::new(x2, y2, z2);
            let cross = v1.cross(v2);

            // Cross product should be perpendicular to both (within tolerance)
            // For small cross products, the dot product tolerance scales with magnitude
            let tol = (cross.length() + 1.0) * 1e-4;
            prop_assert!(cross.dot(v1).abs() < tol,
                "Cross product should be perpendicular to v1");
            prop_assert!(cross.dot(v2).abs() < tol,
                "Cross product should be perpendicular to v2");
        }

        /// Lerp at t=0 returns first point
        #[test]
        fn prop_lerp_at_zero(
            x1 in -1000.0f32..1000.0, y1 in -1000.0f32..1000.0, z1 in -1000.0f32..1000.0,
            x2 in -1000.0f32..1000.0, y2 in -1000.0f32..1000.0, z2 in -1000.0f32..1000.0
        ) {
            let p1 = Point3::new(x1, y1, z1);
            let p2 = Point3::new(x2, y2, z2);
            let result = p1.lerp(p2, 0.0);
            prop_assert!(result.approx_eq(p1, TOLERANCE));
        }

        /// Lerp at t=1 returns second point
        #[test]
        fn prop_lerp_at_one(
            x1 in -1000.0f32..1000.0, y1 in -1000.0f32..1000.0, z1 in -1000.0f32..1000.0,
            x2 in -1000.0f32..1000.0, y2 in -1000.0f32..1000.0, z2 in -1000.0f32..1000.0
        ) {
            let p1 = Point3::new(x1, y1, z1);
            let p2 = Point3::new(x2, y2, z2);
            let result = p1.lerp(p2, 1.0);
            prop_assert!(result.approx_eq(p2, TOLERANCE));
        }

        /// Lerp at t=0.5 produces midpoint
        #[test]
        fn prop_lerp_midpoint(
            x1 in -1000.0f32..1000.0, y1 in -1000.0f32..1000.0, z1 in -1000.0f32..1000.0,
            x2 in -1000.0f32..1000.0, y2 in -1000.0f32..1000.0, z2 in -1000.0f32..1000.0
        ) {
            let p1 = Point3::new(x1, y1, z1);
            let p2 = Point3::new(x2, y2, z2);
            let mid = p1.lerp(p2, 0.5);
            let expected = Point3::new((x1 + x2) / 2.0, (y1 + y2) / 2.0, (z1 + z2) / 2.0);
            prop_assert!(mid.approx_eq(expected, 1e-4));
        }

        /// BoundingBox contains all input points
        #[test]
        fn prop_bbox_contains_inputs(
            points in prop::collection::vec(
                (-100.0f32..100.0, -100.0f32..100.0, -100.0f32..100.0),
                1..20
            )
        ) {
            let pts: Vec<Point3> = points.iter()
                .map(|(x, y, z)| Point3::new(*x, *y, *z))
                .collect();
            let bbox = BoundingBox3::from_points(&pts);

            for p in &pts {
                prop_assert!(bbox.contains(*p),
                    "BoundingBox should contain input point {:?}", p);
            }
        }

        /// Plane projection point lies on plane (distance = 0)
        #[test]
        fn prop_plane_projection_on_plane(
            px in -100.0f32..100.0, py in -100.0f32..100.0, pz in -100.0f32..100.0,
            nx in -1.0f32..1.0, ny in -1.0f32..1.0, nz in -1.0f32..1.0,
            qx in -100.0f32..100.0, qy in -100.0f32..100.0, qz in -100.0f32..100.0
        ) {
            let normal = Vector3::new(nx, ny, nz);
            if normal.length() < TOLERANCE {
                return Ok(());
            }

            if let Some(plane) = Plane::new(Point3::new(px, py, pz), normal) {
                let point = Point3::new(qx, qy, qz);
                let projected = plane.project_point(point);
                let dist = plane.signed_distance(projected).abs();
                prop_assert!(dist < 1e-4,
                    "Projected point should be on plane, distance = {}", dist);
            }
        }

        /// Transform then inverse transform returns original point
        #[test]
        fn prop_transform_inverse_roundtrip(
            px in -100.0f32..100.0, py in -100.0f32..100.0, pz in -100.0f32..100.0,
            tx in -50.0f32..50.0, ty in -50.0f32..50.0, tz in -50.0f32..50.0,
            angle in 0.0f32..6.28
        ) {
            let p = Point3::new(px, py, pz);
            let transform = Transform3::from_translation(Vector3::new(tx, ty, tz))
                .then(Transform3::from_rotation_z(angle));

            let transformed = p.transform(&transform);
            let back = transformed.transform(&transform.inverse());

            prop_assert!(p.approx_eq(back, 1e-4),
                "Transform roundtrip failed: {:?} -> {:?}", p, back);
        }

        /// All geometry operations produce finite results (no NaN/Inf)
        #[test]
        fn prop_operations_finite(
            x in -1000.0f32..1000.0, y in -1000.0f32..1000.0, z in -1000.0f32..1000.0
        ) {
            let p = Point3::new(x, y, z);
            let v = Vector3::new(x, y, z);

            // All basic operations should produce finite results
            prop_assert!(p.x.is_finite() && p.y.is_finite() && p.z.is_finite());
            prop_assert!(v.length().is_finite());
            prop_assert!(v.length_squared().is_finite());
            prop_assert!(v.dot(v).is_finite());

            let cross = v.cross(Vector3::X);
            prop_assert!(cross.x.is_finite() && cross.y.is_finite() && cross.z.is_finite());
        }
    }
}
