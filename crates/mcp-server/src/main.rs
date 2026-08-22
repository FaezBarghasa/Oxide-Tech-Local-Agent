use std::path::PathBuf;
use std::sync::Arc;
use tracing::{info, warn, error};
use tracing_subscriber::fmt;

use config_loader::AppConfig;
use rag_pipeline::RagPipeline;
use mcp_server::McpServer;
use rmcp::ServiceExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logging for the MCP server.
    // Standard output is used by the stdio transport, so we MUST log only to stderr.
    fmt()
        .with_writer(std::io::stderr)
        .with_target(true)
        .init();

    info!("Oxide-Tech MCP Server starting...");

    // Load config
    let cfg = AppConfig::load_default().unwrap_or_else(|e| {
        eprintln!("Warning: config load failed ({e}), using default configuration.");
        AppConfig::load_default().expect("Built-in defaults must succeed")
    });

    // Resolve workspace root
    let workspace_root = PathBuf::from("/home/jrad/RustroverProjects/Oxide-Tech-Local-Agent");

    // Connect to local Qdrant/SurrealDB RAG pipeline
    let rag = match RagPipeline::new().await {
        Ok(rp) => {
            info!("Successfully connected to RAG pipeline from MCP Server.");
            Some(Arc::new(rp))
        }
        Err(e) => {
            warn!("RAG pipeline not available from MCP Server ({e}). Qdrant search will be disabled.");
            None
        }
    };

    // Instantiate our MCP server
    let server = McpServer::new(workspace_root, rag);

    // Stdio vs TCP transport
    let transport_mode = cfg.mcp.transport.to_lowercase();
    if transport_mode == "tcp" {
        let port = cfg.mcp.tcp_port;
        let listener = tokio::net::TcpListener::bind(("127.0.0.1", port)).await?;
        info!("MCP Server listening on TCP 127.0.0.1:{}", port);

        loop {
            let (stream, addr) = listener.accept().await?;
            info!("Accepted new TCP client connection from: {}", addr);
            let server_clone = server.clone();

            tokio::spawn(async move {
                match server_clone.serve(stream).await {
                    Ok(service) => {
                        info!("Serving TCP client: {}", addr);
                        if let Err(e) = service.waiting().await {
                            warn!("TCP connection to {} finished with error: {}", addr, e);
                        } else {
                            info!("TCP connection to {} closed cleanly", addr);
                        }
                    }
                    Err(e) => {
                        error!("Failed to serve TCP connection to {}: {}", addr, e);
                    }
                }
            });
        }
    } else {
        info!("MCP Server running in stdio transport mode (ready for JSON-RPC over stdin/stdout)");
        let stdio_transport = (tokio::io::stdin(), tokio::io::stdout());
        let service = server.serve(stdio_transport).await?;
        service.waiting().await?;
        info!("MCP Server shutting down");
    }

    Ok(())
}
