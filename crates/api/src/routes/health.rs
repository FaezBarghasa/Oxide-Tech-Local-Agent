use actix_web::{get, HttpResponse, Responder};
use surrealdb_service::client::SurrealClient;
use qdrant_service::client::QdrantServiceClient;
use serde_json::json;

#[get("/health/live")]
pub async fn liveness_probe() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "alive",
        "timestamp": chrono::Utc::now()
    }))
}

#[get("/health/ready")]
pub async fn readiness_probe() -> impl Responder {
    let mut ready = true;
    let mut checks = json!({});

    // Check SurrealDB
    match SurrealClient::new().await {
        Ok(client) => {
            match client.db.query("INFO FOR DB").await {
                Ok(_) => checks["database"] = json!({"status": "ok"}),
                Err(e) => {
                    ready = false;
                    checks["database"] = json!({"status": "error", "message": e.to_string()});
                }
            }
        }
        Err(e) => {
            ready = false;
            checks["database"] = json!({"status": "error", "message": e.to_string()});
        }
    }

    // Check Qdrant
    match QdrantServiceClient::new() {
        Ok(client) => {
            match client.health_check().await {
                Ok(_) => checks["qdrant"] = json!({"status": "ok"}),
                Err(e) => {
                    ready = false;
                    checks["qdrant"] = json!({"status": "error", "message": e.to_string()});
                }
            }
        }
        Err(e) => {
            ready = false;
            checks["qdrant"] = json!({"status": "error", "message": e.to_string()});
        }
    }

    // Check vLLM health via direct HTTP ping
    match reqwest::Client::new().get("http://localhost:8000/health").send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                checks["vllm"] = json!({"status": "ok"});
            } else {
                ready = false;
                checks["vllm"] = json!({"status": "error", "message": format!("vLLM returned status {}", resp.status())});
            }
        }
        Err(e) => {
            ready = false;
            checks["vllm"] = json!({"status": "error", "message": e.to_string()});
        }
    }

    let status_code = if ready {
        actix_web::http::StatusCode::OK
    } else {
        actix_web::http::StatusCode::SERVICE_UNAVAILABLE
    };

    HttpResponse::build(status_code).json(json!({
        "status": if ready { "ready" } else { "not_ready" },
        "checks": checks
    }))
}

#[get("/metrics")]
pub async fn metrics_endpoint() -> impl Responder {
    use prometheus::Encoder;
    let metric_families = prometheus::gather();
    let encoder = prometheus::TextEncoder::new();
    let mut buffer = Vec::new();
    
    match encoder.encode(&metric_families, &mut buffer) {
        Ok(_) => HttpResponse::Ok()
            .content_type("text/plain; version=0.0.4; charset=utf-8")
            .body(String::from_utf8(buffer).unwrap_or_default()),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
