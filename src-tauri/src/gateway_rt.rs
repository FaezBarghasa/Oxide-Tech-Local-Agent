//! Embedded gateway runtime.
//!
//! The desktop binary runs the full stack in-process:
//!   - `daemon` subcommand  → gateway in the foreground (systemd: `oxide-agent daemon`)
//!   - desktop window mode   → gateway on a background thread, WebView in front
//!
//! Mirrors `workspace/gateway/src/main.rs` threading and config behaviour.

use common::config::AppConfig;

/// Load config from an explicit path or fall back to built-in defaults.
pub fn load_config(config_path: Option<&str>) -> AppConfig {
    if let Some(path) = config_path {
        match AppConfig::from_file(path) {
            Ok(cfg) => return cfg,
            Err(e) => {
                eprintln!("Warning: config load failed for '{path}' ({e:?}), using built-in defaults.");
            }
        }
    }
    AppConfig::load_default().unwrap_or_else(|e| {
        eprintln!("Warning: config load failed ({e:?}), using built-in defaults.");
        AppConfig::load_default().expect("Built-in defaults must succeed")
    })
}

/// Run the gateway on the calling thread (foreground / headless service mode).
pub fn run_headless(config_path: Option<&str>) -> anyhow::Result<()> {
    let cfg = load_config(config_path);
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(16)
        .thread_name("oxide-gateway-worker")
        .thread_stack_size(4 * 1024 * 1024)
        .enable_all()
        .build()?
        .block_on(async { gateway::run_gateway_server(cfg).await })?;
    Ok(())
}

/// Spawn the gateway on a background thread; returns immediately.
/// The desktop WebView then talks to it at `http://127.0.0.1:8080`.
pub fn spawn_background(config_path: Option<String>) {
    std::thread::Builder::new()
        .name("oxide-gateway".to_string())
        .spawn(move || {
            let cfg = load_config(config_path.as_deref());
            let rt = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(8)
                .thread_name("oxide-gateway-worker")
                .thread_stack_size(4 * 1024 * 1024)
                .enable_all()
                .build();
            match rt {
                Ok(rt) => {
                    if let Err(e) = rt.block_on(async { gateway::run_gateway_server(cfg).await }) {
                        tracing::error!("embedded gateway exited with error: {e:?}");
                    }
                }
                Err(e) => {
                    tracing::error!("failed to build gateway runtime: {e:?}");
                }
            }
        })
        .expect("failed to spawn embedded gateway thread");
}

/// Minimal gateway liveness probe used by `doctor` and the UI status command.
pub async fn probe_gateway(base_url: &str, timeout_secs: u64) -> Result<u16, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(timeout_secs))
        .build()
        .map_err(|e| format!("http client build failed: {e}"))?;
    let url = format!("{base_url}/health/live");
    client
        .get(&url)
        .send()
        .await
        .map(|r| r.status().as_u16())
        .map_err(|e| format!("gateway probe {url} failed: {e}"))
}
