use std::env;
use std::path::PathBuf;
use mcp_cargo_gatekeeper::CargoGatekeeperServer;
use rmcp::handler::server::run_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Workspace root is the current directory (or can be overridden via env var)
    let workspace_root = env::var("WORKSPACE_ROOT").ok().map(PathBuf::from).unwrap_or_else(|| env::current_dir().unwrap());
    let server = CargoGatekeeperServer::new(workspace_root);
    // Run MCP server on stdio (default) – you can also configure TCP via env vars if needed
    run_server(server).await?;
    Ok(())
}
