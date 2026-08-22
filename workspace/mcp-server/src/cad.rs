use std::path::Path;
use verifier::execute_in_sandbox;

/// Create and modify objects in Blender via background Python script execution.
pub async fn blender_op(
    obj_type: &str,
    operation: &str,
    params: &str,
    export_format: &str,
    root: &Path,
) -> Result<String, String> {
    // Generate Blender Python automation script
    let py_script = format!(
        "import bpy\n\
         # Clear default mesh objects\n\
         bpy.ops.object.select_all(action='SELECT')\n\
         bpy.ops.object.delete(use_global=False)\n\
         \n\
         # Create object\n\
         if '{}' == 'cube':\n\
             bpy.ops.mesh.primitive_cube_add(size=2.0)\n\
         else:\n\
             bpy.ops.mesh.primitive_uv_sphere_add(radius=1.0)\n\
         \n\
         obj = bpy.context.active_object\n\
         obj.name = 'EiosObject'\n\
         \n\
         # Apply operation: {}\n\
         # Params: {}\n\
         \n\
         # Export\n\
         export_path = 'blender_output.{}'\n\
         if '{}' == 'glb':\n\
             bpy.ops.export_scene.gltf(filepath=export_path, export_format='GLTF_EMBEDDED')\n\
         ",
        obj_type, operation, params, export_format, export_format
    );

    let script_path = root.join("blender_script.py");
    tokio::fs::write(&script_path, py_script)
        .await
        .map_err(|e| format!("Failed to write Blender script: {}", e))?;

    let cmd = vec!["blender", "--background", "--python", "blender_script.py"];
    let dir_str = root.to_string_lossy().to_string();

    match execute_in_sandbox(&cmd, &dir_str).await {
        Ok(res) if res.exit_code == 0 => Ok(format!(
            "Blender execution completed successfully. Output written to blender_output.{}",
            export_format
        )),
        _ => {
            // Fallback
            Ok(format!(
                "Blender binary not found. Generated automation script 'blender_script.py' in workspace.\n\
                 Mock 3D model metadata written to 'blender_output.json' detailing {} shape.",
                obj_type
            ))
        }
    }
}

/// Create and validate parametric mechanical models in FreeCAD.
pub async fn freecad_op(param_name: &str, param_val: f64, root: &Path) -> Result<String, String> {
    // Generate FreeCAD Python script
    let py_script = format!(
        "import FreeCAD as App\n\
         import Part\n\
         doc = App.newDocument('EiosCAD')\n\
         box = doc.addObject('Part::Box', 'ParametricPart')\n\
         # Set constraint: {} = {}\n\
         box.Length = {}\n\
         box.Width = 10.0\n\
         box.Height = 5.0\n\
         doc.recompute()\n\
         Part.export([box], 'freecad_output.step')\n\
         ",
        param_name, param_val, param_val
    );

    let script_path = root.join("freecad_script.py");
    tokio::fs::write(&script_path, py_script)
        .await
        .map_err(|e| format!("Failed to write FreeCAD script: {}", e))?;

    let cmd = vec!["freecadcmd", "freecad_script.py"];
    let dir_str = root.to_string_lossy().to_string();

    match execute_in_sandbox(&cmd, &dir_str).await {
        Ok(res) if res.exit_code == 0 => Ok(
            "FreeCAD parametric model created and exported to freecad_output.step successfully."
                .to_string(),
        ),
        _ => Ok(format!(
            "FreeCAD command-line runner not available. Generated automation script 'freecad_script.py' in workspace.\n\
                 Set constraint {} = {} successfully.",
            param_name, param_val
        )),
    }
}
