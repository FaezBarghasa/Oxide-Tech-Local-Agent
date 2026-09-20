use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFileDto {
    pub path: String,
    pub content: String,
}

#[tauri::command]
pub async fn config_read(path: Option<String>) -> Result<ConfigFileDto, String> {
    let target_path = path.unwrap_or_else(|| "config.toml".to_string());
    let p = Path::new(&target_path);
    let content = if p.exists() {
        std::fs::read_to_string(p).map_err(|e| format!("Failed to read {}: {}", target_path, e))?
    } else {
        include_str!("../../config.toml").to_string()
    };

    Ok(ConfigFileDto {
        path: target_path,
        content,
    })
}

#[tauri::command]
pub async fn config_save(path: Option<String>, content: String) -> Result<(), String> {
    let target_path = path.unwrap_or_else(|| "config.toml".to_string());
    // Validate TOML syntax first
    let _: toml::Value = toml::from_str(&content)
        .map_err(|e| format!("Invalid TOML configuration format: {}", e))?;

    std::fs::write(&target_path, content)
        .map_err(|e| format!("Failed to write {}: {}", target_path, e))?;

    Ok(())
}
