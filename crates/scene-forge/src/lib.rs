use thiserror::Error;

pub mod codegen;
pub mod ipc_bridge;
pub mod mesh;
pub mod state_machine;
pub mod verifier;

pub use codegen::MacroCodegen;
pub use ipc_bridge::{
    BlenderBridge, BlenderCommand, BlenderResponse, MockBlenderEngine, PrimitiveType,
};
pub use mesh::{MeshData, primitives};
pub use state_machine::{InteractionMode, SceneObject, ShadowSceneGraph};
pub use verifier::{GeometryVerifier, ManifoldReport, TopologyErrorMap};

/// Errors encountered in the 3D scene, binary IPC, and geometric verification pipeline.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SceneForgeError {
    #[error("Shadow scene graph error: {0}")]
    StateError(String),

    #[error("IPC serialization error: {0}")]
    IpcSerialization(String),

    #[error("IPC I/O error: {0}")]
    IpcIo(String),

    #[error("3D object '{0}' not found")]
    ObjectNotFound(String),

    #[error("Geometric verification failed: {0}")]
    VerificationFailed(String),

    #[error("Non-manifold mesh topology: {0}")]
    NonManifoldTopology(String),
}
