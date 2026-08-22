use std::env;
use std::path::PathBuf;
use mcp_probe_rs::ProbeRsServer;
use mcp_probe_rs::hitl::HitlGate;
use rmcp::handler::server::run_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Workspace root from env or cwd
    let workspace_root = env::var("WORKSPACE_ROOT").ok().map(PathBuf::from).unwrap_or_else(|| env::current_dir().unwrap());
    // HITL configuration
    let socket_path = env::var("HITL_SOCKET_PATH").unwrap_or_else(|_| "/tmp/oxide-hitl.sock".to_string());
    let timeout_secs: u64 = env::var("HITL_TIMEOUT_SECS").ok().and_then(|s| s.parse().ok()).unwrap_or(120);
    let hitl = HitlGate::new(PathBuf::from(socket_path), timeout_secs);
    // Spawn background listener (fire‑and‑forget)
    let hitl_clone = hitl.clone();
    tokio::spawn(async move { hitl_clone.spawn_listener().await });

    let server = ProbeRsServer::new(workspace_root, hitl);
    run_server(server).await?;
    Ok(())
}
