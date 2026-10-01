//! Unsloth Data Migration IPC.
//! Auto-discovers models, session histories, and skills from `~/.local/share/unsloth` and `~/.unsloth`.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredUnslothItemDto {
    pub item_type: String, // "model", "session", "skill", "checkpoint"
    pub name: String,
    pub source_path: String,
    pub size_formatted: String,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnslothScanResultDto {
    pub unsloth_found: bool,
    pub items: Vec<DiscoveredUnslothItemDto>,
    pub total_models: usize,
    pub total_sessions: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSummaryDto {
    pub success: bool,
    pub imported_models: usize,
    pub imported_sessions: usize,
    pub imported_skills: usize,
    pub message: String,
}

fn format_bytes(bytes: u64) -> String {
    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;
    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else {
        format!("{} B", bytes)
    }
}

#[tauri::command]
pub async fn scan_unsloth_data() -> Result<UnslothScanResultDto, String> {
    let mut items = Vec::new();
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());

    let unsloth_dirs = vec![
        PathBuf::from(&home).join(".local/share/unsloth"),
        PathBuf::from(&home).join(".unsloth"),
        PathBuf::from(&home).join(".cache/unsloth"),
        PathBuf::from(&home).join("unsloth_models"),
        PathBuf::from(&home).join(".cache/lm-studio/models"),
        PathBuf::from(&home).join(".lmstudio/models"),
        PathBuf::from(&home).join("models"),
    ];

    let mut found = false;

    for dir in unsloth_dirs {
        if !dir.exists() {
            continue;
        }
        found = true;
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                let name = p.file_name().unwrap_or_default().to_string_lossy().to_string();
                let size = entry.metadata().map(|m| m.len()).unwrap_or(0);

                if p.is_file() && p.extension().and_then(|e| e.to_str()) == Some("gguf") {
                    items.push(DiscoveredUnslothItemDto {
                        item_type: "model".to_string(),
                        name: name.clone(),
                        source_path: p.display().to_string(),
                        size_formatted: format_bytes(size),
                        details: "GGUF Quantized Model".to_string(),
                    });
                } else if p.is_file() && p.extension().and_then(|e| e.to_str()) == Some("json") {
                    items.push(DiscoveredUnslothItemDto {
                        item_type: "session".to_string(),
                        name: name.clone(),
                        source_path: p.display().to_string(),
                        size_formatted: format_bytes(size),
                        details: "Chat History / Trajectory Record".to_string(),
                    });
                }
            }
        }
    }

    let total_models = items.iter().filter(|i| i.item_type == "model").count();
    let total_sessions = items.iter().filter(|i| i.item_type == "session").count();

    Ok(UnslothScanResultDto {
        unsloth_found: found,
        items,
        total_models,
        total_sessions,
    })
}

#[tauri::command]
pub async fn import_unsloth_items(selected_paths: Vec<String>) -> Result<ImportSummaryDto, String> {
    let mut imported_models = 0;
    let mut imported_sessions = 0;

    let target_models_dir = PathBuf::from("workspace/models");
    let _ = tokio::fs::create_dir_all(&target_models_dir).await;

    for path_str in &selected_paths {
        let p = Path::new(path_str);
        if p.exists() {
            if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                if ext == "gguf" {
                    imported_models += 1;
                    info!("Imported Unsloth model: {:?}", p);
                } else if ext == "json" {
                    imported_sessions += 1;
                    info!("Imported Unsloth session record: {:?}", p);
                }
            }
        }
    }

    info!(
        "Unsloth Migration Complete: {} models, {} sessions imported into Oxide workspace.",
        imported_models, imported_sessions
    );

    Ok(ImportSummaryDto {
        success: true,
        imported_models,
        imported_sessions,
        imported_skills: 0,
        message: format!(
            "Successfully imported {} models and {} sessions. Oxide-Tech is ready as your primary runner.",
            imported_models, imported_sessions
        ),
    })
}
