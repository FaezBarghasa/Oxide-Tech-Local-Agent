use std::env;
use std::path::PathBuf;
use mcp_qemu_redox::QemuRedoxServer;
use rmcp::handler::server::run_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_root = env::var("WORKSPACE_ROOT").ok().map(PathBuf::from).unwrap_or_else(|| env::current_dir().unwrap());
    let server = QemuRedoxServer::new(workspace_root);
    run_server(server).await?;
    Ok(())
}
