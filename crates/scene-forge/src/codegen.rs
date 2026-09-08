use crate::ipc_bridge::BlenderCommand;

/// Synthesizes verified Blender command traces into compiled Rust functions & macros for `self-evolver`.
#[derive(Debug, Default)]
pub struct MacroCodegen;

impl MacroCodegen {
    /// Generates a reusable Rust declarative macro (`nexus_macro::...`) for instant replay.
    pub fn generate_macro(macro_name: &str, commands: &[BlenderCommand]) -> String {
        let clean_name = macro_name.to_lowercase().replace(['-', ' '], "_");
        let mut code = String::new();

        code.push_str(&format!("/// Crystallized 3D macro for {}\n", macro_name));
        code.push_str("#[macro_export]\n");
        code.push_str(&format!("macro_rules! {} {{\n", clean_name));
        code.push_str("    ($bridge:expr) => {{\n");
        code.push_str("        async {\n");

        for cmd in commands {
            match cmd {
                BlenderCommand::SetActiveObject(id) => {
                    code.push_str(&format!(
                        "            $bridge.execute(BlenderCommand::SetActiveObject(uuid::uuid!(\"{}\"))).await?;\n",
                        id
                    ));
                }
                BlenderCommand::SwitchMode(mode) => {
                    code.push_str(&format!(
                        "            $bridge.execute(BlenderCommand::SwitchMode({:?})).await?;\n",
                        mode
                    ));
                }
                BlenderCommand::CreatePrimitive {
                    primitive_type,
                    dimensions,
                } => {
                    code.push_str(&format!(
                        "            $bridge.execute(BlenderCommand::CreatePrimitive {{ primitive_type: {:?}, dimensions: {:?} }}).await?;\n",
                        primitive_type, dimensions
                    ));
                }
                BlenderCommand::ExtrudeSelection { axis, distance } => {
                    code.push_str(&format!(
                        "            $bridge.execute(BlenderCommand::ExtrudeSelection {{ axis: {:?}, distance: {:.4} }}).await?;\n",
                        axis, distance
                    ));
                }
                BlenderCommand::BevelSelection { offset, segments } => {
                    code.push_str(&format!(
                        "            $bridge.execute(BlenderCommand::BevelSelection {{ offset: {:.4}, segments: {} }}).await?;\n",
                        offset, segments
                    ));
                }
                BlenderCommand::BooleanUnion { target_a, target_b } => {
                    code.push_str(&format!(
                        "            $bridge.execute(BlenderCommand::BooleanUnion {{ target_a: uuid::uuid!(\"{}\"), target_b: uuid::uuid!(\"{}\") }}).await?;\n",
                        target_a, target_b
                    ));
                }
                BlenderCommand::BooleanDifference { target_a, target_b } => {
                    code.push_str(&format!(
                        "            $bridge.execute(BlenderCommand::BooleanDifference {{ target_a: uuid::uuid!(\"{}\"), target_b: uuid::uuid!(\"{}\") }}).await?;\n",
                        target_a, target_b
                    ));
                }
                BlenderCommand::Transform {
                    target,
                    translation,
                    rotation,
                    scale,
                } => {
                    code.push_str(&format!(
                        "            $bridge.execute(BlenderCommand::Transform {{ target: uuid::uuid!(\"{}\"), translation: {:?}, rotation: {:?}, scale: {:?} }}).await?;\n",
                        target, translation, rotation, scale
                    ));
                }
                BlenderCommand::GetMeshData(id) => {
                    code.push_str(&format!(
                        "            $bridge.execute(BlenderCommand::GetMeshData(uuid::uuid!(\"{}\"))).await?;\n",
                        id
                    ));
                }
            }
        }

        code.push_str("            Ok::<(), SceneForgeError>(())\n");
        code.push_str("        }\n");
        code.push_str("    }};\n");
        code.push_str("}\n");

        code
    }
}
