use actix_web::middleware as actix_middleware;
use actix_web::{App, HttpServer, web};
use oxide_core::RuntimeTopology;
use oxide_state::AppState;
use std::sync::Arc;

pub mod middleware;
pub mod mobile_bridge;
pub mod probe;
pub mod quality;
pub mod router;
pub mod routes;

pub use mobile_bridge::{
    AgentControlAction, MobileBridgeManager, MobileSignalMessage, PendingApprovalPayload,
};
pub use quality::QualityGate;
pub use router::{CoderBackend, GatewayRouter};

/// Construct a tuned multi-threaded Tokio runtime with CPU topology awareness and core pinning.
pub fn build_tuned_runtime(
    topology: Option<RuntimeTopology>,
) -> std::io::Result<tokio::runtime::Runtime> {
    let topo = topology.unwrap_or_default();
    let enable_pinning = topo.enable_core_pinning;

    let mut builder = tokio::runtime::Builder::new_multi_thread();
    builder
        .worker_threads(topo.worker_threads)
        .thread_stack_size(topo.stack_size_bytes)
        .thread_name(&topo.thread_prefix)
        .enable_all();

    if enable_pinning {
        builder.on_thread_start(move || {
            let core_id = match std::thread::current().name() {
                Some(name) => name
                    .split('-')
                    .next_back()
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(0),
                None => 0,
            };
            let _ = RuntimeTopology::pin_current_thread_to_core(core_id);
        });
    }

    builder.build()
}

pub async fn run_gateway(state: Arc<AppState>, host: &str, port: u16) -> std::io::Result<()> {
    let state_data = web::Data::new(state.clone());
    let bind_addr = format!("{}:{}", host, port);
    let workers = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    tracing::info!(
        "Starting Oxide-Tech Actix-Web Gateway on http://{} with {} workers",
        bind_addr,
        workers
    );

    HttpServer::new(move || {
        let auth = middleware::ApiKeyAuth {
            security: state.security.clone(),
        };

        App::new()
            .app_data(state_data.clone())
            // Global middleware
            .wrap(actix_middleware::Logger::default())
            .wrap(actix_middleware::Compress::default())
            // Public endpoints
            .route("/health", web::get().to(routes::health_ready))
            .route("/health/ready", web::get().to(routes::health_ready))
            .route("/metrics", web::get().to(routes::metrics))
            // Protected OpenAI-compatible /v1 scope
            .service(
                web::scope("/v1")
                    .wrap(auth)
                    .route(
                        "/chat/completions",
                        web::post().to(routes::chat_completions),
                    )
                    .route("/models", web::get().to(routes::list_models)),
            )
    })
    .workers(workers)
    .shutdown_timeout(30)
    .bind(&bind_addr)?
    .run()
    .await
}
