use std::path::PathBuf;
use std::sync::Arc;
use tracing::{error, info, warn};
use tracing_subscriber::fmt;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use common::config::AppConfig;
use knowledge::KnowledgeClient;
use mcp_server::McpServer;
use memory::SurrealClient;
use rmcp::ServiceExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Standard output is used by the stdio transport, so we MUST log only to stderr.
    fmt().with_writer(std::io::stderr).with_target(true).init();

    info!("Oxide-Tech Unified MCP Server starting...");

    // Load config
    let cfg = AppConfig::load_default().unwrap_or_else(|e| {
        eprintln!(
            "Warning: config load failed ({:?}), using built-in defaults.",
            e
        );
        AppConfig::load_default().expect("Built-in defaults must succeed")
    });

    let workspace_root = PathBuf::from("/home/jrad/RustroverProjects/Oxide-Tech-Local-Agent");

    // Initialize SurrealDB Client
    let surreal = match SurrealClient::new().await {
        Ok(c) => {
            info!("SurrealDB client successfully initialized for MCP server.");
            Some(Arc::new(c))
        }
        Err(e) => {
            warn!(
                "SurrealDB client failed to initialize for MCP: {}. Memory tools will be limited.",
                e
            );
            None
        }
    };

    // Initialize Qdrant Client (Knowledge)
    let knowledge = match KnowledgeClient::new().await {
        Ok(c) => {
            info!("Qdrant client successfully initialized for MCP server.");
            Some(Arc::new(c))
        }
        Err(e) => {
            warn!(
                "Qdrant client failed to initialize for MCP: {}. Knowledge retrieval will be limited.",
                e
            );
            None
        }
    };

    // Instantiate our MCP server
    let server = McpServer::new(workspace_root, knowledge, surreal);

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
