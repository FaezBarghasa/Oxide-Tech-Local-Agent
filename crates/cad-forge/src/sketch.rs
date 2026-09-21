use crate::kernel::{Point2D, Sketch};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Constraint2D {
    Distance {
        p1_idx: usize,
        p2_idx: usize,
        distance_mm: f64,
    },
    Horizontal {
        p1_idx: usize,
        p2_idx: usize,
    },
    Vertical {
        p1_idx: usize,
        p2_idx: usize,
    },
    Coincident {
        p1_idx: usize,
        p2_idx: usize,
    },
}

pub struct SketchConstraintSolver {
    pub max_iterations: usize,
    pub tolerance: f64,
}

impl Default for SketchConstraintSolver {
    fn default() -> Self {
        Self {
            max_iterations: 100,
            tolerance: 1e-4,
        }
    }
}

impl SketchConstraintSolver {
    /// Solve 2D geometric constraints using iterative relaxation (Newton-Raphson step)
    pub fn solve(&self, sketch: &mut Sketch, constraints: &[Constraint2D]) -> bool {
        for _ in 0..self.max_iterations {
            let mut max_residual = 0.0;

            for c in constraints {
                match *c {
                    Constraint2D::Distance {
                        p1_idx,
                        p2_idx,
                        distance_mm,
                    } => {
                        if p1_idx < sketch.points.len() && p2_idx < sketch.points.len() {
                            let dx = sketch.points[p2_idx].x - sketch.points[p1_idx].x;
                            let dy = sketch.points[p2_idx].y - sketch.points[p1_idx].y;
                            let current_dist = (dx * dx + dy * dy).sqrt();
                            let residual = (current_dist - distance_mm).abs();
                            if residual > max_residual {
                                max_residual = residual;
                            }

                            if current_dist > 1e-6 {
                                let factor = (distance_mm - current_dist) / (2.0 * current_dist);
                                sketch.points[p1_idx].x -= dx * factor * 0.5;
                                sketch.points[p1_idx].y -= dy * factor * 0.5;
                                sketch.points[p2_idx].x += dx * factor * 0.5;
                                sketch.points[p2_idx].y += dy * factor * 0.5;
                            }
                        }
                    }
                    Constraint2D::Horizontal { p1_idx, p2_idx } => {
                        if p1_idx < sketch.points.len() && p2_idx < sketch.points.len() {
                            let avg_y = (sketch.points[p1_idx].y + sketch.points[p2_idx].y) / 2.0;
                            let residual =
                                (sketch.points[p1_idx].y - sketch.points[p2_idx].y).abs();
                            if residual > max_residual {
                                max_residual = residual;
                            }
                            sketch.points[p1_idx].y = avg_y;
                            sketch.points[p2_idx].y = avg_y;
                        }
                    }
                    Constraint2D::Vertical { p1_idx, p2_idx } => {
                        if p1_idx < sketch.points.len() && p2_idx < sketch.points.len() {
                            let avg_x = (sketch.points[p1_idx].x + sketch.points[p2_idx].x) / 2.0;
                            let residual =
                                (sketch.points[p1_idx].x - sketch.points[p2_idx].x).abs();
                            if residual > max_residual {
                                max_residual = residual;
                            }
                            sketch.points[p1_idx].x = avg_x;
                            sketch.points[p2_idx].x = avg_x;
                        }
                    }
                    Constraint2D::Coincident { p1_idx, p2_idx } => {
                        if p1_idx < sketch.points.len() && p2_idx < sketch.points.len() {
                            let avg_x = (sketch.points[p1_idx].x + sketch.points[p2_idx].x) / 2.0;
                            let avg_y = (sketch.points[p1_idx].y + sketch.points[p2_idx].y) / 2.0;
                            sketch.points[p1_idx] = Point2D { x: avg_x, y: avg_y };
                            sketch.points[p2_idx] = Point2D { x: avg_x, y: avg_y };
                        }
                    }
                }
            }

            if max_residual <= self.tolerance {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sketch_constraint_solver() {
        let solver = SketchConstraintSolver::default();
        let mut sketch = Sketch {
            name: "line".to_string(),
            points: vec![Point2D { x: 0.0, y: 0.0 }, Point2D { x: 8.0, y: 0.5 }],
            closed: false,
        };

        let constraints = vec![
            Constraint2D::Horizontal {
                p1_idx: 0,
                p2_idx: 1,
            },
            Constraint2D::Distance {
                p1_idx: 0,
                p2_idx: 1,
                distance_mm: 10.0,
            },
        ];

        let converged = solver.solve(&mut sketch, &constraints);
        assert!(converged);
        assert!((sketch.points[0].y - sketch.points[1].y).abs() < 1e-3);
        let dist = (sketch.points[1].x - sketch.points[0].x).abs();
        assert!((dist - 10.0).abs() < 1e-3);
    }
}
