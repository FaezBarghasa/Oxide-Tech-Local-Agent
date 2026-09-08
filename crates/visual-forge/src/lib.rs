pub mod codegen;
pub mod optimizer;
pub mod sdf_feedback;
pub mod shadow_kernel;

use thiserror::Error;

pub use codegen::VisualMatchCodegen;
pub use optimizer::VisualFeedbackOptimizer;
pub use sdf_feedback::{GeometryCorrectionReport, SDFVolume};
pub use shadow_kernel::{KernelError, ShadowKernel, SolidRepresentation};

/// Core error types for the `visual-forge` crate.
#[derive(Error, Debug, PartialEq)]
pub enum VisualForgeError {
    #[error("Shadow Kernel error: {0}")]
    KernelError(#[from] KernelError),

    #[error("Volumetric tolerance not met: {0}")]
    ToleranceNotMet(String),

    #[error("Incompatible volume dimensions or bounds: {0}")]
    IncompatibleDimensions(String),

    #[error("Serialization / Deserialization error: {0}")]
    SerializationError(String),
}
