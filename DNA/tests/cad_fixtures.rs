//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: cad_fixtures.rs | DNA/tests/cad_fixtures.rs
//! PURPOSE: Test fixtures, builders, and assertions for CAD testing
//! MODIFIED: 2026-01-08
//! LAYER: Test utilities
//! ═══════════════════════════════════════════════════════════════════════════════

use dna::cad::{
    make_box, make_cylinder, make_sphere, Constraint, DimensionalConstraint, GeometricConstraint,
    Point2, Point3, Sketch, SketchEntity, SketchEntityId, SketchPlane, SketchPointId, Solid,
    Vector3, TOLERANCE,
};

// ═══════════════════════════════════════════════════════════════════════════════
// SKETCH FIXTURES
// ═══════════════════════════════════════════════════════════════════════════════

/// Builder for creating test sketches with common configurations
pub struct SketchFixture {
    pub sketch: Sketch,
    pub constraints: Vec<Constraint>,
}

impl SketchFixture {
    /// Empty sketch on XY plane
    pub fn empty_xy() -> Self {
        Self {
            sketch: Sketch::new(SketchPlane::XY),
            constraints: Vec::new(),
        }
    }

    /// Empty sketch on specified plane
    pub fn empty(plane: SketchPlane) -> Self {
        Self {
            sketch: Sketch::new(plane),
            constraints: Vec::new(),
        }
    }

    /// Simple line from (0,0) to (length, 0)
    pub fn horizontal_line(length: f32) -> Self {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p0 = sketch.add_point(Point2::new(0.0, 0.0));
        let p1 = sketch.add_point(Point2::new(length, 0.0));
        sketch.add_entity(SketchEntity::Line {
            id: SketchEntityId(0),
            start: p0,
            end: p1,
        });
        Self {
            sketch,
            constraints: Vec::new(),
        }
    }

    /// Simple line from (0,0) to (0, length)
    pub fn vertical_line(length: f32) -> Self {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p0 = sketch.add_point(Point2::new(0.0, 0.0));
        let p1 = sketch.add_point(Point2::new(0.0, length));
        sketch.add_entity(SketchEntity::Line {
            id: SketchEntityId(0),
            start: p0,
            end: p1,
        });
        Self {
            sketch,
            constraints: Vec::new(),
        }
    }

    /// Rectangle with 4 corners and 4 lines
    /// Returns fixture with points at corners: p0=(0,0), p1=(w,0), p2=(w,h), p3=(0,h)
    pub fn rectangle(width: f32, height: f32) -> Self {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p0 = sketch.add_point(Point2::new(0.0, 0.0));
        let p1 = sketch.add_point(Point2::new(width, 0.0));
        let p2 = sketch.add_point(Point2::new(width, height));
        let p3 = sketch.add_point(Point2::new(0.0, height));

        // Bottom edge
        sketch.add_entity(SketchEntity::Line {
            id: SketchEntityId(0),
            start: p0,
            end: p1,
        });
        // Right edge
        sketch.add_entity(SketchEntity::Line {
            id: SketchEntityId(1),
            start: p1,
            end: p2,
        });
        // Top edge
        sketch.add_entity(SketchEntity::Line {
            id: SketchEntityId(2),
            start: p2,
            end: p3,
        });
        // Left edge
        sketch.add_entity(SketchEntity::Line {
            id: SketchEntityId(3),
            start: p3,
            end: p0,
        });

        Self {
            sketch,
            constraints: Vec::new(),
        }
    }

    /// Triangle with 3 points and 3 lines
    pub fn triangle(base: f32, height: f32) -> Self {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p0 = sketch.add_point(Point2::new(0.0, 0.0));
        let p1 = sketch.add_point(Point2::new(base, 0.0));
        let p2 = sketch.add_point(Point2::new(base / 2.0, height));

        sketch.add_entity(SketchEntity::Line {
            id: SketchEntityId(0),
            start: p0,
            end: p1,
        });
        sketch.add_entity(SketchEntity::Line {
            id: SketchEntityId(1),
            start: p1,
            end: p2,
        });
        sketch.add_entity(SketchEntity::Line {
            id: SketchEntityId(2),
            start: p2,
            end: p0,
        });

        Self {
            sketch,
            constraints: Vec::new(),
        }
    }

    /// Circle centered at origin
    pub fn circle(radius: f32) -> Self {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let center = sketch.add_point(Point2::new(0.0, 0.0));
        sketch.add_entity(SketchEntity::Circle {
            id: SketchEntityId(0),
            center,
            radius,
        });
        Self {
            sketch,
            constraints: Vec::new(),
        }
    }

    /// Circle at specified center
    pub fn circle_at(center_x: f32, center_y: f32, radius: f32) -> Self {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let center = sketch.add_point(Point2::new(center_x, center_y));
        sketch.add_entity(SketchEntity::Circle {
            id: SketchEntityId(0),
            center,
            radius,
        });
        Self {
            sketch,
            constraints: Vec::new(),
        }
    }

    /// Two separate points (useful for coincident constraint tests)
    pub fn two_points(p1: Point2, p2: Point2) -> Self {
        let mut sketch = Sketch::new(SketchPlane::XY);
        sketch.add_point(p1);
        sketch.add_point(p2);
        Self {
            sketch,
            constraints: Vec::new(),
        }
    }

    /// Two parallel lines (useful for parallel/perpendicular tests)
    pub fn two_lines(
        line1_start: Point2,
        line1_end: Point2,
        line2_start: Point2,
        line2_end: Point2,
    ) -> Self {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p0 = sketch.add_point(line1_start);
        let p1 = sketch.add_point(line1_end);
        let p2 = sketch.add_point(line2_start);
        let p3 = sketch.add_point(line2_end);

        sketch.add_entity(SketchEntity::Line {
            id: SketchEntityId(0),
            start: p0,
            end: p1,
        });
        sketch.add_entity(SketchEntity::Line {
            id: SketchEntityId(1),
            start: p2,
            end: p3,
        });

        Self {
            sketch,
            constraints: Vec::new(),
        }
    }

    /// Add a geometric constraint
    pub fn with_geometric(mut self, constraint: GeometricConstraint) -> Self {
        self.constraints.push(Constraint::Geometric(constraint));
        self
    }

    /// Add a dimensional constraint
    pub fn with_dimensional(mut self, constraint: DimensionalConstraint) -> Self {
        self.constraints.push(Constraint::Dimensional(constraint));
        self
    }

    /// Add horizontal constraint to first line entity
    pub fn with_horizontal(mut self) -> Self {
        self.constraints
            .push(Constraint::Geometric(GeometricConstraint::Horizontal {
                line: SketchEntityId(0),
            }));
        self
    }

    /// Add vertical constraint to first line entity
    pub fn with_vertical(mut self) -> Self {
        self.constraints
            .push(Constraint::Geometric(GeometricConstraint::Vertical {
                line: SketchEntityId(0),
            }));
        self
    }

    /// Add distance constraint between first two points
    pub fn with_distance(mut self, value: f32) -> Self {
        self.constraints
            .push(Constraint::Dimensional(DimensionalConstraint::Distance {
                p1: SketchPointId(0),
                p2: SketchPointId(1),
                value,
            }));
        self
    }

    /// Consume and return components
    pub fn build(self) -> (Sketch, Vec<Constraint>) {
        (self.sketch, self.constraints)
    }

    /// Get mutable reference to sketch for further modifications
    pub fn sketch_mut(&mut self) -> &mut Sketch {
        &mut self.sketch
    }

    /// Get reference to constraints
    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SOLID FIXTURES
// ═══════════════════════════════════════════════════════════════════════════════

/// Factory for creating test solids
pub struct SolidFixture;

impl SolidFixture {
    /// Unit cube at origin (1x1x1)
    pub fn unit_cube() -> Solid {
        make_box(1.0, 1.0, 1.0)
    }

    /// Standard test box (100x50x25)
    pub fn test_box() -> Solid {
        make_box(100.0, 50.0, 25.0)
    }

    /// Small box for boolean tests (10x10x10)
    pub fn small_box() -> Solid {
        make_box(10.0, 10.0, 10.0)
    }

    /// Test cylinder (radius=10, height=50)
    pub fn test_cylinder() -> Solid {
        make_cylinder(10.0, 50.0, 16)
    }

    /// Test sphere (radius=25)
    pub fn test_sphere() -> Solid {
        make_sphere(25.0, 16, 8)
    }

    /// Two overlapping boxes for boolean tests
    /// Box A: 100x100x100 at origin
    /// Box B: 50x50x50 at (50,50,50) - overlaps corner of A
    pub fn overlapping_boxes() -> (Solid, Solid) {
        let a = make_box(100.0, 100.0, 100.0);
        // Note: make_box creates at origin, would need translation
        // For now, just create two boxes
        let b = make_box(50.0, 50.0, 50.0);
        (a, b)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// CUSTOM ASSERTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Assert two Point3 values are approximately equal
pub fn assert_point3_eq(actual: Point3, expected: Point3) {
    assert!(
        actual.approx_eq(expected, TOLERANCE),
        "Point3 mismatch:\n  actual:   ({:.6}, {:.6}, {:.6})\n  expected: ({:.6}, {:.6}, {:.6})",
        actual.x,
        actual.y,
        actual.z,
        expected.x,
        expected.y,
        expected.z
    );
}

/// Assert two Point3 values are approximately equal within custom tolerance
pub fn assert_point3_near(actual: Point3, expected: Point3, tolerance: f32) {
    assert!(
        actual.approx_eq(expected, tolerance),
        "Point3 mismatch (tol={}):\n  actual:   ({:.6}, {:.6}, {:.6})\n  expected: ({:.6}, {:.6}, {:.6})",
        tolerance,
        actual.x, actual.y, actual.z,
        expected.x, expected.y, expected.z
    );
}

/// Assert two Point2 values are approximately equal
pub fn assert_point2_eq(actual: Point2, expected: Point2) {
    let dist = actual.distance(&expected);
    assert!(
        dist < TOLERANCE,
        "Point2 mismatch:\n  actual:   ({:.6}, {:.6})\n  expected: ({:.6}, {:.6})\n  distance: {:.6}",
        actual.x, actual.y,
        expected.x, expected.y,
        dist
    );
}

/// Assert two Point2 values are approximately equal within custom tolerance
pub fn assert_point2_near(actual: Point2, expected: Point2, tolerance: f32) {
    let dist = actual.distance(&expected);
    assert!(
        dist < tolerance,
        "Point2 mismatch (tol={}):\n  actual:   ({:.6}, {:.6})\n  expected: ({:.6}, {:.6})\n  distance: {:.6}",
        tolerance,
        actual.x, actual.y,
        expected.x, expected.y,
        dist
    );
}

/// Assert two floats are approximately equal
pub fn assert_float_eq(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < TOLERANCE,
        "Float mismatch:\n  actual:   {:.6}\n  expected: {:.6}\n  diff:     {:.6}",
        actual,
        expected,
        (actual - expected).abs()
    );
}

/// Assert two floats are approximately equal within custom tolerance
pub fn assert_float_near(actual: f32, expected: f32, tolerance: f32) {
    assert!(
        (actual - expected).abs() < tolerance,
        "Float mismatch (tol={}):\n  actual:   {:.6}\n  expected: {:.6}\n  diff:     {:.6}",
        tolerance,
        actual,
        expected,
        (actual - expected).abs()
    );
}

/// Assert a float is finite (not NaN or Infinity)
pub fn assert_finite(value: f32, name: &str) {
    assert!(value.is_finite(), "{} is not finite: {}", name, value);
}

/// Assert vector is approximately unit length
pub fn assert_unit_vector(v: Vector3) {
    let len = v.length();
    assert!(
        (len - 1.0).abs() < TOLERANCE,
        "Vector not unit length:\n  vector: ({:.6}, {:.6}, {:.6})\n  length: {:.6}",
        v.x,
        v.y,
        v.z,
        len
    );
}

/// Assert two vectors are approximately orthogonal
pub fn assert_orthogonal(v1: Vector3, v2: Vector3) {
    let dot = v1.dot(v2);
    assert!(
        dot.abs() < TOLERANCE,
        "Vectors not orthogonal:\n  v1: ({:.6}, {:.6}, {:.6})\n  v2: ({:.6}, {:.6}, {:.6})\n  dot: {:.6}",
        v1.x, v1.y, v1.z,
        v2.x, v2.y, v2.z,
        dot
    );
}

/// Assert constraint is satisfied (error below tolerance)
pub fn assert_constraint_satisfied(sketch: &Sketch, constraint: &Constraint, tolerance: f32) {
    let error = constraint.evaluate(sketch);
    assert!(
        error < tolerance,
        "Constraint not satisfied:\n  error: {:.6}\n  tolerance: {:.6}",
        error,
        tolerance
    );
}

/// Assert all constraints are satisfied
pub fn assert_all_constraints_satisfied(
    sketch: &Sketch,
    constraints: &[Constraint],
    tolerance: f32,
) {
    for (i, constraint) in constraints.iter().enumerate() {
        let error = constraint.evaluate(sketch);
        assert!(
            error < tolerance,
            "Constraint {} not satisfied:\n  error: {:.6}\n  tolerance: {:.6}",
            i,
            error,
            tolerance
        );
    }
}

/// Assert no NaN or Infinity in sketch point positions
pub fn assert_sketch_finite(sketch: &Sketch) {
    for (i, point) in sketch.points.iter().enumerate() {
        assert!(
            point.position.x.is_finite(),
            "Point {} x is not finite: {}",
            i,
            point.position.x
        );
        assert!(
            point.position.y.is_finite(),
            "Point {} y is not finite: {}",
            i,
            point.position.y
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS FOR FIXTURES THEMSELVES
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sketch_fixture_rectangle() {
        let fixture = SketchFixture::rectangle(100.0, 50.0);
        assert_eq!(fixture.sketch.points.len(), 4);
        assert_eq!(fixture.sketch.entities.len(), 4);
    }

    #[test]
    fn test_sketch_fixture_with_horizontal() {
        let (sketch, constraints) = SketchFixture::horizontal_line(100.0)
            .with_horizontal()
            .build();

        assert_eq!(sketch.points.len(), 2);
        assert_eq!(sketch.entities.len(), 1);
        assert_eq!(constraints.len(), 1);
    }

    #[test]
    fn test_sketch_fixture_chaining() {
        let (sketch, constraints) =
            SketchFixture::two_points(Point2::new(0.0, 0.0), Point2::new(10.0, 5.0))
                .with_distance(15.0)
                .build();

        assert_eq!(sketch.points.len(), 2);
        assert_eq!(constraints.len(), 1);
    }

    #[test]
    fn test_solid_fixture_unit_cube() {
        let solid = SolidFixture::unit_cube();
        assert_eq!(solid.vertices.len(), 8);
        assert_eq!(solid.edges.len(), 12);
        assert_eq!(solid.faces.len(), 6);
    }

    #[test]
    fn test_assert_point3_eq_passes() {
        let p1 = Point3::new(1.0, 2.0, 3.0);
        let p2 = Point3::new(1.0, 2.0, 3.0);
        assert_point3_eq(p1, p2);
    }

    #[test]
    fn test_assert_float_eq_passes() {
        assert_float_eq(1.0, 1.0);
        assert_float_eq(1.0, 1.0 + 1e-7);
    }

    #[test]
    fn test_assert_unit_vector_passes() {
        assert_unit_vector(Vector3::X);
        assert_unit_vector(Vector3::Y);
        assert_unit_vector(Vector3::Z);
    }

    #[test]
    fn test_assert_orthogonal_passes() {
        assert_orthogonal(Vector3::X, Vector3::Y);
        assert_orthogonal(Vector3::Y, Vector3::Z);
        assert_orthogonal(Vector3::Z, Vector3::X);
    }
}
