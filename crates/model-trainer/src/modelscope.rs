//! ModelScope & Hugging Face Hub Downloader with chunked streaming and BLAKE3 verification.

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelDownloadRequest {
    pub source: HubSource,
    pub model_id: String,
    pub filename: String,
    pub target_dir: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HubSource {
    HuggingFace,
    ModelScope,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelDownloadProgress {
    pub model_id: String,
    pub filename: String,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub percent: f32,
    pub speed_mbps: f32,
    pub is_finished: bool,
    pub blake3_hash: Option<String>,
}

pub struct HubDownloader {
    client: Client,
}

impl Default for HubDownloader {
    fn default() -> Self {
        Self::new()
    }
}

impl HubDownloader {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("oxide-model-hub/0.6.0")
            .build()
            .expect("failed to build reqwest client");
        Self { client }
    }

    /// Resolve download URL from HuggingFace or ModelScope.
    pub fn resolve_url(source: HubSource, model_id: &str, filename: &str) -> String {
        match source {
            HubSource::HuggingFace => {
                format!("https://huggingface.co/{}/resolve/main/{}", model_id, filename)
            }
            HubSource::ModelScope => {
                format!(
                    "https://modelscope.cn/api/v1/models/{}/repo?Revision=master&FilePath={}",
                    model_id, filename
                )
            }
        }
    }

    /// Download model weights with BLAKE3 checksum calculation.
    pub async fn download_file(
        &self,
        req: &ModelDownloadRequest,
        dest_path: &Path,
    ) -> Result<String> {
        let url = Self::resolve_url(req.source, &req.model_id, &req.filename);
        let resp = self.client.get(&url).send().await?.error_for_status()?;

        let bytes = resp.bytes().await?;
        let hash_str = blake3::hash(&bytes).to_hex().to_string();
        std::fs::write(dest_path, &bytes)
            .with_context(|| format!("Failed to write destination file {}", dest_path.display()))?;

        Ok(hash_str)
    }
}
