#![allow(
    clippy::collapsible_if,
    clippy::manual_div_ceil,
    clippy::manual_unwrap_or,
    clippy::manual_unwrap_or_default,
    clippy::match_like_matches_macro,
    clippy::should_implement_trait
)]

pub mod auth;
pub mod h3_gateway;
pub mod routes;
pub mod websocket;

use actix_web::{middleware::Logger, web, App, HttpServer};
use common::config::AppConfig;
use knowledge::KnowledgeClient;
use memory::SurrealClient;
use std::sync::Arc;
use tracing::{error, info, warn};

pub fn ensure_certs() {
    use std::path::Path;
    if !Path::new("cert.pem").exists() || !Path::new("key.pem").exists() {
        info!("TLS certificates not found. Generating self-signed certificates using openssl...");
        let status = std::process::Command::new("openssl")
            .args([
                "req",
                "-x509",
                "-newkey",
                "rsa:2048",
                "-keyout",
                "key.pem",
                "-out",
                "cert.pem",
                "-sha256",
                "-days",
                "365",
                "-nodes",
                "-subj",
                "/CN=localhost",
            ])
            .status();
        match status {
            Ok(s) if s.success() => info!("Self-signed TLS certificates generated successfully."),
            _ => warn!(
                "Failed to generate self-signed TLS certificates. QUIC/HTTP3 gateway might fail to start."
            ),
        }
    }
}

pub async fn run_gateway_server(cfg: AppConfig) -> std::io::Result<()> {
    info!(
        "Oxide-Tech Gateway starting on {}:{}",
        cfg.gateway.host, cfg.gateway.port
    );

    // Ensure self-signed TLS certificates are present
    ensure_certs();

    // Initialize SurrealDB Client
    let surreal_client = match SurrealClient::new().await {
        Ok(c) => {
            info!("SurrealDB memory service initialized.");
            Some(Arc::new(c))
        }
        Err(e) => {
            warn!(
                "SurrealDB failed to initialize: {}. Graph features will be disabled.",
                e
            );
            None
        }
    };

    // Initialize Qdrant Client (Knowledge)
    let knowledge_client = match KnowledgeClient::new().await {
        Ok(c) => {
            info!("Qdrant knowledge service initialized.");
            Some(Arc::new(c))
        }
        Err(e) => {
            warn!(
                "Qdrant knowledge failed to initialize: {}. Vector features will be disabled.",
                e
            );
            None
        }
    };

    // Spawn the QUIC/HTTP3 gateway task
    let h3_config = Arc::new(cfg.clone());
    let h3_surreal = surreal_client.clone();
    let h3_knowledge = knowledge_client.clone();
    tokio::spawn(async move {
        info!("Spawning QUIC/HTTP3 Edge Gateway task...");
        if let Err(e) = h3_gateway::start_h3_gateway(h3_config, h3_surreal, h3_knowledge).await {
            error!("QUIC/HTTP3 Edge Gateway terminated with error: {}", e);
        }
    });

    // Spawn background scheduler if SurrealDB is initialized
    if let Some(ref sc) = surreal_client {
        scheduler::start_scheduler(cfg.clone(), sc.db.clone());
        info!("Background scheduler spawned.");
    }

    let cfg_clone = cfg.clone();
    let memory_clone = surreal_client.clone();
    let knowledge_clone = knowledge_client.clone();
    let ws_broadcaster = Arc::new(websocket::WsBroadcaster::new());

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(cfg_clone.clone()))
            .app_data(web::Data::new(memory_clone.clone()))
            .app_data(web::Data::new(knowledge_clone.clone()))
            .app_data(web::Data::new(ws_broadcaster.clone()))
            // Auth routes
            .route("/api/auth/login", web::post().to(routes::login))
            // Protected Status route
            .route("/api/status", web::get().to(routes::get_status))
            // Core Agent routes
            .route("/api/agent/think", web::post().to(routes::agent_think))
            .route("/api/agent/execute", web::post().to(routes::agent_execute))
            // RAG Query and Indexing routes
            .route("/api/rag/query", web::post().to(routes::rag_query))
            .route("/api/rag/index", web::post().to(routes::rag_index))
            // Cargo Toolchain routes
            .route("/api/cargo/check", web::post().to(routes::handle_cargo_check))
            .route("/api/cargo/clippy", web::post().to(routes::handle_cargo_clippy))
            // KiCad CAD routes
            .route(
                "/api/kicad/load-board",
                web::post().to(routes::handle_kicad_load_board),
            )
            .route(
                "/api/kicad/run-drc",
                web::post().to(routes::handle_kicad_run_drc),
            )
            // SKiDL PCB generation routes
            .route(
                "/api/skidl/generate",
                web::post().to(routes::handle_skidl_generate),
            )
            // Thermal simulation routes
            .route(
                "/api/thermal/simulate",
                web::post().to(routes::handle_thermal_simulate),
            )
            // Repository Structure routes
            .route(
                "/api/repository/structure",
                web::get().to(routes::handle_repository_structure),
            )
            // Manual Knowledge Update route
            .route(
                "/api/knowledge/update",
                web::post().to(routes::update_knowledge),
            )
            // Blog routes
            .route("/blog", web::get().to(routes::blog_index))
            .route("/blog/", web::get().to(routes::blog_index))
            .route("/blog/post/{id}", web::get().to(routes::blog_post))
            .route("/blog/tag/{tag}", web::get().to(routes::blog_tag))
            .route("/blog/feed.xml", web::get().to(routes::blog_feed))
            .route("/blog/api/posts", web::get().to(routes::blog_api_posts))
            // Health routes
            .route("/health/live", web::get().to(routes::health_live))
            .route("/health/ready", web::get().to(routes::health_ready))
            // WebSocket Streaming routes
            .service(websocket::handle_ws_compilation)
            .service(websocket::handle_ws_agent_progress)
    })
    .bind((cfg.gateway.host.as_str(), cfg.gateway.port))?
    .run();

    let server_handle = server.handle();

    tokio::select! {
        res = server => res,
        _ = tokio::signal::ctrl_c() => {
            info!("Received SIGINT (Ctrl+C). Performing graceful shutdown of Oxide Gateway...");
            server_handle.stop(true).await;
            info!("Oxide Gateway shut down cleanly.");
            Ok(())
        }
    }
}
