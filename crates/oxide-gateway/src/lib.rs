use actix_web::middleware as actix_middleware;
use actix_web::{web, App, HttpServer};
use oxide_state::AppState;
use std::sync::Arc;

pub mod middleware;
pub mod routes;

pub async fn run_gateway(
    state: Arc<AppState>,
    host: &str,
    port: u16,
) -> std::io::Result<()> {
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
                    .route("/chat/completions", web::post().to(routes::chat_completions))
                    .route("/models", web::get().to(routes::list_models)),
            )
    })
    .workers(workers)
    .shutdown_timeout(30)
    .bind(&bind_addr)?
    .run()
    .await
}
