use common::error::{EiosError, Result};
use rmcp::model::{CallToolRequestParam, CallToolResult, ClientInfo, ListToolsResult};
use rmcp::service::RunningService;
use rmcp::{ClientHandler, RoleClient, ServiceExt};
use std::process::Stdio;
use tokio::process::Command;
use tracing::info;

pub struct EiosMcpClient {
    pub client: RunningService<RoleClient, LocalClientHandler>,
}

#[derive(Clone, Default)]
pub struct LocalClientHandler;

impl ClientHandler for LocalClientHandler {
    fn get_info(&self) -> ClientInfo {
        let mut info = ClientInfo::default();
        info.client_info.name = "EIOS-MCP-Client".to_string();
        info.client_info.version = "0.1.0".to_string();
        info
    }
}

impl EiosMcpClient {
    /// Spawn a local process (stdio transport) and bind a client to it.
    pub async fn connect_stdio(command: &str, args: &[&str]) -> Result<Self> {
        info!("Connecting to MCP server via stdio: {} {:?}", command, args);

        let mut child = Command::new(command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| EiosError::Internal(format!("Failed to spawn MCP server: {}", e)))?;

        let stdin = child.stdin.take().ok_or_else(|| {
            EiosError::Internal("Failed to open stdin for MCP child process".to_string())
        })?;
        let stdout = child.stdout.take().ok_or_else(|| {
            EiosError::Internal("Failed to open stdout for MCP child process".to_string())
        })?;

        let handler = LocalClientHandler;
        let client = handler
            .serve((stdout, stdin))
            .await
            .map_err(|e| EiosError::Internal(format!("Failed to serve MCP client: {}", e)))?;

        Ok(Self { client })
    }

    /// List all tools exposed by the MCP server.
    pub async fn list_tools(&self) -> Result<ListToolsResult> {
        self.client
            .list_tools(None)
            .await
            .map_err(|e| EiosError::Internal(format!("Failed to list tools: {}", e)))
    }

    /// Call a specific tool with arguments.
    pub async fn call_tool(
        &self,
        name: &str,
        arguments: serde_json::Value,
    ) -> Result<CallToolResult> {
        let obj = arguments
            .as_object()
            .ok_or_else(|| EiosError::Internal("Arguments must be a JSON object".to_string()))?
            .clone();

        let req = CallToolRequestParam {
            name: name.to_string().into(),
            arguments: Some(obj),
        };

        self.client
            .call_tool(req)
            .await
            .map_err(|e| EiosError::Internal(format!("Failed to call tool {}: {}", name, e)))
    }
}

pub mod bridge_proto {
    tonic::include_proto!("oxide.bridge");
}

use bridge_proto::bridge_service_client::BridgeServiceClient;
use tokio::net::UnixStream;
use tonic::transport::{Channel, Endpoint, Uri};
use tower::service_fn;

pub async fn build_uds_bridge_client()
-> std::result::Result<BridgeServiceClient<Channel>, anyhow::Error> {
    let socket_path = "/tmp/oxide_bridge.sock";

    let channel = Endpoint::try_from("http://[::]:50051")?
        .connect_with_connector(service_fn(move |_: Uri| UnixStream::connect(socket_path)))
        .await?;

    Ok(BridgeServiceClient::new(channel))
}
