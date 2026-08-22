use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use tracing::{info, warn, error};
use tracing_subscriber::fmt;

use config_loader::AppConfig;
use rag_pipeline::RagPipeline;

mod routes;
mod websocket;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

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
            fmt()
                .json()
                .with_target(true)
                .with_thread_ids(true)
                .init();

            // Load config — falls back to hard-coded defaults if config.toml is absent.
            let cfg = AppConfig::load_default().unwrap_or_else(|e| {
                eprintln!("Warning: config load failed ({e}), using built-in defaults.");
                AppConfig::load_default().expect("Built-in defaults must always succeed")
            });

            info!(
                thinker_model = %cfg.thinker.model,
                online_primary = %cfg.coder.online.primary.model,
                local_coder = %cfg.coder.local.model,
                "Oxide-Tech Local Agent OS starting"
            );

            // Initialize RAG pipeline
            let rag_pipeline = match RagPipeline::new().await {
                Ok(p) => {
                    info!("RAG pipeline initialized successfully.");
                    let p_arc = Arc::new(p);
                    
                    // Handle auto-update on startup in a background thread
                    if cfg.rag.auto_update_on_startup {
                        let p_clone = p_arc.clone();
                        let watchlist = cfg.rag.watchlist.clone();
                        tokio::spawn(async move {
                            if let Err(e) = rag_pipeline::updater::check_and_update_crates(&watchlist, &p_clone).await {
                                error!("RAG startup auto-update failed: {}", e);
                            }
                        });
                    }
                    Some(p_arc)
                }
                Err(e) => {
                    warn!("Qdrant/RAG pipeline failed to initialize ({e}). RAG features will be disabled.");
                    None
                }
            };

            let cfg_data = web::Data::new(cfg);
            let broadcaster = Arc::new(websocket::WsBroadcaster::new());
            let rag_data = web::Data::new(rag_pipeline);

            HttpServer::new(move || {
                App::new()
                    .app_data(cfg_data.clone())
                    .app_data(web::Data::new(broadcaster.clone()))
                    .app_data(rag_data.clone())
                    .service(routes::cargo::handle_cargo_check)
                    .service(routes::cargo::handle_cargo_clippy)
                    .service(routes::tree_sitter::handle_tree_sitter_parse)
                    .service(routes::kicad::handle_kicad_load_board)
                    .service(routes::kicad::handle_kicad_run_drc)
                    .service(routes::skidl::handle_skidl_generate)
                    .service(routes::thermal::handle_thermal_simulate)
                    .service(routes::agent::handle_agent_generate)
                    .service(routes::agent::handle_rag_update)
                    .service(routes::agent::handle_status)
                    .service(routes::health::liveness_probe)
                    .service(routes::health::readiness_probe)
                    .service(routes::health::metrics_endpoint)
                    .service(routes::repository::handle_repository_structure)
                    .service(websocket::handle_ws_compilation)
                    .service(websocket::handle_ws_agent_progress)
            })
            .bind(("127.0.0.1", 8080))?
            .run()
            .await
        })
}

