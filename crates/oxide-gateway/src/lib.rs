use actix_web::{web, App, HttpServer};
use oxide_state::AppState;
use std::sync::Arc;

pub mod routes;

pub async fn run_gateway(
    state: Arc<AppState>,
    host: &str,
    port: u16,
) -> std::io::Result<()> {
    let state_data = web::Data::new(state);
    let bind_addr = format!("{}:{}", host, port);

    tracing::info!("Starting Oxide-Tech Actix-Web Gateway on http://{}", bind_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(state_data.clone())
            .route("/v1/chat/completions", web::post().to(routes::chat_completions))
            .route("/v1/models", web::get().to(routes::list_models))
            .route("/health/ready", web::get().to(routes::health_ready))
    })
    .bind(&bind_addr)?
    .run()
    .await
}
