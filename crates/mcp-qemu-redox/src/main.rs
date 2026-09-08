use mcp_qemu_redox::QemuRedoxServer;
use rmcp::ServiceExt;
use std::env;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_root = env::var("WORKSPACE_ROOT")
        .ok()
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().unwrap());
    let server = QemuRedoxServer::new(workspace_root);
    let stdio_transport = (tokio::io::stdin(), tokio::io::stdout());
    let service = server.serve(stdio_transport).await?;
    service.waiting().await?;
    Ok(())
}
