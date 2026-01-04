//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: sketch.rs | DNA/src/cad/sketch.rs
//! PURPOSE: 2D parametric sketch for CAD modeling
//! MODIFIED: 2026-01-04
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

use super::geometry::{Point3, Vector3};
use serde::{Serialize, Deserialize};

/// Sketch plane orientation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SketchPlane {
    XY,  // Normal: +Z
    YZ,  // Normal: +X
    XZ,  // Normal: +Y
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
    pub is_construction: bool,  // Construction geometry (guides, not extruded)
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
        match self.plane {
            SketchPlane::XY => Point3::new(p.x, p.y, 0.0),
            SketchPlane::YZ => Point3::new(0.0, p.x, p.y),
            SketchPlane::XZ => Point3::new(p.x, 0.0, p.y),
        }
    }

    /// Transform 3D world point to 2D sketch space
    pub fn from_3d_point(&self, p: Point3) -> Point2 {
        match self.plane {
            SketchPlane::XY => Point2::new(p.x, p.y),
            SketchPlane::YZ => Point2::new(p.y, p.z),
            SketchPlane::XZ => Point2::new(p.x, p.z),
        }
    }

    /// Get all entities that reference a point
    pub fn entities_with_point(&self, point_id: SketchPointId) -> Vec<SketchEntityId> {
        self.entities
            .iter()
            .filter(|e| match e {
                SketchEntity::Line { start, end, .. } => *start == point_id || *end == point_id,
                SketchEntity::Arc { center, start, end, .. } => {
                    *center == point_id || *start == point_id || *end == point_id
                }
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
}
