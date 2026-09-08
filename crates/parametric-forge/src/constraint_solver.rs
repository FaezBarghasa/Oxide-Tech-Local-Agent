use serde::{Deserialize, Serialize};

use crate::ParametricError;

/// Geometric 2D constraint applied to sketch control points.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "constraint_type", rename_all = "snake_case")]
pub enum GeometricConstraint {
    /// Fix a specific point to a constant (x, y) coordinate.
    FixPoint { point: usize, x: f64, y: f64 },
    /// Constrain two points to share the same Y coordinate (y2 - y1 = 0).
    Horizontal { p1: usize, p2: usize },
    /// Constrain two points to share the same X coordinate (x2 - x1 = 0).
    Vertical { p1: usize, p2: usize },
    /// Constrain distance between two points to a target length.
    Distance { p1: usize, p2: usize, distance: f64 },
    /// Constrain two points to be identical in (x, y).
    Coincident { p1: usize, p2: usize },
    /// Constrain a point to lie on the line formed by line_p1 and line_p2.
    PointOnLine {
        point: usize,
        line_p1: usize,
        line_p2: usize,
    },
}

/// Deterministic 2D Newton-Raphson Geometric Constraint Solver.
#[derive(Debug, Default)]
pub struct ConstraintSolver;

impl ConstraintSolver {
    pub fn new() -> Self {
        Self
    }

    /// Solves the 2D sketch point coordinates deterministically to satisfy all geometric constraints.
    pub fn solve_sketch(
        constraints: &[GeometricConstraint],
        points: &mut [[f64; 2]],
        max_iterations: usize,
        tolerance: f64,
    ) -> Result<(), ParametricError> {
        let num_points = points.len();
        if num_points == 0 || constraints.is_empty() {
            return Ok(());
        }

        let num_vars = num_points * 2;
        let mut x = vec![0.0f64; num_vars];
        for i in 0..num_points {
            x[2 * i] = points[i][0];
            x[2 * i + 1] = points[i][1];
        }

        let mut lambda = 1e-4; // Levenberg-Marquardt damping factor

        for _iter in 0..max_iterations {
            let residuals = Self::calculate_residuals(constraints, &x, num_points);
            let residual_norm = Self::vector_norm(&residuals);

            if residual_norm < tolerance {
                // Constraints satisfied! Copy solved coordinates back to points
                for i in 0..num_points {
                    points[i][0] = x[2 * i];
                    points[i][1] = x[2 * i + 1];
                }
                return Ok(());
            }

            // Calculate Jacobian numerically via central finite differences
            let jacobian = Self::calculate_jacobian(constraints, &x, num_points);
            let m = residuals.len();
            let n = num_vars;

            // Compute Normal Equations: (J^T * J + lambda * I) * delta = -J^T * residuals
            let mut jtj = vec![vec![0.0f64; n]; n];
            let mut jtf = vec![0.0f64; n];

            for i in 0..n {
                for j in 0..n {
                    let mut sum = 0.0f64;
                    for k in 0..m {
                        sum += jacobian[k][i] * jacobian[k][j];
                    }
                    if i == j {
                        sum += lambda;
                    }
                    jtj[i][j] = sum;
                }

                let mut sum_f = 0.0f64;
                for k in 0..m {
                    sum_f += jacobian[k][i] * residuals[k];
                }
                jtf[i] = -sum_f;
            }

            // Solve linear system: jtj * delta = jtf
            if let Some(delta) = Self::solve_linear_system(&jtj, &jtf) {
                let mut x_candidate = x.clone();
                for i in 0..n {
                    x_candidate[i] += delta[i];
                }

                let candidate_residuals =
                    Self::calculate_residuals(constraints, &x_candidate, num_points);
                let candidate_norm = Self::vector_norm(&candidate_residuals);

                if candidate_norm < residual_norm {
                    x = x_candidate;
                    lambda = (lambda * 0.5).max(1e-7);
                } else {
                    lambda = (lambda * 2.0).min(1e3);
                }
            } else {
                lambda *= 10.0;
            }
        }

        // Final check after iterations
        let final_residuals = Self::calculate_residuals(constraints, &x, num_points);
        if Self::vector_norm(&final_residuals) < tolerance {
            for i in 0..num_points {
                points[i][0] = x[2 * i];
                points[i][1] = x[2 * i + 1];
            }
            Ok(())
        } else {
            Err(ParametricError::OverConstrainedOrUnsolvable(format!(
                "Constraint solver failed to converge. Residual norm: {:.6}",
                Self::vector_norm(&final_residuals)
            )))
        }
    }

    /// Evaluates constraint equations $F(x) = 0$.
    fn calculate_residuals(
        constraints: &[GeometricConstraint],
        x: &[f64],
        num_points: usize,
    ) -> Vec<f64> {
        let mut residuals = Vec::new();

        for c in constraints {
            match c {
                GeometricConstraint::FixPoint { point, x: fx, y: fy } => {
                    if *point < num_points {
                        residuals.push(x[2 * point] - fx);
                        residuals.push(x[2 * point + 1] - fy);
                    }
                }
                GeometricConstraint::Horizontal { p1, p2 } => {
                    if *p1 < num_points && *p2 < num_points {
                        residuals.push(x[2 * p2 + 1] - x[2 * p1 + 1]);
                    }
                }
                GeometricConstraint::Vertical { p1, p2 } => {
                    if *p1 < num_points && *p2 < num_points {
                        residuals.push(x[2 * p2] - x[2 * p1]);
                    }
                }
                GeometricConstraint::Distance { p1, p2, distance } => {
                    if *p1 < num_points && *p2 < num_points {
                        let dx = x[2 * p2] - x[2 * p1];
                        let dy = x[2 * p2 + 1] - x[2 * p1 + 1];
                        let current_dist = (dx * dx + dy * dy).sqrt();
                        residuals.push(current_dist - distance);
                    }
                }
                GeometricConstraint::Coincident { p1, p2 } => {
                    if *p1 < num_points && *p2 < num_points {
                        residuals.push(x[2 * p1] - x[2 * p2]);
                        residuals.push(x[2 * p1 + 1] - x[2 * p2 + 1]);
                    }
                }
                GeometricConstraint::PointOnLine {
                    point,
                    line_p1,
                    line_p2,
                } => {
                    if *point < num_points && *line_p1 < num_points && *line_p2 < num_points {
                        let px = x[2 * point];
                        let py = x[2 * point + 1];
                        let x1 = x[2 * line_p1];
                        let y1 = x[2 * line_p1 + 1];
                        let x2 = x[2 * line_p2];
                        let y2 = x[2 * line_p2 + 1];

                        // Cross product (P - P1) x (P2 - P1) = 0
                        let cross = (px - x1) * (y2 - y1) - (py - y1) * (x2 - x1);
                        residuals.push(cross);
                    }
                }
            }
        }

        residuals
    }

    /// Computes the Jacobian matrix using central finite differences.
    fn calculate_jacobian(
        constraints: &[GeometricConstraint],
        x: &[f64],
        num_points: usize,
    ) -> Vec<Vec<f64>> {
        let h = 1e-6;
        let num_vars = x.len();
        let base_res = Self::calculate_residuals(constraints, x, num_points);
        let num_residuals = base_res.len();

        let mut jacobian = vec![vec![0.0f64; num_vars]; num_residuals];

        for j in 0..num_vars {
            let mut x_plus = x.to_vec();
            let mut x_minus = x.to_vec();

            x_plus[j] += h;
            x_minus[j] -= h;

            let res_plus = Self::calculate_residuals(constraints, &x_plus, num_points);
            let res_minus = Self::calculate_residuals(constraints, &x_minus, num_points);

            for i in 0..num_residuals {
                jacobian[i][j] = (res_plus[i] - res_minus[i]) / (2.0 * h);
            }
        }

        jacobian
    }

    /// Solves $A x = b$ via Gaussian Elimination with Partial Pivoting.
    fn solve_linear_system(a: &[Vec<f64>], b: &[f64]) -> Option<Vec<f64>> {
        let n = b.len();
        if a.len() != n {
            return None;
        }

        let mut aug = vec![vec![0.0f64; n + 1]; n];
        for i in 0..n {
            for j in 0..n {
                aug[i][j] = a[i][j];
            }
            aug[i][n] = b[i];
        }

        for col in 0..n {
            // Find pivot
            let mut max_row = col;
            let mut max_val = aug[col][col].abs();
            for row in (col + 1)..n {
                if aug[row][col].abs() > max_val {
                    max_val = aug[row][col].abs();
                    max_row = row;
                }
            }

            if max_val < 1e-12 {
                return None; // Singular matrix
            }

            aug.swap(col, max_row);

            // Eliminate
            for row in (col + 1)..n {
                let factor = aug[row][col] / aug[col][col];
                for k in col..=n {
                    aug[row][k] -= factor * aug[col][k];
                }
            }
        }

        // Back-substitution
        let mut x = vec![0.0f64; n];
        for i in (0..n).rev() {
            let mut sum = aug[i][n];
            for j in (i + 1)..n {
                sum -= aug[i][j] * x[j];
            }
            x[i] = sum / aug[i][i];
        }

        Some(x)
    }

    /// Vector L2 Euclidean norm.
    fn vector_norm(v: &[f64]) -> f64 {
        v.iter().map(|&val| val * val).sum::<f64>().sqrt()
    }
}
