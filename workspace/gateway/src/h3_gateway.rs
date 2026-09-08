use bytes::{Buf, Bytes, BytesMut};
use http::{Method, Request, Response, StatusCode};
use quinn::Endpoint;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{error, info, warn};

use crate::auth::{Claims, Role, verify_token};
use crate::routes::{
    ExecuteRequest, LoginRequest, LoginResponse, RagIndexRequest, RagQueryRequest, ThinkRequest,
    handle_execute, handle_index_rag, handle_query_rag, handle_status, handle_think,
};
use common::config::AppConfig;
use knowledge::KnowledgeClient;
use memory::SurrealClient;

/// Enforces role requirements for HTTP/3 routes.
fn enforce_role_h3(claims: &Claims, required: Role) -> Result<(), String> {
    match (claims.role, required) {
        (Role::Admin, _) => Ok(()),
        (Role::Developer, Role::Developer | Role::Viewer) => Ok(()),
        (Role::Viewer, Role::Viewer) => Ok(()),
        _ => Err("Insufficient permissions".to_string()),
    }
}

/// Send an HTTP/3 response back to the client.
async fn send_resp<S>(
    mut stream: h3::server::RequestStream<S, Bytes>,
    status: StatusCode,
    body: Bytes,
) -> Result<(), String>
where
    S: h3::quic::BidiStream<Bytes>,
{
    let response = Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(())
        .map_err(|e| e.to_string())?;

    stream
        .send_response(response)
        .await
        .map_err(|e| format!("Send response failed: {}", e))?;

    stream
        .send_data(body)
        .await
        .map_err(|e| format!("Send data failed: {}", e))?;

    stream
        .finish()
        .await
        .map_err(|e| format!("Finish stream failed: {}", e))?;

    Ok(())
}

/// Process a single incoming HTTP/3 request.
async fn process_request<S>(
    req: Request<()>,
    mut stream: h3::server::RequestStream<S, Bytes>,
    config: Arc<AppConfig>,
    memory: Option<Arc<SurrealClient>>,
    knowledge: Option<Arc<KnowledgeClient>>,
) -> Result<(), String>
where
    S: h3::quic::BidiStream<Bytes>,
{
    let path = req.uri().path();
    let method = req.method();

    // Read request body
    let mut body_bytes = BytesMut::new();
    while let Some(mut chunk) = stream
        .recv_data()
        .await
        .map_err(|e| format!("Recv body error: {}", e))?
    {
        while chunk.has_remaining() {
            let slice = chunk.chunk();
            body_bytes.extend_from_slice(slice);
            chunk.advance(slice.len());
        }
    }
    let body = body_bytes.freeze();

    // Check authorization for protected paths
    let is_public = path == "/api/auth/login"
        || path == "/health/live"
        || path == "/health/ready"
        || path == "/metrics";

    let claims = if !is_public {
        let auth_header = req
            .headers()
            .get("authorization")
            .ok_or_else(|| "Missing Authorization header".to_string())?
            .to_str()
            .map_err(|_| "Invalid Authorization header".to_string())?;

        if !auth_header.starts_with("Bearer ") {
            return Err("Authorization scheme must be Bearer".to_string());
        }

        let token = &auth_header[7..];
        verify_token(token, &config.auth.jwt_secret)
            .map_err(|_| "Invalid or expired token".to_string())?
    } else {
        Claims {
            sub: "".to_string(),
            role: Role::Viewer,
            exp: 0,
        }
    };

    match (path, method) {
        ("/api/auth/login", &Method::POST) => {
            let login_req: LoginRequest =
                serde_json::from_slice(&body).map_err(|e| format!("Invalid JSON: {}", e))?;
            let role = Role::from_str(&login_req.role);
            let token = crate::auth::generate_token(
                &login_req.username,
                role,
                &config.auth.jwt_secret,
                config.auth.jwt_expiration_hours,
            )
            .map_err(|e| format!("JWT generation failed: {}", e))?;
            let resp_bytes = serde_json::to_vec(&LoginResponse { token }).unwrap();
            send_resp(stream, StatusCode::OK, resp_bytes.into()).await?;
        }
        ("/api/agent/think", &Method::POST) => {
            enforce_role_h3(&claims, Role::Developer)?;
            let think_req: ThinkRequest =
                serde_json::from_slice(&body).map_err(|e| format!("Invalid JSON: {}", e))?;
            let plan = handle_think(&think_req.prompt, &config).await?;
            let resp_bytes = serde_json::to_vec(&plan).unwrap();
            send_resp(stream, StatusCode::OK, resp_bytes.into()).await?;
        }
        ("/api/agent/execute", &Method::POST) => {
            enforce_role_h3(&claims, Role::Developer)?;
            let exec_req: ExecuteRequest =
                serde_json::from_slice(&body).map_err(|e| format!("Invalid JSON: {}", e))?;
            let res = handle_execute(&exec_req.command, &exec_req.work_dir).await?;
            let resp_bytes = serde_json::to_vec(&res).unwrap();
            send_resp(stream, StatusCode::OK, resp_bytes.into()).await?;
        }
        ("/api/rag/query", &Method::POST) => {
            enforce_role_h3(&claims, Role::Viewer)?;
            let query_req: RagQueryRequest =
                serde_json::from_slice(&body).map_err(|e| format!("Invalid JSON: {}", e))?;
            let chunks = handle_query_rag(
                &query_req.collection,
                &query_req.query,
                query_req.limit,
                &knowledge,
            )
            .await?;
            let resp_bytes = serde_json::to_vec(&chunks).unwrap();
            send_resp(stream, StatusCode::OK, resp_bytes.into()).await?;
        }
        ("/api/rag/index", &Method::POST) => {
            enforce_role_h3(&claims, Role::Developer)?;
            let index_req: RagIndexRequest =
                serde_json::from_slice(&body).map_err(|e| format!("Invalid JSON: {}", e))?;
            let msg =
                handle_index_rag(&index_req.crate_name, &index_req.version, &knowledge).await?;
            send_resp(stream, StatusCode::ACCEPTED, msg.into()).await?;
        }
        ("/api/status", &Method::GET) => {
            enforce_role_h3(&claims, Role::Viewer)?;
            let res = handle_status(&memory, &knowledge).await;
            let resp_bytes = serde_json::to_vec(&res).unwrap();
            send_resp(stream, StatusCode::OK, resp_bytes.into()).await?;
        }
        ("/health/live", &Method::GET) => {
            send_resp(stream, StatusCode::OK, "Liveness probe OK".into()).await?;
        }
        ("/health/ready", &Method::GET) => {
            send_resp(stream, StatusCode::OK, "Readiness probe OK".into()).await?;
        }
        _ => {
            send_resp(stream, StatusCode::NOT_FOUND, "Not Found".into()).await?;
        }
    }

    Ok(())
}

/// Handle a single active QUIC connection.
async fn handle_connection(
    connecting: quinn::Connecting,
    config: Arc<AppConfig>,
    memory: Option<Arc<SurrealClient>>,
    knowledge: Option<Arc<KnowledgeClient>>,
) -> Result<(), anyhow::Error> {
    let conn = connecting.await?;
    let h3_conn = h3_quinn::Connection::new(conn);
    let mut h3_server = h3::server::Connection::new(h3_conn).await?;

    while let Some(resolver) = h3_server.accept().await? {
        let (req, stream) = resolver.resolve_request().await?;
        let config_clone = config.clone();
        let memory_clone = memory.clone();
        let knowledge_clone = knowledge.clone();

        tokio::spawn(async move {
            if let Err(e) =
                process_request(req, stream, config_clone, memory_clone, knowledge_clone).await
            {
                error!("HTTP/3 request handling error: {}", e);
            }
        });
    }

    Ok(())
}

/// Configure the rustls ServerConfig with HTTP/3 ALPN and loaded certificate.
fn configure_rustls() -> Result<rustls::ServerConfig, anyhow::Error> {
    use std::fs::File;
    use std::io::BufReader;

    let _ = rustls::crypto::ring::default_provider().install_default();

    let cert_file = File::open("cert.pem")?;
    let key_file = File::open("key.pem")?;

    let mut cert_reader = BufReader::new(cert_file);
    let mut key_reader = BufReader::new(key_file);

    let certs: Vec<rustls::pki_types::CertificateDer<'static>> =
        rustls_pemfile::certs(&mut cert_reader).collect::<Result<_, _>>()?;

    let key = rustls_pemfile::private_key(&mut key_reader)?
        .ok_or_else(|| anyhow::anyhow!("Private key not found in key.pem"))?;

    let mut server_config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)?;

    server_config.alpn_protocols = vec![b"h3".to_vec()];

    Ok(server_config)
}

/// Start the HTTP/3 gateway on a specific socket address, returning the bound local address and task handle.
pub async fn start_h3_gateway_on_addr(
    bind_addr: SocketAddr,
    config: Arc<AppConfig>,
    memory: Option<Arc<SurrealClient>>,
    knowledge: Option<Arc<KnowledgeClient>>,
) -> Result<(SocketAddr, tokio::task::JoinHandle<()>), anyhow::Error> {
    // Configure QUIC
    let rustls_config = configure_rustls()?;
    let wrapped_config = quinn::crypto::rustls::QuicServerConfig::try_from(rustls_config)?;
    let quic_config = quinn::ServerConfig::with_crypto(Arc::new(wrapped_config));

    // Quinn 0.11 Endpoint setup
    let endpoint = Endpoint::server(quic_config, bind_addr)?;
    let local_addr = endpoint.local_addr()?;
    info!("HTTP/3 Gateway listening on UDP {}", local_addr);

    let handle = tokio::spawn(async move {
        let endpoint = endpoint;
        while let Some(incoming) = endpoint.accept().await {
            let config_clone = config.clone();
            let memory_clone = memory.clone();
            let knowledge_clone = knowledge.clone();

            match incoming.accept() {
                Ok(connecting) => {
                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(
                            connecting,
                            config_clone,
                            memory_clone,
                            knowledge_clone,
                        )
                        .await
                        {
                            warn!("HTTP/3 connection finished with error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    warn!("QUIC incoming connection failed: {}", e);
                }
            }
        }
    });

    Ok((local_addr, handle))
}

/// Start the HTTP/3 gateway background loop.
pub async fn start_h3_gateway(
    config: Arc<AppConfig>,
    memory: Option<Arc<SurrealClient>>,
    knowledge: Option<Arc<KnowledgeClient>>,
) -> Result<(), anyhow::Error> {
    let bind_addr: SocketAddr =
        format!("{}:{}", config.gateway.host, config.gateway.udp_port).parse()?;
    let (_, handle) = start_h3_gateway_on_addr(bind_addr, config, memory, knowledge).await?;
    handle
        .await
        .map_err(|e| anyhow::anyhow!("H3 gateway task failed: {}", e))
}
