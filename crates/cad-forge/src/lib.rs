pub mod dsl;
pub mod kernel;
pub mod sketch;
pub mod verifier;
pub mod voxelizer;

pub use dsl::{BoolOp, CadBuilder, CadScript, Primitive};
pub use kernel::{
    Axis, Body, GeometryKernel, MeasureQuery, Mesh, OcctBackend, Point2D, PolyBackend, Sketch,
};
pub use sketch::{Constraint2D, SketchConstraintSolver};
pub use verifier::{VoxelDiffMap, compute_iou};
pub use voxelizer::VoxelGrid;
