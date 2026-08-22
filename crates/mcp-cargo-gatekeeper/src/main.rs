use std::env;
use std::path::PathBuf;
use mcp_cargo_gatekeeper::CargoGatekeeperServer;
use rmcp::ServiceExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_root = env::var("WORKSPACE_ROOT").ok().map(PathBuf::from).unwrap_or_else(|| env::current_dir().unwrap());
    let server = CargoGatekeeperServer::new(workspace_root);
    let stdio_transport = (tokio::io::stdin(), tokio::io::stdout());
    let service = server.serve(stdio_transport).await?;
    service.waiting().await?;
    Ok(())
}
