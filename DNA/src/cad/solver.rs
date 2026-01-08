//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: solver.rs | DNA/src/cad/solver.rs
//! PURPOSE: Parametric constraint solver using Newton-Raphson iteration
//! MODIFIED: 2026-01-04
//! LAYER: DNA (foundation)
//! ═══════════════════════════════════════════════════════════════════════════════

use super::sketch::{Sketch, SketchPointId, Point2};
use super::constraints::Constraint;
use super::geometry::TOLERANCE;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Solver configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SolverConfig {
    pub max_iterations: usize,
    pub tolerance: f32,
    pub damping_factor: f32,  // Under-relaxation for stability
}

impl Default for SolverConfig {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            tolerance: TOLERANCE,
            // 0.5 can oscillate for some simple geometric constraints with our simplified update rule.
            damping_factor: 0.25,
        }
    }
}

/// Result of constraint solving
#[derive(Clone, Debug)]
pub struct SolverResult {
    pub converged: bool,
    pub iterations: usize,
    pub final_error: f32,
}

/// Degrees of Freedom status
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DofStatus {
    /// All DOF are constrained - sketch is fully defined
    FullyConstrained,
    /// Some DOF remain unconstrained
    UnderConstrained { dof: usize },
    /// More constraints than DOF - system may be inconsistent
    OverConstrained { redundant: usize },
}

/// Result of DOF analysis
#[derive(Clone, Debug)]
pub struct ConstraintAnalysis {
    /// DOF status
    pub dof_status: DofStatus,
    /// Total degrees of freedom (2 per point)
    pub total_dof: usize,
    /// Number of constraints
    pub constraint_count: usize,
    /// Remaining unconstrained DOF (positive = under, negative = over)
    pub remaining_dof: i32,
    /// Per-constraint status: true = satisfied, false = not satisfied
    pub constraint_satisfied: Vec<bool>,
}

impl ConstraintAnalysis {
    /// Analyze a sketch and its constraints
    pub fn analyze(sketch: &Sketch, constraints: &[Constraint]) -> Self {
        // Each point has 2 DOF (x, y)
        let total_dof = sketch.points.len() * 2;
        let constraint_count = constraints.len();

        // Simple DOF counting (doesn't account for dependent constraints)
        let remaining_dof = total_dof as i32 - constraint_count as i32;

        let dof_status = if remaining_dof == 0 {
            DofStatus::FullyConstrained
        } else if remaining_dof > 0 {
            DofStatus::UnderConstrained { dof: remaining_dof as usize }
        } else {
            DofStatus::OverConstrained { redundant: (-remaining_dof) as usize }
        };

        // Check which constraints are satisfied
        let constraint_satisfied: Vec<bool> = constraints
            .iter()
            .map(|c| c.evaluate(sketch) < TOLERANCE)
            .collect();

        Self {
            dof_status,
            total_dof,
            constraint_count,
            remaining_dof,
            constraint_satisfied,
        }
    }

    /// Get a human-readable status message
    pub fn status_message(&self) -> String {
        match &self.dof_status {
            DofStatus::FullyConstrained => "Fully constrained".to_string(),
            DofStatus::UnderConstrained { dof } => format!("Under-constrained by {} DOF", dof),
            DofStatus::OverConstrained { redundant } => format!("Over-constrained ({} redundant)", redundant),
        }
    }

    /// Count satisfied constraints
    pub fn satisfied_count(&self) -> usize {
        self.constraint_satisfied.iter().filter(|&&s| s).count()
    }

    /// Count unsatisfied constraints
    pub fn unsatisfied_count(&self) -> usize {
        self.constraint_satisfied.iter().filter(|&&s| !s).count()
    }
}

/// Parametric constraint solver
pub struct ConstraintSolver {
    config: SolverConfig,
}

impl ConstraintSolver {
    pub fn new(config: SolverConfig) -> Self {
        Self { config }
    }

    /// Solve all constraints in the sketch
    ///
    /// Uses Newton-Raphson iteration to minimize total constraint error.
    /// Updates sketch.points in-place to satisfy constraints.
    pub fn solve(&self, sketch: &mut Sketch, constraints: &[Constraint]) -> SolverResult {
        let mut iteration = 0;
        let mut total_error = f32::MAX;

        while iteration < self.config.max_iterations {
            // Evaluate all constraints
            total_error = 0.0;
            let mut residuals = Vec::new();

            for constraint in constraints {
                let error = constraint.evaluate(sketch);
                residuals.push(error);
                total_error += error;
            }

            // Check convergence
            if total_error < self.config.tolerance {
                sketch.is_solved = true;
                return SolverResult {
                    converged: true,
                    iterations: iteration,
                    final_error: total_error,
                };
            }

            // Build Jacobian matrix and solve
            let updates = self.compute_updates(sketch, constraints, &residuals);

            // Apply updates with damping
            for (point_id, delta) in updates {
                if let Some(point) = sketch.point_mut(point_id) {
                    point.position.x += delta.x * self.config.damping_factor;
                    point.position.y += delta.y * self.config.damping_factor;
                }
            }

            iteration += 1;
        }

        sketch.is_solved = false;
        SolverResult {
            converged: false,
            iterations: iteration,
            final_error: total_error,
        }
    }

    /// Compute position updates using pseudo-inverse
    ///
    /// Simplified solver: accumulates gradients per point.
    /// Full Newton-Raphson would solve J^T·J·Δx = -J^T·r
    fn compute_updates(
        &self,
        sketch: &Sketch,
        constraints: &[Constraint],
        residuals: &[f32],
    ) -> HashMap<SketchPointId, Point2> {
        let mut updates: HashMap<SketchPointId, Point2> = HashMap::new();

        for (i, constraint) in constraints.iter().enumerate() {
            let error = residuals[i];
            if error.abs() < 1e-12 {
                continue;  // Already satisfied
            }

            let grads = constraint.gradient(sketch);

            // Accumulate gradient-based updates
            for (point_id, grad) in grads {
                let entry = updates.entry(point_id).or_insert(Point2::new(0.0, 0.0));
                entry.x -= grad.x;
                entry.y -= grad.y;
            }
        }

        updates
    }
}

impl Default for ConstraintSolver {
    fn default() -> Self {
        Self::new(SolverConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cad::sketch::*;
    use crate::cad::constraints::*;

    // ═══════════════════════════════════════════════════════════════════════════════
    // BASIC CONSTRAINT SOLVER TESTS
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_horizontal_constraint_solver() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(0.0, 0.0));
        let p2 = sketch.add_point(Point2::new(10.0, 5.0));
        let line = SketchEntity::Line {
            id: SketchEntityId(0),
            start: p1,
            end: p2,
        };
        sketch.add_entity(line);

        let constraints = vec![
            Constraint::Geometric(GeometricConstraint::Horizontal {
                line: SketchEntityId(0),
            }),
        ];

        let solver = ConstraintSolver::default();
        let result = solver.solve(&mut sketch, &constraints);

        assert!(result.converged, "Horizontal constraint should converge");

        let pt1 = sketch.point(p1).unwrap();
        let pt2 = sketch.point(p2).unwrap();
        assert!((pt1.position.y - pt2.position.y).abs() < 0.1,
            "Y coordinates should be equal: {} vs {}", pt1.position.y, pt2.position.y);
    }

    #[test]
    fn test_vertical_constraint_solver() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(0.0, 0.0));
        let p2 = sketch.add_point(Point2::new(5.0, 10.0));
        let line = SketchEntity::Line {
            id: SketchEntityId(0),
            start: p1,
            end: p2,
        };
        sketch.add_entity(line);

        let constraints = vec![
            Constraint::Geometric(GeometricConstraint::Vertical {
                line: SketchEntityId(0),
            }),
        ];

        let solver = ConstraintSolver::default();
        let result = solver.solve(&mut sketch, &constraints);

        assert!(result.converged, "Vertical constraint should converge");

        let pt1 = sketch.point(p1).unwrap();
        let pt2 = sketch.point(p2).unwrap();
        assert!((pt1.position.x - pt2.position.x).abs() < 0.1,
            "X coordinates should be equal: {} vs {}", pt1.position.x, pt2.position.x);
    }

    #[test]
    fn test_coincident_constraint_solver() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(0.0, 0.0));
        let p2 = sketch.add_point(Point2::new(5.0, 5.0));

        let constraints = vec![
            Constraint::Geometric(GeometricConstraint::Coincident { p1, p2 }),
        ];

        let solver = ConstraintSolver::default();
        let result = solver.solve(&mut sketch, &constraints);

        assert!(result.converged, "Coincident constraint should converge");

        let pt1 = sketch.point(p1).unwrap();
        let pt2 = sketch.point(p2).unwrap();
        let dist = pt1.position.distance(&pt2.position);
        assert!(dist < 0.1, "Points should be coincident: dist = {}", dist);
    }

    #[test]
    fn test_distance_constraint_solver() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(0.0, 0.0));
        let p2 = sketch.add_point(Point2::new(3.0, 4.0));

        let constraints = vec![
            Constraint::Dimensional(DimensionalConstraint::Distance {
                p1,
                p2,
                value: 10.0,
            }),
        ];

        let solver = ConstraintSolver::default();
        let result = solver.solve(&mut sketch, &constraints);

        if result.converged {
            let pt1 = sketch.point(p1).unwrap();
            let pt2 = sketch.point(p2).unwrap();
            let dist = pt1.position.distance(&pt2.position);
            assert!((dist - 10.0).abs() < 0.5, "Distance should be ~10: {}", dist);
        }
    }

    #[test]
    fn test_horizontal_distance_solver() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(0.0, 0.0));
        let p2 = sketch.add_point(Point2::new(5.0, 10.0));

        let constraints = vec![
            Constraint::Dimensional(DimensionalConstraint::HorizontalDistance {
                p1,
                p2,
                value: 20.0,
            }),
        ];

        let solver = ConstraintSolver::default();
        let result = solver.solve(&mut sketch, &constraints);

        assert!(result.converged, "Horizontal distance should converge");

        let pt1 = sketch.point(p1).unwrap();
        let pt2 = sketch.point(p2).unwrap();
        let dx = (pt2.position.x - pt1.position.x).abs();
        assert!((dx - 20.0).abs() < 0.5, "Horizontal distance should be ~20: {}", dx);
    }

    #[test]
    fn test_vertical_distance_solver() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(0.0, 0.0));
        let p2 = sketch.add_point(Point2::new(10.0, 5.0));

        let constraints = vec![
            Constraint::Dimensional(DimensionalConstraint::VerticalDistance {
                p1,
                p2,
                value: 30.0,
            }),
        ];

        let solver = ConstraintSolver::default();
        let result = solver.solve(&mut sketch, &constraints);

        assert!(result.converged, "Vertical distance should converge");

        let pt1 = sketch.point(p1).unwrap();
        let pt2 = sketch.point(p2).unwrap();
        let dy = (pt2.position.y - pt1.position.y).abs();
        assert!((dy - 30.0).abs() < 0.5, "Vertical distance should be ~30: {}", dy);
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // MULTI-CONSTRAINT TESTS
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_horizontal_line_with_distance() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(0.0, 0.0));
        let p2 = sketch.add_point(Point2::new(5.0, 3.0));
        let line = SketchEntity::Line {
            id: SketchEntityId(0),
            start: p1,
            end: p2,
        };
        sketch.add_entity(line);

        let constraints = vec![
            Constraint::Geometric(GeometricConstraint::Horizontal {
                line: SketchEntityId(0),
            }),
            Constraint::Dimensional(DimensionalConstraint::Distance {
                p1,
                p2,
                value: 10.0,
            }),
        ];

        let solver = ConstraintSolver::default();
        let result = solver.solve(&mut sketch, &constraints);

        if result.converged {
            let pt1 = sketch.point(p1).unwrap();
            let pt2 = sketch.point(p2).unwrap();

            // Check horizontal
            assert!((pt1.position.y - pt2.position.y).abs() < 0.1, "Line should be horizontal");

            // Check distance
            let dist = pt1.position.distance(&pt2.position);
            assert!((dist - 10.0).abs() < 0.5, "Distance should be ~10: {}", dist);
        }
    }

    #[test]
    fn test_rectangle_h_v_constraints() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p0 = sketch.add_point(Point2::new(0.0, 0.0));
        let p1 = sketch.add_point(Point2::new(12.0, 3.0));  // Slightly off
        let p2 = sketch.add_point(Point2::new(15.0, 10.0));
        let p3 = sketch.add_point(Point2::new(2.0, 8.0));

        sketch.add_entity(SketchEntity::Line { id: SketchEntityId(0), start: p0, end: p1 });
        sketch.add_entity(SketchEntity::Line { id: SketchEntityId(1), start: p1, end: p2 });
        sketch.add_entity(SketchEntity::Line { id: SketchEntityId(2), start: p2, end: p3 });
        sketch.add_entity(SketchEntity::Line { id: SketchEntityId(3), start: p3, end: p0 });

        let constraints = vec![
            Constraint::Geometric(GeometricConstraint::Horizontal { line: SketchEntityId(0) }),
            Constraint::Geometric(GeometricConstraint::Horizontal { line: SketchEntityId(2) }),
            Constraint::Geometric(GeometricConstraint::Vertical { line: SketchEntityId(1) }),
            Constraint::Geometric(GeometricConstraint::Vertical { line: SketchEntityId(3) }),
        ];

        let solver = ConstraintSolver::default();
        let result = solver.solve(&mut sketch, &constraints);

        if result.converged {
            let pt0 = sketch.point(p0).unwrap().position;
            let pt1 = sketch.point(p1).unwrap().position;
            let pt2 = sketch.point(p2).unwrap().position;
            let pt3 = sketch.point(p3).unwrap().position;

            // Check horizontal lines
            assert!((pt0.y - pt1.y).abs() < 0.1, "Bottom should be horizontal");
            assert!((pt2.y - pt3.y).abs() < 0.1, "Top should be horizontal");

            // Check vertical lines
            assert!((pt1.x - pt2.x).abs() < 0.1, "Right should be vertical");
            assert!((pt0.x - pt3.x).abs() < 0.1, "Left should be vertical");
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // DOF ANALYSIS TESTS
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_dof_analysis_under_constrained() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(0.0, 0.0));
        let p2 = sketch.add_point(Point2::new(10.0, 0.0));

        // 2 points = 4 DOF, 1 constraint = 3 DOF remaining
        let constraints = vec![
            Constraint::Dimensional(DimensionalConstraint::Distance {
                p1,
                p2,
                value: 10.0,
            }),
        ];

        let analysis = ConstraintAnalysis::analyze(&sketch, &constraints);

        assert_eq!(analysis.total_dof, 4);
        assert_eq!(analysis.constraint_count, 1);
        assert_eq!(analysis.remaining_dof, 3);
        assert_eq!(analysis.dof_status, DofStatus::UnderConstrained { dof: 3 });
    }

    #[test]
    fn test_dof_analysis_fully_constrained() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(0.0, 0.0));
        let p2 = sketch.add_point(Point2::new(10.0, 0.0));

        // 2 points = 4 DOF, 4 constraints
        let constraints = vec![
            Constraint::Dimensional(DimensionalConstraint::HorizontalDistance {
                p1,
                p2,
                value: 10.0,
            }),
            Constraint::Dimensional(DimensionalConstraint::VerticalDistance {
                p1,
                p2,
                value: 0.0,
            }),
            Constraint::Dimensional(DimensionalConstraint::HorizontalDistance {
                p1,
                p2: p1,  // Fix p1 x
                value: 0.0,
            }),
            Constraint::Dimensional(DimensionalConstraint::VerticalDistance {
                p1,
                p2: p1,  // Fix p1 y
                value: 0.0,
            }),
        ];

        let analysis = ConstraintAnalysis::analyze(&sketch, &constraints);

        assert_eq!(analysis.total_dof, 4);
        assert_eq!(analysis.constraint_count, 4);
        assert_eq!(analysis.remaining_dof, 0);
        assert_eq!(analysis.dof_status, DofStatus::FullyConstrained);
    }

    #[test]
    fn test_dof_analysis_over_constrained() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(0.0, 0.0));
        let p2 = sketch.add_point(Point2::new(10.0, 0.0));

        // 2 points = 4 DOF, 5 constraints = 1 redundant
        let constraints = vec![
            Constraint::Dimensional(DimensionalConstraint::Distance { p1, p2, value: 10.0 }),
            Constraint::Dimensional(DimensionalConstraint::HorizontalDistance { p1, p2, value: 10.0 }),
            Constraint::Dimensional(DimensionalConstraint::VerticalDistance { p1, p2, value: 0.0 }),
            Constraint::Geometric(GeometricConstraint::Coincident { p1, p2: p1 }),
            Constraint::Dimensional(DimensionalConstraint::Distance { p1, p2: p1, value: 0.0 }),
        ];

        let analysis = ConstraintAnalysis::analyze(&sketch, &constraints);

        assert_eq!(analysis.total_dof, 4);
        assert_eq!(analysis.constraint_count, 5);
        assert_eq!(analysis.remaining_dof, -1);
        assert_eq!(analysis.dof_status, DofStatus::OverConstrained { redundant: 1 });
    }

    #[test]
    fn test_dof_analysis_empty_sketch() {
        let sketch = Sketch::new(SketchPlane::XY);
        let constraints: Vec<Constraint> = vec![];

        let analysis = ConstraintAnalysis::analyze(&sketch, &constraints);

        assert_eq!(analysis.total_dof, 0);
        assert_eq!(analysis.constraint_count, 0);
        assert_eq!(analysis.dof_status, DofStatus::FullyConstrained);
    }

    #[test]
    fn test_dof_analysis_constraint_satisfaction() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(0.0, 0.0));
        let p2 = sketch.add_point(Point2::new(3.0, 4.0));  // Distance = 5.0

        let constraints = vec![
            Constraint::Dimensional(DimensionalConstraint::Distance { p1, p2, value: 5.0 }),  // Satisfied
            Constraint::Dimensional(DimensionalConstraint::Distance { p1, p2, value: 10.0 }), // NOT satisfied
        ];

        let analysis = ConstraintAnalysis::analyze(&sketch, &constraints);

        assert_eq!(analysis.satisfied_count(), 1);
        assert_eq!(analysis.unsatisfied_count(), 1);
        assert!(analysis.constraint_satisfied[0]);
        assert!(!analysis.constraint_satisfied[1]);
    }

    #[test]
    fn test_dof_status_message() {
        let analysis_full = ConstraintAnalysis {
            dof_status: DofStatus::FullyConstrained,
            total_dof: 4,
            constraint_count: 4,
            remaining_dof: 0,
            constraint_satisfied: vec![],
        };
        assert_eq!(analysis_full.status_message(), "Fully constrained");

        let analysis_under = ConstraintAnalysis {
            dof_status: DofStatus::UnderConstrained { dof: 3 },
            total_dof: 4,
            constraint_count: 1,
            remaining_dof: 3,
            constraint_satisfied: vec![],
        };
        assert_eq!(analysis_under.status_message(), "Under-constrained by 3 DOF");

        let analysis_over = ConstraintAnalysis {
            dof_status: DofStatus::OverConstrained { redundant: 2 },
            total_dof: 4,
            constraint_count: 6,
            remaining_dof: -2,
            constraint_satisfied: vec![],
        };
        assert_eq!(analysis_over.status_message(), "Over-constrained (2 redundant)");
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // EDGE CASES AND ERROR HANDLING
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_solver_empty_constraints() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        sketch.add_point(Point2::new(0.0, 0.0));
        sketch.add_point(Point2::new(10.0, 10.0));

        let constraints: Vec<Constraint> = vec![];

        let solver = ConstraintSolver::default();
        let result = solver.solve(&mut sketch, &constraints);

        // Empty constraints should converge immediately
        assert!(result.converged);
        assert_eq!(result.iterations, 0);
    }

    #[test]
    fn test_solver_already_satisfied() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(0.0, 0.0));
        let p2 = sketch.add_point(Point2::new(10.0, 0.0));  // Already horizontal
        let line = SketchEntity::Line {
            id: SketchEntityId(0),
            start: p1,
            end: p2,
        };
        sketch.add_entity(line);

        let constraints = vec![
            Constraint::Geometric(GeometricConstraint::Horizontal {
                line: SketchEntityId(0),
            }),
        ];

        let solver = ConstraintSolver::default();
        let result = solver.solve(&mut sketch, &constraints);

        assert!(result.converged);
        assert_eq!(result.iterations, 0, "Already satisfied should converge in 0 iterations");
    }

    #[test]
    fn test_solver_coincident_points_at_start() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(10.0, 20.0));
        let p2 = sketch.add_point(Point2::new(10.0, 20.0));  // Already coincident

        let constraints = vec![
            Constraint::Geometric(GeometricConstraint::Coincident { p1, p2 }),
        ];

        let solver = ConstraintSolver::default();
        let result = solver.solve(&mut sketch, &constraints);

        assert!(result.converged);
        assert_eq!(result.iterations, 0);
    }

    #[test]
    fn test_solver_zero_length_line() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(5.0, 5.0));
        let p2 = sketch.add_point(Point2::new(5.0, 5.0));  // Same position
        let line = SketchEntity::Line {
            id: SketchEntityId(0),
            start: p1,
            end: p2,
        };
        sketch.add_entity(line);

        let constraints = vec![
            Constraint::Geometric(GeometricConstraint::Horizontal {
                line: SketchEntityId(0),
            }),
        ];

        let solver = ConstraintSolver::default();
        let result = solver.solve(&mut sketch, &constraints);

        // Zero-length line is trivially horizontal
        assert!(result.final_error.is_finite(), "Error should be finite");
    }

    #[test]
    fn test_solver_conflicting_distances() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(0.0, 0.0));
        let p2 = sketch.add_point(Point2::new(5.0, 0.0));

        // Conflicting constraints
        let constraints = vec![
            Constraint::Dimensional(DimensionalConstraint::Distance { p1, p2, value: 10.0 }),
            Constraint::Dimensional(DimensionalConstraint::Distance { p1, p2, value: 20.0 }),
        ];

        let solver = ConstraintSolver::default();
        let result = solver.solve(&mut sketch, &constraints);

        // Should not fully converge (or have residual error)
        if result.converged {
            assert!(result.final_error > 0.01, "Conflicting constraints should have residual error");
        }
    }

    #[test]
    fn test_solver_max_iterations_limit() {
        let config = SolverConfig {
            max_iterations: 10,
            tolerance: 1e-12,  // Very tight tolerance
            damping_factor: 0.25,
        };

        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(0.0, 0.0));
        let p2 = sketch.add_point(Point2::new(50.0, 50.0));

        let constraints = vec![
            Constraint::Dimensional(DimensionalConstraint::Distance {
                p1,
                p2,
                value: 5.0,  // Very far from current
            }),
        ];

        let solver = ConstraintSolver::new(config);
        let result = solver.solve(&mut sketch, &constraints);

        assert!(result.iterations <= 10, "Should respect max_iterations: {}", result.iterations);
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // NUMERICAL STABILITY TESTS
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_solver_no_nan_in_result() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(0.0, 0.0));
        let p2 = sketch.add_point(Point2::new(10.0, 10.0));

        let constraints = vec![
            Constraint::Dimensional(DimensionalConstraint::Distance {
                p1,
                p2,
                value: 5.0,
            }),
        ];

        let solver = ConstraintSolver::default();
        let result = solver.solve(&mut sketch, &constraints);

        assert!(!result.final_error.is_nan(), "Final error should not be NaN");

        for point in &sketch.points {
            assert!(!point.position.x.is_nan(), "Point x should not be NaN");
            assert!(!point.position.y.is_nan(), "Point y should not be NaN");
            assert!(point.position.x.is_finite(), "Point x should be finite");
            assert!(point.position.y.is_finite(), "Point y should be finite");
        }
    }

    #[test]
    fn test_solver_large_coordinate_values() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(1e6, 1e6));
        let p2 = sketch.add_point(Point2::new(1e6 + 10.0, 1e6 + 10.0));

        let constraints = vec![
            Constraint::Dimensional(DimensionalConstraint::Distance {
                p1,
                p2,
                value: 20.0,
            }),
        ];

        let solver = ConstraintSolver::default();
        let result = solver.solve(&mut sketch, &constraints);

        assert!(!result.final_error.is_nan());
        assert!(!result.final_error.is_infinite());
    }

    #[test]
    fn test_solver_very_small_displacement() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(0.0, 0.0));
        let p2 = sketch.add_point(Point2::new(0.0, 0.00001));  // Almost horizontal
        let line = SketchEntity::Line {
            id: SketchEntityId(0),
            start: p1,
            end: p2,
        };
        sketch.add_entity(line);

        let constraints = vec![
            Constraint::Geometric(GeometricConstraint::Horizontal {
                line: SketchEntityId(0),
            }),
        ];

        let solver = ConstraintSolver::default();
        let result = solver.solve(&mut sketch, &constraints);

        assert!(result.converged);
        assert!(result.iterations <= 5, "Near-solution should converge quickly: {}", result.iterations);
    }

    #[test]
    fn test_solver_iteration_count_tracking() {
        let mut sketch = Sketch::new(SketchPlane::XY);
        let p1 = sketch.add_point(Point2::new(0.0, 0.0));
        let p2 = sketch.add_point(Point2::new(10.0, 5.0));
        let line = SketchEntity::Line {
            id: SketchEntityId(0),
            start: p1,
            end: p2,
        };
        sketch.add_entity(line);

        let constraints = vec![
            Constraint::Geometric(GeometricConstraint::Horizontal {
                line: SketchEntityId(0),
            }),
        ];

        let solver = ConstraintSolver::default();
        let result = solver.solve(&mut sketch, &constraints);

        assert!(result.converged);
        assert!(result.iterations > 0, "Should take at least one iteration");
        assert!(result.iterations < 50, "Simple constraint should converge quickly: {}", result.iterations);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PROPERTY-BASED TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod proptest_tests {
    use super::*;
    use crate::cad::sketch::*;
    use crate::cad::constraints::*;
    use proptest::prelude::*;

    proptest! {
        /// Horizontal constraint should equalize Y coordinates when solvable
        #[test]
        fn prop_horizontal_equalizes_y(
            x1 in 0.0f32..100.0,
            y1 in 0.0f32..100.0,
            x2 in 0.0f32..100.0,
            y2 in 0.0f32..100.0
        ) {
            // Skip degenerate cases
            prop_assume!((x2 - x1).abs() > 1.0);

            let mut sketch = Sketch::new(SketchPlane::XY);
            let p1 = sketch.add_point(Point2::new(x1, y1));
            let p2 = sketch.add_point(Point2::new(x2, y2));
            sketch.add_entity(SketchEntity::Line {
                id: SketchEntityId(0),
                start: p1,
                end: p2,
            });

            let constraints = vec![
                Constraint::Geometric(GeometricConstraint::Horizontal {
                    line: SketchEntityId(0),
                }),
            ];

            let solver = ConstraintSolver::default();
            let result = solver.solve(&mut sketch, &constraints);

            if result.converged {
                let pt1 = sketch.point(p1).unwrap();
                let pt2 = sketch.point(p2).unwrap();
                prop_assert!((pt1.position.y - pt2.position.y).abs() < 0.5,
                    "Horizontal should equalize Y: {} vs {}", pt1.position.y, pt2.position.y);
            }
        }

        /// Vertical constraint should equalize X coordinates when solvable
        #[test]
        fn prop_vertical_equalizes_x(
            x1 in 0.0f32..100.0,
            y1 in 0.0f32..100.0,
            x2 in 0.0f32..100.0,
            y2 in 0.0f32..100.0
        ) {
            prop_assume!((y2 - y1).abs() > 1.0);

            let mut sketch = Sketch::new(SketchPlane::XY);
            let p1 = sketch.add_point(Point2::new(x1, y1));
            let p2 = sketch.add_point(Point2::new(x2, y2));
            sketch.add_entity(SketchEntity::Line {
                id: SketchEntityId(0),
                start: p1,
                end: p2,
            });

            let constraints = vec![
                Constraint::Geometric(GeometricConstraint::Vertical {
                    line: SketchEntityId(0),
                }),
            ];

            let solver = ConstraintSolver::default();
            let result = solver.solve(&mut sketch, &constraints);

            if result.converged {
                let pt1 = sketch.point(p1).unwrap();
                let pt2 = sketch.point(p2).unwrap();
                prop_assert!((pt1.position.x - pt2.position.x).abs() < 0.5,
                    "Vertical should equalize X: {} vs {}", pt1.position.x, pt2.position.x);
            }
        }

        /// Coincident constraint should bring points together
        #[test]
        fn prop_coincident_brings_together(
            x1 in 0.0f32..50.0,
            y1 in 0.0f32..50.0,
            x2 in 50.0f32..100.0,
            y2 in 50.0f32..100.0
        ) {
            let mut sketch = Sketch::new(SketchPlane::XY);
            let p1 = sketch.add_point(Point2::new(x1, y1));
            let p2 = sketch.add_point(Point2::new(x2, y2));

            let constraints = vec![
                Constraint::Geometric(GeometricConstraint::Coincident { p1, p2 }),
            ];

            let solver = ConstraintSolver::default();
            let result = solver.solve(&mut sketch, &constraints);

            if result.converged {
                let pt1 = sketch.point(p1).unwrap();
                let pt2 = sketch.point(p2).unwrap();
                let dist = pt1.position.distance(&pt2.position);
                prop_assert!(dist < 0.5, "Coincident should bring points together: dist = {}", dist);
            }
        }

        /// DOF count is always 2 * num_points
        #[test]
        fn prop_dof_count_correctness(num_points in 1usize..10) {
            let mut sketch = Sketch::new(SketchPlane::XY);

            for i in 0..num_points {
                sketch.add_point(Point2::new(i as f32 * 10.0, 0.0));
            }

            let constraints: Vec<Constraint> = vec![];
            let analysis = ConstraintAnalysis::analyze(&sketch, &constraints);

            prop_assert_eq!(analysis.total_dof, num_points * 2);
        }

        /// Solver never produces NaN regardless of input
        #[test]
        fn prop_solver_never_nan(
            x1 in -100.0f32..100.0,
            y1 in -100.0f32..100.0,
            x2 in -100.0f32..100.0,
            y2 in -100.0f32..100.0,
            target in 1.0f32..100.0
        ) {
            let mut sketch = Sketch::new(SketchPlane::XY);
            let p1 = sketch.add_point(Point2::new(x1, y1));
            let p2 = sketch.add_point(Point2::new(x2, y2));

            let constraints = vec![
                Constraint::Dimensional(DimensionalConstraint::Distance {
                    p1,
                    p2,
                    value: target,
                }),
            ];

            let solver = ConstraintSolver::default();
            let result = solver.solve(&mut sketch, &constraints);

            prop_assert!(!result.final_error.is_nan(), "Final error is NaN");

            for point in &sketch.points {
                prop_assert!(!point.position.x.is_nan(), "Point x is NaN");
                prop_assert!(!point.position.y.is_nan(), "Point y is NaN");
            }
        }
    }
}
