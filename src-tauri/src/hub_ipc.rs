use model_trainer::modelscope::{HubDownloader, HubSource, ModelDownloadRequest};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::info;

#[derive(Clone, Serialize, Deserialize)]
pub struct HubDownloadDto {
    pub source: String, // "modelscope" or "huggingface"
    pub model_id: String,
    pub filename: String,
    pub target_dir: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct HubDownloadResponseDto {
    pub success: bool,
    pub file_path: String,
    pub blake3_hash: String,
    pub message: String,
}

#[tauri::command]
pub async fn hub_download_model(req: HubDownloadDto) -> Result<HubDownloadResponseDto, String> {
    let source = if req.source.to_lowercase() == "modelscope" {
        HubSource::ModelScope
    } else {
        HubSource::HuggingFace
    };

    let target_directory = if let Some(dir) = req.target_dir {
        PathBuf::from(dir)
    } else {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".cache/oxide-models")
    };

    tokio::fs::create_dir_all(&target_directory)
        .await
        .map_err(|e| format!("Failed to create directory {}: {}", target_directory.display(), e))?;

    let dest_file = target_directory.join(&req.filename);
    let download_req = ModelDownloadRequest {
        source,
        model_id: req.model_id.clone(),
        filename: req.filename.clone(),
        target_dir: Some(target_directory.clone()),
    };

    info!(
        "Hub IPC: Starting download of {}/{} from {:?}",
        req.model_id, req.filename, source
    );

    let downloader = HubDownloader::new();
    let hash = downloader
        .download_file(&download_req, &dest_file)
        .await
        .map_err(|e| format!("Download error: {}", e))?;

    info!(
        "Hub IPC: Downloaded to {} (BLAKE3: {})",
        dest_file.display(),
        hash
    );

    Ok(HubDownloadResponseDto {
        success: true,
        file_path: dest_file.display().to_string(),
        blake3_hash: hash,
        message: format!("Successfully downloaded {} from {:?}", req.filename, source),
    })
}
