use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::ipc_bridge::BlenderCommand;
use crate::mesh::MeshData;

/// Blender 3D viewport interaction mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum InteractionMode {
    #[default]
    Object,
    Edit {
        vertex: bool,
        edge: bool,
        face: bool,
    },
    Sculpt,
    WeightPaint,
    TexturePaint,
}

impl InteractionMode {
    /// Convenience helper for Edit Mode with face selection enabled.
    pub fn edit_faces() -> Self {
        Self::Edit {
            vertex: false,
            edge: false,
            face: true,
        }
    }

    /// Convenience helper for Edit Mode with vertex selection enabled.
    pub fn edit_vertices() -> Self {
        Self::Edit {
            vertex: true,
            edge: false,
            face: false,
        }
    }
}

/// Representation of an object tracked in the Shadow Scene Graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneObject {
    pub id: Uuid,
    pub name: String,
    pub dimensions: [f32; 3],
    pub mesh: Option<MeshData>,
    pub visible: bool,
}

impl SceneObject {
    pub fn new(id: Uuid, name: impl Into<String>, dimensions: [f32; 3]) -> Self {
        Self {
            id,
            name: name.into(),
            dimensions,
            mesh: None,
            visible: true,
        }
    }
}

/// The Shadow Scene Graph state machine mirroring the 3D host environment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShadowSceneGraph {
    pub active_object: Option<Uuid>,
    pub mode: InteractionMode,
    pub objects: HashMap<Uuid, SceneObject>,
}

impl Default for ShadowSceneGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl ShadowSceneGraph {
    pub fn new() -> Self {
        Self {
            active_object: None,
            mode: InteractionMode::Object,
            objects: HashMap::new(),
        }
    }

    /// Prepares and auto-synchronizes the Blender context BEFORE executing an operation.
    /// Returns a list of auto-injected synchronization commands.
    pub fn prepare_context(
        &mut self,
        required_mode: InteractionMode,
        target_obj: Uuid,
    ) -> Vec<BlenderCommand> {
        let mut commands = Vec::new();

        // 1. Auto-correct active object selection if needed
        if self.active_object != Some(target_obj) {
            commands.push(BlenderCommand::SetActiveObject(target_obj));
            self.active_object = Some(target_obj);
        }

        // 2. Auto-correct interaction mode (e.g. switching to Edit Mode before extrude/bevel)
        if self.mode != required_mode {
            commands.push(BlenderCommand::SwitchMode(required_mode));
            self.mode = required_mode;
        }

        commands
    }

    /// Register or track a new object in the shadow graph.
    pub fn register_object(
        &mut self,
        id: Uuid,
        name: impl Into<String>,
        dimensions: [f32; 3],
        mesh: Option<MeshData>,
    ) -> &mut SceneObject {
        let mut obj = SceneObject::new(id, name, dimensions);
        obj.mesh = mesh;
        self.objects.insert(id, obj);
        self.objects.get_mut(&id).expect("Object was just inserted")
    }

    /// Retrieve an object by its UUID.
    pub fn get_object(&self, id: Uuid) -> Option<&SceneObject> {
        self.objects.get(&id)
    }

    /// Retrieve an object mutably by its UUID.
    pub fn get_object_mut(&mut self, id: Uuid) -> Option<&mut SceneObject> {
        self.objects.get_mut(&id)
    }

    /// Apply state transition resulting from a `BlenderCommand`.
    pub fn apply_command(&mut self, cmd: &BlenderCommand) {
        match cmd {
            BlenderCommand::SetActiveObject(id) => {
                self.active_object = Some(*id);
            }
            BlenderCommand::SwitchMode(mode) => {
                self.mode = *mode;
            }
            BlenderCommand::CreatePrimitive {
                primitive_type: _,
                dimensions,
            } => {
                let id = Uuid::new_v4();
                self.register_object(id, "Primitive", *dimensions, None);
                self.active_object = Some(id);
            }
            _ => {}
        }
    }
}
