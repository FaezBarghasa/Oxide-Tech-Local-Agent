use mcp_desktop_tester::DesktopTesterServer;
use rmcp::ServiceExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = DesktopTesterServer::new();
    let stdio_transport = (tokio::io::stdin(), tokio::io::stdout());
    let service = server.serve(stdio_transport).await?;
    service.waiting().await?;
    Ok(())
}
