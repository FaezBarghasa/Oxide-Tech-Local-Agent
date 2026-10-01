use oxide_updater::{OxideUpdater, PlatformRelease, UpdateProgress};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use tracing::{error, info};

#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateCheckResponseDto {
    pub update_available: bool,
    pub current_version: String,
    pub target_version: Option<String>,
    pub changelog: Option<String>,
    pub platform_release: Option<PlatformRelease>,
    pub min_os_version: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateDownloadRequestDto {
    pub binary_url: String,
    pub expected_blake3: String,
    pub total_bytes: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateDownloadResponseDto {
    pub success: bool,
    pub message: String,
    pub target_path: Option<String>,
}

#[tauri::command]
pub async fn updater_check(manifest_url: Option<String>) -> Result<UpdateCheckResponseDto, String> {
    let url = manifest_url.unwrap_or_else(|| {
        "https://releases.oxide.tech/oxide-update-manifest.json".to_string()
    });
    let updater = OxideUpdater::new(&url);
    let current_version = env!("CARGO_PKG_VERSION");

    match updater.check_for_update(current_version).await {
        Ok(Some(manifest)) => {
            let platform_key = OxideUpdater::get_current_platform();
            let platform_release = manifest.platforms.get(platform_key).cloned();

            Ok(UpdateCheckResponseDto {
                update_available: true,
                current_version: current_version.to_string(),
                target_version: Some(manifest.version),
                changelog: Some(manifest.changelog),
                platform_release,
                min_os_version: manifest.min_os_version,
            })
        }
        Ok(None) => Ok(UpdateCheckResponseDto {
            update_available: false,
            current_version: current_version.to_string(),
            target_version: None,
            changelog: None,
            platform_release: None,
            min_os_version: None,
        }),
        Err(e) => {
            error!("Updater check failed: {}", e);
            Err(format!("Check failed: {}", e))
        }
    }
}

#[tauri::command]
pub async fn updater_download_and_apply(
    app: AppHandle,
    req: UpdateDownloadRequestDto,
) -> Result<UpdateDownloadResponseDto, String> {
    let updater = OxideUpdater::new("");
    info!("Starting atomic update download from {}", req.binary_url);

    let app_clone = app.clone();
    let res = updater
        .download_and_apply(
            &req.binary_url,
            &req.expected_blake3,
            req.total_bytes,
            move |progress: UpdateProgress| {
                let _ = app_clone.emit("oxide-update-progress", progress);
            },
        )
        .await;

    match res {
        Ok(path) => {
            info!("Update staged and atomically applied to {}", path.display());
            Ok(UpdateDownloadResponseDto {
                success: true,
                message: "Update applied successfully. Ready to restart.".to_string(),
                target_path: Some(path.to_string_lossy().to_string()),
            })
        }
        Err(e) => {
            error!("Update download/apply failed: {}", e);
            Err(format!("Update failed: {}", e))
        }
    }
}

#[tauri::command]
pub async fn updater_restart() -> Result<(), String> {
    info!("Triggering atomic updater restart...");
    OxideUpdater::restart_process().map_err(|e| format!("Restart failed: {}", e))
}
