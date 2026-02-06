//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: constraints.rs | DNA/src/cad/constraints.rs
//! PURPOSE: Parametric constraints for 2D sketches
//! MODIFIED: 2026-01-04
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

use super::sketch::{Point2, Sketch, SketchEntity, SketchEntityId, SketchPointId};
use serde::{Deserialize, Serialize};

/// Geometric constraint (maintains relationships)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GeometricConstraint {
    /// Line is horizontal (parallel to X axis)
    Horizontal { line: SketchEntityId },

    /// Line is vertical (parallel to Y axis)
    Vertical { line: SketchEntityId },

    /// Two lines are parallel
    Parallel {
        line1: SketchEntityId,
        line2: SketchEntityId,
    },

    /// Two lines are perpendicular
    Perpendicular {
        line1: SketchEntityId,
        line2: SketchEntityId,
    },

    /// Two points are coincident
    Coincident {
        p1: SketchPointId,
        p2: SketchPointId,
    },

    /// Arc/Circle are tangent
    Tangent {
        entity1: SketchEntityId,
        entity2: SketchEntityId,
    },

    /// Circles/arcs are concentric
    Concentric {
        entity1: SketchEntityId,
        entity2: SketchEntityId,
    },
}

/// Dimensional constraint (drives specific values)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DimensionalConstraint {
    /// Distance between two points
    Distance {
        p1: SketchPointId,
        p2: SketchPointId,
        value: f32,
    },

    /// Horizontal distance (X-direction only)
    HorizontalDistance {
        p1: SketchPointId,
        p2: SketchPointId,
        value: f32,
    },

    /// Vertical distance (Y-direction only)
    VerticalDistance {
        p1: SketchPointId,
        p2: SketchPointId,
        value: f32,
    },

    /// Angle between two lines (radians)
    Angle {
        line1: SketchEntityId,
        line2: SketchEntityId,
        value: f32,
    },

    /// Radius of arc or circle
    Radius { entity: SketchEntityId, value: f32 },

    /// Diameter of arc or circle
    Diameter { entity: SketchEntityId, value: f32 },
}

/// Unified constraint type
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Constraint {
    Geometric(GeometricConstraint),
    Dimensional(DimensionalConstraint),
}

impl Constraint {
    /// Evaluate constraint error
    ///
    /// Returns 0.0 if constraint is perfectly satisfied.
    /// For geometric constraints, returns squared error.
    /// For dimensional constraints, returns (actual - target)².
    pub fn evaluate(&self, sketch: &Sketch) -> f32 {
        match self {
            Constraint::Geometric(gc) => evaluate_geometric(gc, sketch),
            Constraint::Dimensional(dc) => evaluate_dimensional(dc, sketch),
        }
    }

    /// Compute gradient (partial derivatives) for solver
    ///
    /// Returns list of (point_id, gradient_vector) pairs.
    /// gradient_vector.x = ∂error/∂point.x
    /// gradient_vector.y = ∂error/∂point.y
    pub fn gradient(&self, sketch: &Sketch) -> Vec<(SketchPointId, Point2)> {
        match self {
            Constraint::Geometric(gc) => gradient_geometric(gc, sketch),
            Constraint::Dimensional(dc) => gradient_dimensional(dc, sketch),
        }
    }
}

fn evaluate_geometric(constraint: &GeometricConstraint, sketch: &Sketch) -> f32 {
    match constraint {
        GeometricConstraint::Horizontal { line } => {
            if let Some(SketchEntity::Line { start, end, .. }) = sketch.entity(*line) {
                if let (Some(p1), Some(p2)) = (sketch.point(*start), sketch.point(*end)) {
                    let dy = p2.position.y - p1.position.y;
                    return dy * dy; // Squared error
                }
            }
            0.0
        }

        GeometricConstraint::Vertical { line } => {
            if let Some(SketchEntity::Line { start, end, .. }) = sketch.entity(*line) {
                if let (Some(p1), Some(p2)) = (sketch.point(*start), sketch.point(*end)) {
                    let dx = p2.position.x - p1.position.x;
                    return dx * dx;
                }
            }
            0.0
        }

        GeometricConstraint::Coincident { p1, p2 } => {
            if let (Some(pt1), Some(pt2)) = (sketch.point(*p1), sketch.point(*p2)) {
                return pt1.position.distance_squared(&pt2.position);
            }
            0.0
        }

        _ => 0.0, // Other constraints not yet implemented
    }
}

fn evaluate_dimensional(constraint: &DimensionalConstraint, sketch: &Sketch) -> f32 {
    match constraint {
        DimensionalConstraint::Distance { p1, p2, value } => {
            if let (Some(pt1), Some(pt2)) = (sketch.point(*p1), sketch.point(*p2)) {
                let actual = pt1.position.distance(&pt2.position);
                let error = actual - value;
                return error * error;
            }
            0.0
        }

        DimensionalConstraint::HorizontalDistance { p1, p2, value } => {
            if let (Some(pt1), Some(pt2)) = (sketch.point(*p1), sketch.point(*p2)) {
                let actual = (pt2.position.x - pt1.position.x).abs();
                let error = actual - value;
                return error * error;
            }
            0.0
        }

        DimensionalConstraint::VerticalDistance { p1, p2, value } => {
            if let (Some(pt1), Some(pt2)) = (sketch.point(*p1), sketch.point(*p2)) {
                let actual = (pt2.position.y - pt1.position.y).abs();
                let error = actual - value;
                return error * error;
            }
            0.0
        }

        DimensionalConstraint::Radius { entity, value } => {
            if let Some(SketchEntity::Circle { radius, .. }) = sketch.entity(*entity) {
                let error = radius - value;
                return error * error;
            }
            0.0
        }

        _ => 0.0, // Other constraints not yet implemented
    }
}

fn gradient_geometric(
    constraint: &GeometricConstraint,
    sketch: &Sketch,
) -> Vec<(SketchPointId, Point2)> {
    match constraint {
        GeometricConstraint::Horizontal { line } => {
            if let Some(SketchEntity::Line { start, end, .. }) = sketch.entity(*line) {
                if let (Some(p1), Some(p2)) = (sketch.point(*start), sketch.point(*end)) {
                    let dy = p2.position.y - p1.position.y;
                    // ∂(dy²)/∂y1 = -2dy, ∂(dy²)/∂y2 = +2dy
                    return vec![
                        (*start, Point2::new(0.0, -2.0 * dy)),
                        (*end, Point2::new(0.0, 2.0 * dy)),
                    ];
                }
            }
            Vec::new()
        }

        GeometricConstraint::Vertical { line } => {
            if let Some(SketchEntity::Line { start, end, .. }) = sketch.entity(*line) {
                if let (Some(p1), Some(p2)) = (sketch.point(*start), sketch.point(*end)) {
                    let dx = p2.position.x - p1.position.x;
                    // ∂(dx²)/∂x1 = -2dx, ∂(dx²)/∂x2 = +2dx
                    return vec![
                        (*start, Point2::new(-2.0 * dx, 0.0)),
                        (*end, Point2::new(2.0 * dx, 0.0)),
                    ];
                }
            }
            Vec::new()
        }

        GeometricConstraint::Coincident { p1, p2 } => {
            if let (Some(pt1), Some(pt2)) = (sketch.point(*p1), sketch.point(*p2)) {
                let dx = pt2.position.x - pt1.position.x;
                let dy = pt2.position.y - pt1.position.y;
                // ∂((dx² + dy²))/∂p1 = -2(dx, dy), ∂/∂p2 = +2(dx, dy)
                return vec![
                    (*p1, Point2::new(-2.0 * dx, -2.0 * dy)),
                    (*p2, Point2::new(2.0 * dx, 2.0 * dy)),
                ];
            }
            Vec::new()
        }

        _ => Vec::new(),
    }
}

fn gradient_dimensional(
    constraint: &DimensionalConstraint,
    sketch: &Sketch,
) -> Vec<(SketchPointId, Point2)> {
    match constraint {
        DimensionalConstraint::Distance { p1, p2, value } => {
            if let (Some(pt1), Some(pt2)) = (sketch.point(*p1), sketch.point(*p2)) {
                let dx = pt2.position.x - pt1.position.x;
                let dy = pt2.position.y - pt1.position.y;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < 1e-8 {
                    return Vec::new(); // Degenerate
                }

                let error = dist - value;
                // ∂((dist - d)²)/∂p1 = -2(error)(dx/dist, dy/dist)
                let grad_mag = 2.0 * error / dist;

                return vec![
                    (*p1, Point2::new(-grad_mag * dx, -grad_mag * dy)),
                    (*p2, Point2::new(grad_mag * dx, grad_mag * dy)),
                ];
            }
            Vec::new()
        }

        DimensionalConstraint::HorizontalDistance { p1, p2, value } => {
            if let (Some(pt1), Some(pt2)) = (sketch.point(*p1), sketch.point(*p2)) {
                let dx = pt2.position.x - pt1.position.x;
                let actual = dx.abs();
                let error = actual - value;
                let sign = if dx >= 0.0 { 1.0 } else { -1.0 };

                return vec![
                    (*p1, Point2::new(-2.0 * error * sign, 0.0)),
                    (*p2, Point2::new(2.0 * error * sign, 0.0)),
                ];
            }
            Vec::new()
        }

        DimensionalConstraint::VerticalDistance { p1, p2, value } => {
            if let (Some(pt1), Some(pt2)) = (sketch.point(*p1), sketch.point(*p2)) {
                let dy = pt2.position.y - pt1.position.y;
                let actual = dy.abs();
                let error = actual - value;
                let sign = if dy >= 0.0 { 1.0 } else { -1.0 };

                return vec![
                    (*p1, Point2::new(0.0, -2.0 * error * sign)),
                    (*p2, Point2::new(0.0, 2.0 * error * sign)),
                ];
            }
            Vec::new()
        }

        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cad::sketch::*;

    #[test]
    fn test_horizontal_constraint_evaluation() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(0.0, 0.0));
        let p2 = sketch.add_point(Point2::new(10.0, 5.0));
        let line = SketchEntity::Line {
            id: SketchEntityId(0),
            start: p1,
            end: p2,
        };
        let line_id = sketch.add_entity(line);

        let constraint = Constraint::Geometric(GeometricConstraint::Horizontal { line: line_id });
        let error = constraint.evaluate(&sketch);

        // Error should be dy² = 25.0
        assert!((error - 25.0).abs() < 1e-3);
    }

    #[test]
    fn test_distance_constraint_gradient() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(0.0, 0.0));
        let p2 = sketch.add_point(Point2::new(3.0, 4.0));

        let constraint = Constraint::Dimensional(DimensionalConstraint::Distance {
            p1,
            p2,
            value: 10.0,
        });

        let grads = constraint.gradient(&sketch);
        assert_eq!(grads.len(), 2);
    }
}
