pub mod codegen;
pub mod constraint_solver;
pub mod dag;
pub mod evaluator;
pub mod patcher;

use thiserror::Error;

/// Core error types for the `parametric-forge` crate.
#[derive(Error, Debug, PartialEq)]
pub enum ParametricError {
    #[error("Node or Feature '{0}' not found in DAG")]
    NodeNotFound(String),

    #[error("Cyclic dependency detected in Feature DAG: {0}")]
    CyclicDependency(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Hierarchy mismatch: {0}")]
    HierarchyMismatch(String),

    #[error("Over-constrained or unsolvable system: {0}")]
    OverConstrainedOrUnsolvable(String),

    #[error("Degenerate geometry: {0}")]
    DegenerateGeometry(String),

    #[error("Geometric invariant violation: {0}")]
    GeometricInvariantViolation(String),
}

pub use codegen::MacroCodegen;
pub use constraint_solver::{ConstraintSolver, GeometricConstraint};
pub use dag::{BooleanType, CadOperation, DependencyEdge, FeatureDAG, Plane, SketchEntity};
pub use evaluator::{DagEvaluator, EvaluatedSolid};
pub use patcher::{DeltaPatch, HierarchyLevel, Mutation};
