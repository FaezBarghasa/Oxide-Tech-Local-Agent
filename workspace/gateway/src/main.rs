use actix_web::{App, HttpServer, middleware::Logger, web};
use std::sync::Arc;
use tracing::{error, info, warn};
use tracing_subscriber::fmt;

use common::config::AppConfig;
use knowledge::KnowledgeClient;
use memory::SurrealClient;

mod auth;
mod h3_gateway;
mod routes;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn ensure_certs() {
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

fn main() -> std::io::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(16) // Scale to physical core count to prevent SMT (Hyper-Threading) core thrashing
        .thread_name("oxide-core-worker")
        .thread_stack_size(4 * 1024 * 1024) // 4MB stack size for deep recursion in AST parsing
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // Structured JSON tracing to stdout.
            fmt().with_target(true).with_thread_ids(true).init();

            // Load config
            let cfg = AppConfig::load_default().unwrap_or_else(|e| {
                eprintln!(
                    "Warning: config load failed ({:?}), using built-in defaults.",
                    e
                );
                AppConfig::load_default().expect("Built-in defaults must succeed")
            });

            info!(
                "EIOS Gateway starting on {}:{}",
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

            let cfg_data = web::Data::new(cfg.clone());
            let surreal_data = web::Data::new(surreal_client);
            let knowledge_data = web::Data::new(knowledge_client);

            HttpServer::new(move || {
                App::new()
                    .wrap(Logger::default())
                    .app_data(cfg_data.clone())
                    .app_data(surreal_data.clone())
                    .app_data(knowledge_data.clone())
                    // Auth routes
                    .route("/api/auth/login", web::post().to(routes::login))
                    // Agent routes
                    .route("/api/agent/think", web::post().to(routes::think))
                    .route("/api/agent/execute", web::post().to(routes::execute))
                    .route("/api/status", web::get().to(routes::status))
                    // RAG routes
                    .route("/api/rag/query", web::post().to(routes::query_rag))
                    .route("/api/rag/index", web::post().to(routes::index_rag))
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
            })
            .bind((cfg.gateway.host.as_str(), cfg.gateway.port))?
            .run()
            .await
        })
}

