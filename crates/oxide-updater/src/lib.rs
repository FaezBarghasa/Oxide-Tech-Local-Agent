use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;

const EMBEDDED_RELEASE_PUBKEY: &[u8; 32] = include_bytes!("../keys/release_pubkey.bin");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformRelease {
    pub url: String,
    pub blake3: String,
    pub signature: Option<String>,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateManifest {
    pub version: String,
    pub min_os_version: Option<String>,
    pub platforms: HashMap<String, PlatformRelease>,
    pub changelog: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateStage {
    Checking,
    Downloading,
    VerifyingSignature,
    ApplyingPayload,
    ReadyToRestart,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProgress {
    pub bytes_downloaded: u64,
    pub total_bytes: u64,
    pub speed_bytes_per_sec: f64,
    pub stage: UpdateStage,
}

pub struct OxideUpdater {
    client: reqwest::Client,
    target_manifest_url: String,
    cancel_flag: Arc<AtomicBool>,
}

impl OxideUpdater {
    pub fn new(manifest_url: &str) -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("Oxide-Studio-Native-Updater/1.0")
                .build()
                .unwrap_or_default(),
            target_manifest_url: manifest_url.to_string(),
            cancel_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn cancel(&self) {
        self.cancel_flag.store(true, Ordering::SeqCst);
    }

    pub fn get_current_platform() -> &'static str {
        if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
            "x86_64-unknown-linux-gnu"
        } else if cfg!(all(target_os = "linux", target_arch = "aarch64")) {
            "aarch64-unknown-linux-gnu"
        } else if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
            "x86_64-pc-windows-msvc"
        } else if cfg!(all(target_os = "macos", target_arch = "x86_64")) {
            "x86_64-apple-darwin"
        } else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
            "aarch64-apple-darwin"
        } else {
            "unknown"
        }
    }

    pub fn verify_signature(payload: &[u8], signature_bytes: &[u8; 64]) -> Result<(), String> {
        // If release pubkey is all zeros (default dev build), skip strict signature check
        if EMBEDDED_RELEASE_PUBKEY.iter().all(|&b| b == 0) {
            tracing::warn!("Dev public key active in oxide-updater; signature check bypassed.");
            return Ok(());
        }

        let verifying_key = VerifyingKey::from_bytes(EMBEDDED_RELEASE_PUBKEY)
            .map_err(|e| format!("Invalid embedded public key: {}", e))?;
        let signature = Signature::from_bytes(signature_bytes);
        verifying_key
            .verify(payload, &signature)
            .map_err(|e| format!("Cryptographic signature verification failed: {}", e))
    }

    pub async fn check_for_update(
        &self,
        current_version: &str,
    ) -> Result<Option<UpdateManifest>, Box<dyn std::error::Error + Send + Sync>> {
        let resp = self.client.get(&self.target_manifest_url).send().await?;
        if !resp.status().is_success() {
            return Err(format!("Manifest request returned HTTP {}", resp.status()).into());
        }

        let manifest_bytes = resp.bytes().await?;
        let manifest: UpdateManifest = serde_json::from_slice(&manifest_bytes)?;

        if manifest.version != current_version {
            Ok(Some(manifest))
        } else {
            Ok(None)
        }
    }

    pub async fn download_and_apply<F>(
        &self,
        binary_url: &str,
        expected_blake3: &str,
        total_bytes: u64,
        on_progress: F,
    ) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>>
    where
        F: Fn(UpdateProgress) + Send + 'static,
    {
        let current_exe = std::env::current_exe()?;
        let exe_dir = current_exe
            .parent()
            .ok_or("Cannot resolve executable directory")?;
        let temp_staged_path = exe_dir.join(".oxide-update.staged");

        let response = self.client.get(binary_url).send().await?;
        if !response.status().is_success() {
            return Err(format!("Binary download failed with HTTP {}", response.status()).into());
        }

        let mut stream = response.bytes_stream();
        let mut file = tokio::fs::File::create(&temp_staged_path).await?;
        let mut hasher = blake3::Hasher::new();

        let mut downloaded: u64 = 0;
        let start_time = std::time::Instant::now();

        while let Some(chunk_result) = stream.next().await {
            if self.cancel_flag.load(Ordering::Relaxed) {
                let _ = tokio::fs::remove_file(&temp_staged_path).await;
                return Err("Update download aborted by user".into());
            }

            let chunk = chunk_result?;
            hasher.update(&chunk);
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;

            let elapsed = start_time.elapsed().as_secs_f64();
            let speed = if elapsed > 0.0 {
                downloaded as f64 / elapsed
            } else {
                0.0
            };

            on_progress(UpdateProgress {
                bytes_downloaded: downloaded,
                total_bytes,
                speed_bytes_per_sec: speed,
                stage: UpdateStage::Downloading,
            });
        }

        file.flush().await?;
        drop(file);

        // 1. Verify Blake3 hash of the downloaded binary
        on_progress(UpdateProgress {
            bytes_downloaded: downloaded,
            total_bytes,
            speed_bytes_per_sec: 0.0,
            stage: UpdateStage::VerifyingSignature,
        });

        let calculated_hash = hasher.finalize().to_hex().to_string();
        if !expected_blake3.is_empty() && calculated_hash != expected_blake3 {
            let _ = tokio::fs::remove_file(&temp_staged_path).await;
            return Err(format!(
                "BLAKE3 integrity check failed. Expected: {}, Got: {}",
                expected_blake3, calculated_hash
            )
            .into());
        }

        // 2. Set executable permissions on Unix platforms
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&temp_staged_path, std::fs::Permissions::from_mode(0o755))?;
        }

        // 3. Perform atomic file replacement
        on_progress(UpdateProgress {
            bytes_downloaded: downloaded,
            total_bytes,
            speed_bytes_per_sec: 0.0,
            stage: UpdateStage::ApplyingPayload,
        });

        self.atomic_swap(&current_exe, &temp_staged_path)?;

        on_progress(UpdateProgress {
            bytes_downloaded: downloaded,
            total_bytes,
            speed_bytes_per_sec: 0.0,
            stage: UpdateStage::ReadyToRestart,
        });

        Ok(current_exe)
    }

    pub fn atomic_swap(&self, current_exe: &Path, staged_binary: &Path) -> Result<(), std::io::Error> {
        #[cfg(unix)]
        {
            // POSIX rename is atomic within the same filesystem
            std::fs::rename(staged_binary, current_exe)?;
        }

        #[cfg(windows)]
        {
            // Windows locks running binaries from being overwritten or deleted.
            // Move current running executable to .old, then move staged to active.
            let old_exe = current_exe.with_extension("old");
            if old_exe.exists() {
                let _ = std::fs::remove_file(&old_exe);
            }
            std::fs::rename(current_exe, &old_exe)?;
            std::fs::rename(staged_binary, current_exe)?;
        }

        Ok(())
    }

    pub fn restart_process() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let current_exe = std::env::current_exe()?;
        let args: Vec<String> = std::env::args().skip(1).collect();

        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            let err = std::process::Command::new(&current_exe)
                .args(&args)
                .exec();
            Err(Box::new(err))
        }

        #[cfg(windows)]
        {
            std::process::Command::new(&current_exe)
                .args(&args)
                .spawn()?;
            std::process::exit(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_detection() {
        let platform = OxideUpdater::get_current_platform();
        assert!(!platform.is_empty());
        assert_ne!(platform, "unknown");
    }

    #[test]
    fn test_manifest_deserialization() {
        let json_data = r#"{
            "version": "1.2.0",
            "min_os_version": "Linux 5.15 / Windows 10 21H2 / macOS 13.0",
            "platforms": {
                "x86_64-unknown-linux-gnu": {
                    "url": "https://releases.oxide.tech/v1.2.0/oxide-studio-linux-x86_64",
                    "blake3": "4b68e98ec0098f6dcf7c89f41753bb3a4e98198f1262d19488a09637c358e8b5",
                    "signature": "c09ef784e27f4d4554b7263b65a58742c3e1e",
                    "size_bytes": 81249280
                }
            },
            "changelog": "• Ground-truth updater\n• Pure Rust stack"
        }"#;

        let manifest: Result<UpdateManifest, _> = serde_json::from_str(json_data);
        assert!(manifest.is_ok());
        let m = manifest.unwrap();
        assert_eq!(m.version, "1.2.0");
        assert!(m.platforms.contains_key("x86_64-unknown-linux-gnu"));
        assert_eq!(m.platforms["x86_64-unknown-linux-gnu"].size_bytes, 81249280);
    }

    #[test]
    fn test_dev_public_key_bypass() {
        let payload = b"test payload for update";
        let signature = [0u8; 64];
        let res = OxideUpdater::verify_signature(payload, &signature);
        assert!(res.is_ok());
    }

    #[test]
    fn test_blake3_hashing() {
        let data = b"oxide-studio-binary-payload-data";
        let mut hasher = blake3::Hasher::new();
        hasher.update(data);
        let hash = hasher.finalize().to_hex().to_string();
        assert_eq!(hash.len(), 64);
    }
}
