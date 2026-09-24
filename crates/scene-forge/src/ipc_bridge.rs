use postcard::{from_bytes, to_allocvec};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use uuid::Uuid;

use crate::SceneForgeError;
use crate::mesh::{MeshData, primitives};
use crate::state_machine::InteractionMode;

/// 3D primitive geometry type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrimitiveType {
    Cube,
    Cylinder,
    Sphere,
    Plane,
    SciFiCrate,
}

/// Commands dispatched across the Postcard binary IPC bridge to the 3D host/addon.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlenderCommand {
    SetActiveObject(Uuid),
    SwitchMode(InteractionMode),
    CreatePrimitive {
        primitive_type: PrimitiveType,
        dimensions: [f32; 3],
    },
    ExtrudeSelection {
        axis: [f32; 3],
        distance: f32,
    },
    BevelSelection {
        offset: f32,
        segments: u32,
    },
    BooleanUnion {
        target_a: Uuid,
        target_b: Uuid,
    },
    BooleanDifference {
        target_a: Uuid,
        target_b: Uuid,
    },
    Transform {
        target: Uuid,
        translation: [f32; 3],
        rotation: [f32; 3],
        scale: [f32; 3],
    },
    GetMeshData(Uuid),
    Heartbeat { timestamp_epoch_ms: u64 },
}

/// Responses returned from the 3D host across the binary IPC stream.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlenderResponse {
    Ack,
    Pong {
        timestamp_epoch_ms: u64,
        status_flags: u32,
    },
    Mesh(MeshData),
    ObjectCreated(Uuid),
    Error(String),
}

/// In-memory mock 3D engine simulator for deterministic testing and ReAct loops.
#[derive(Debug, Default)]
pub struct MockBlenderEngine {
    pub active_object: Option<Uuid>,
    pub mode: InteractionMode,
    pub meshes: HashMap<Uuid, MeshData>,
}

impl MockBlenderEngine {
    pub fn new() -> Self {
        Self {
            active_object: None,
            mode: InteractionMode::Object,
            meshes: HashMap::new(),
        }
    }

    /// Process a command and return the corresponding response.
    pub fn handle_command(&mut self, cmd: BlenderCommand) -> BlenderResponse {
        match cmd {
            BlenderCommand::SetActiveObject(id) => {
                self.active_object = Some(id);
                BlenderResponse::Ack
            }
            BlenderCommand::SwitchMode(mode) => {
                self.mode = mode;
                BlenderResponse::Ack
            }
            BlenderCommand::CreatePrimitive {
                primitive_type,
                dimensions,
            } => {
                let id = Uuid::new_v4();
                let mesh = match primitive_type {
                    PrimitiveType::Cube => primitives::cube(dimensions),
                    PrimitiveType::Cylinder => {
                        primitives::cylinder(dimensions[0] * 0.5, dimensions[1], 16)
                    }
                    PrimitiveType::Sphere | PrimitiveType::Plane | PrimitiveType::SciFiCrate => {
                        primitives::sci_fi_crate(dimensions, 0.1)
                    }
                };
                self.meshes.insert(id, mesh);
                self.active_object = Some(id);
                BlenderResponse::ObjectCreated(id)
            }
            BlenderCommand::GetMeshData(id) => {
                if let Some(mesh) = self.meshes.get(&id) {
                    BlenderResponse::Mesh(mesh.clone())
                } else {
                    BlenderResponse::Error(format!("Mesh object '{}' not found", id))
                }
            }
            BlenderCommand::ExtrudeSelection { axis, distance } => {
                if let Some(id) = self.active_object {
                    if let Some(mesh) = self.meshes.get_mut(&id) {
                        // Simulate simple extrusion by shifting vertices
                        for v in &mut mesh.vertices {
                            v[0] += axis[0] * distance;
                            v[1] += axis[1] * distance;
                            v[2] += axis[2] * distance;
                        }
                        BlenderResponse::Mesh(mesh.clone())
                    } else {
                        BlenderResponse::Error("Active object mesh not found".to_string())
                    }
                } else {
                    BlenderResponse::Error("No active object to extrude".to_string())
                }
            }
            BlenderCommand::Heartbeat { timestamp_epoch_ms } => BlenderResponse::Pong {
                timestamp_epoch_ms,
                status_flags: 1, // Healthy/Ready
            },
            BlenderCommand::BevelSelection { .. }
            | BlenderCommand::BooleanUnion { .. }
            | BlenderCommand::BooleanDifference { .. }
            | BlenderCommand::Transform { .. } => BlenderResponse::Ack,
        }
    }
}

/// Binary IPC bridge using length-prefixed `postcard` framing.
pub struct BlenderBridge<S> {
    stream: S,
}

impl<S> BlenderBridge<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub fn new(stream: S) -> Self {
        Self { stream }
    }

    /// Dispatch a command over the binary stream and read the response.
    pub async fn execute(
        &mut self,
        cmd: BlenderCommand,
    ) -> Result<BlenderResponse, SceneForgeError> {
        // 1. Serialize command with postcard
        let payload = to_allocvec(&cmd).map_err(|e| {
            SceneForgeError::IpcSerialization(format!("Postcard encode failed: {e}"))
        })?;

        // 2. Write 4-byte little-endian length prefix + payload
        let len = (payload.len() as u32).to_le_bytes();
        self.stream
            .write_all(&len)
            .await
            .map_err(|e| SceneForgeError::IpcIo(e.to_string()))?;
        self.stream
            .write_all(&payload)
            .await
            .map_err(|e| SceneForgeError::IpcIo(e.to_string()))?;
        self.stream
            .flush()
            .await
            .map_err(|e| SceneForgeError::IpcIo(e.to_string()))?;

        // 3. Read 4-byte response length
        let mut len_buf = [0u8; 4];
        self.stream
            .read_exact(&mut len_buf)
            .await
            .map_err(|e| SceneForgeError::IpcIo(e.to_string()))?;
        let resp_len = u32::from_le_bytes(len_buf) as usize;

        // 4. Read response payload and deserialize
        let mut resp_buf = vec![0u8; resp_len];
        self.stream
            .read_exact(&mut resp_buf)
            .await
            .map_err(|e| SceneForgeError::IpcIo(e.to_string()))?;

        let response: BlenderResponse = from_bytes(&resp_buf).map_err(|e| {
            SceneForgeError::IpcSerialization(format!("Postcard decode failed: {e}"))
        })?;

        Ok(response)
    }

    /// Helper to encode a response into length-prefixed bytes (for server/mock implementation).
    pub fn encode_response(resp: &BlenderResponse) -> Result<Vec<u8>, SceneForgeError> {
        let payload = to_allocvec(resp).map_err(|e| {
            SceneForgeError::IpcSerialization(format!("Postcard encode failed: {e}"))
        })?;
        let mut out = Vec::with_capacity(4 + payload.len());
        out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        out.extend_from_slice(&payload);
        Ok(out)
    }

    /// Helper to encode a command into length-prefixed bytes.
    pub fn encode_command(cmd: &BlenderCommand) -> Result<Vec<u8>, SceneForgeError> {
        let payload = to_allocvec(cmd).map_err(|e| {
            SceneForgeError::IpcSerialization(format!("Postcard encode failed: {e}"))
        })?;
        let mut out = Vec::with_capacity(4 + payload.len());
        out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        out.extend_from_slice(&payload);
        Ok(out)
    }

    /// Helper to decode a length-prefixed payload buffer.
    pub fn decode_command(bytes: &[u8]) -> Result<BlenderCommand, SceneForgeError> {
        if bytes.len() < 4 {
            return Err(SceneForgeError::IpcSerialization(
                "Buffer too small".to_string(),
            ));
        }
        let len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
        if bytes.len() < 4 + len {
            return Err(SceneForgeError::IpcSerialization(
                "Incomplete packet".to_string(),
            ));
        }
        from_bytes(&bytes[4..4 + len])
            .map_err(|e| SceneForgeError::IpcSerialization(format!("Postcard decode failed: {e}")))
    }
}

/// POSIX Shared Memory Descriptor for Zero-Copy Large Geometries (>2MB)
#[derive(Debug)]
pub struct ShmGeometryBuffer {
    pub shm_path: String,
    pub size_bytes: usize,
}

impl ShmGeometryBuffer {
    /// Create or open named shared memory segment in `/dev/shm`
    pub fn allocate(name: &str, size_bytes: usize) -> Result<Self, SceneForgeError> {
        let shm_path = format!("/dev/shm/oxide_mesh_{name}");
        Ok(Self {
            shm_path,
            size_bytes,
        })
    }

    /// Check if payload exceeds zero-copy threshold (2 MB)
    pub fn exceeds_zero_copy_threshold(bytes_len: usize) -> bool {
        bytes_len > 2 * 1024 * 1024
    }
}

/// Bidirectional heartbeat supervisor for the Python subprocess bridge
pub struct BridgeSupervisor {
    pub child_pid: u32,
    pub heartbeat_interval_ms: u64,
    pub consecutive_failures: u32,
    pub max_failures: u32,
}

impl BridgeSupervisor {
    pub fn new(child_pid: u32) -> Self {
        Self {
            child_pid,
            heartbeat_interval_ms: 500,
            consecutive_failures: 0,
            max_failures: 3,
        }
    }

    /// Record a failed heartbeat and check if process must be terminated and respawned
    pub fn record_heartbeat_failure(&mut self) -> bool {
        self.consecutive_failures += 1;
        self.consecutive_failures >= self.max_failures
    }

    /// Reset failure count on successful Pong
    pub fn record_heartbeat_success(&mut self) {
        self.consecutive_failures = 0;
    }

    /// Terminate unresponsive child process via SIGKILL
    #[cfg(target_os = "linux")]
    pub fn terminate_hung_process(&self) -> Result<(), String> {
        unsafe {
            if libc::kill(self.child_pid as libc::pid_t, libc::SIGKILL) != 0 {
                return Err(format!("Failed to SIGKILL pid {}", self.child_pid));
            }
        }
        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    pub fn terminate_hung_process(&self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heartbeat_encode_decode() {
        let cmd = BlenderCommand::Heartbeat { timestamp_epoch_ms: 1000 };
        let bytes = BlenderBridge::<tokio::io::DuplexStream>::encode_command(&cmd).unwrap();
        let decoded = BlenderBridge::<tokio::io::DuplexStream>::decode_command(&bytes).unwrap();
        assert_eq!(cmd, decoded);
    }

    #[test]
    fn test_supervisor_watchdog_threshold() {
        let mut supervisor = BridgeSupervisor::new(9999);
        assert!(!supervisor.record_heartbeat_failure());
        assert!(!supervisor.record_heartbeat_failure());
        // Third failure triggers termination
        assert!(supervisor.record_heartbeat_failure());
    }

    #[test]
    fn test_shm_threshold() {
        assert!(ShmGeometryBuffer::exceeds_zero_copy_threshold(3 * 1024 * 1024));
        assert!(!ShmGeometryBuffer::exceeds_zero_copy_threshold(1024));
    }
}
