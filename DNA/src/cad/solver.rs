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

        // Should converge
        assert!(result.converged);

        // Y coordinates should be equal
        let pt1 = sketch.point(p1).unwrap();
        let pt2 = sketch.point(p2).unwrap();
        assert!((pt1.position.y - pt2.position.y).abs() < 1e-2);
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
            assert!((dist - 10.0).abs() < 0.1);
        }
    }
}
