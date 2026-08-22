use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::auth::{Claims, Role, generate_token};
use blog;
use common::config::AppConfig;
use scheduler;

// ── Structs ───────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub role: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Deserialize)]
pub struct ThinkRequest {
    pub prompt: String,
}

#[derive(Deserialize)]
pub struct ExecuteRequest {
    pub prompt: String,
    pub command: String,
    pub work_dir: String,
}

#[derive(Serialize)]
pub struct ExecuteResponse {
    pub success: bool,
    pub score: f32,
    pub output: String,
}

#[derive(Deserialize)]
pub struct RagQueryRequest {
    pub collection: String,
    pub query: String,
    pub limit: Option<usize>,
}

#[derive(Deserialize)]
pub struct RagIndexRequest {
    pub crate_name: String,
    pub version: String,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub surreal_connected: bool,
    pub qdrant_connected: bool,
    pub collections: Vec<String>,
}

// ── Route Handlers ────────────────────────────────────────────────────────────

/// Public Login: Issue JWT for testing Phase 0
pub async fn login(req: web::Json<LoginRequest>, config: web::Data<AppConfig>) -> impl Responder {
    let role = Role::from_str(&req.role);
    match generate_token(
        &req.username,
        role,
        &config.auth.jwt_secret,
        config.auth.jwt_expiration_hours,
    ) {
        Ok(token) => HttpResponse::Ok().json(LoginResponse { token }),
        Err(e) => HttpResponse::InternalServerError().body(format!("JWT Generation Error: {}", e)),
    }
}

/// Protected Think: Dispatches to Thinker (Requires Developer or Admin)
pub async fn think(
    claims: Claims,
    req: web::Json<ThinkRequest>,
    config: web::Data<AppConfig>,
) -> actix_web::Result<impl Responder> {
    claims.enforce_role(Role::Developer)?;
    match handle_think(&req.prompt, &config).await {
        Ok(plan) => Ok(HttpResponse::Ok().json(plan)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
    }
}

/// Protected Execute: Runs sandboxed command (Requires Developer or Admin)
pub async fn execute(
    claims: Claims,
    req: web::Json<ExecuteRequest>,
) -> actix_web::Result<impl Responder> {
    claims.enforce_role(Role::Developer)?;
    tracing::info!("Executing command for prompt: {}", req.prompt);
    match handle_execute(&req.command, &req.work_dir).await {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
    }
}

/// Protected RAG Query: Query Qdrant Collections (Requires Viewer, Developer or Admin)
pub async fn query_rag(
    claims: Claims,
    req: web::Json<RagQueryRequest>,
    knowledge: web::Data<Option<Arc<knowledge::KnowledgeClient>>>,
) -> actix_web::Result<impl Responder> {
    claims.enforce_role(Role::Viewer)?;
    match handle_query_rag(&req.collection, &req.query, req.limit, &knowledge).await {
        Ok(chunks) => Ok(HttpResponse::Ok().json(chunks)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
    }
}

/// Protected RAG Index: Index docs.rs (Requires Developer or Admin)
pub async fn index_rag(
    claims: Claims,
    req: web::Json<RagIndexRequest>,
    knowledge: web::Data<Option<Arc<knowledge::KnowledgeClient>>>,
) -> actix_web::Result<impl Responder> {
    claims.enforce_role(Role::Developer)?;
    match handle_index_rag(&req.crate_name, &req.version, &knowledge).await {
        Ok(msg) => Ok(HttpResponse::Accepted().body(msg)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
    }
}

/// Protected Status: Get system status (Requires Viewer, Developer or Admin)
pub async fn status(
    claims: Claims,
    memory: web::Data<Option<Arc<memory::SurrealClient>>>,
    knowledge: web::Data<Option<Arc<knowledge::KnowledgeClient>>>,
) -> actix_web::Result<impl Responder> {
    claims.enforce_role(Role::Viewer)?;
    let res = handle_status(&memory, &knowledge).await;
    Ok(HttpResponse::Ok().json(res))
}

// ── Shared Protocol-Agnostic Execution Handlers ───────────────────────────────

pub async fn handle_think(
    prompt: &str,
    config: &AppConfig,
) -> Result<thinker::ThinkerOutput, String> {
    let thinker = thinker::ThinkerClient::new();
    thinker
        .plan_task(prompt, config)
        .await
        .map_err(|e| format!("Thinker planning failed: {}", e))
}

pub async fn handle_execute(command: &str, work_dir: &str) -> Result<ExecuteResponse, String> {
    let cmd_parts: Vec<&str> = command.split_whitespace().collect();
    if cmd_parts.is_empty() {
        return Err("Empty command provided".to_string());
    }

    match verifier::execute_in_sandbox(&cmd_parts, work_dir).await {
        Ok(res) => {
            let pass = res.exit_code == 0;
            let score = if pass { 100.0 } else { 0.0 };
            Ok(ExecuteResponse {
                success: pass,
                score,
                output: format!("stdout:\n{}\nstderr:\n{}", res.stdout, res.stderr),
            })
        }
        Err(e) => Err(format!("Sandbox run failed: {}", e)),
    }
}

pub async fn handle_query_rag(
    collection: &str,
    query: &str,
    limit: Option<usize>,
    knowledge: &Option<Arc<knowledge::KnowledgeClient>>,
) -> Result<Vec<knowledge::EiosChunk>, String> {
    let limit = limit.unwrap_or(8);
    let kn = knowledge
        .as_ref()
        .ok_or_else(|| "RAG service is not initialized".to_string())?;

    kn.search(collection, query, limit)
        .await
        .map_err(|e| e.to_string())
}

pub async fn handle_index_rag(
    crate_name: &str,
    version: &str,
    knowledge: &Option<Arc<knowledge::KnowledgeClient>>,
) -> Result<String, String> {
    let _kn = knowledge
        .as_ref()
        .ok_or_else(|| "RAG service is not initialized".to_string())?;

    let embedder_client = match knowledge::KnowledgeClient::new().await {
        Ok(c) => Arc::new(c),
        Err(e) => return Err(format!("Failed to create embedding client: {}", e)),
    };

    let name = crate_name.to_string();
    let ver = version.to_string();

    tokio::spawn(async move {
        let _ = embedder_client.ingest_crate_docs(&name, &ver).await;
    });

    Ok("Ingestion task started in background".to_string())
}

pub async fn handle_status(
    memory: &Option<Arc<memory::SurrealClient>>,
    knowledge: &Option<Arc<knowledge::KnowledgeClient>>,
) -> StatusResponse {
    let surreal_connected = if let Some(m) = memory {
        m.db.query("INFO FOR DB").await.is_ok()
    } else {
        false
    };

    let qdrant_connected = if let Some(k) = knowledge {
        k.qdrant.list_collections().await.is_ok()
    } else {
        false
    };

    StatusResponse {
        surreal_connected,
        qdrant_connected,
        collections: knowledge::COLLECTIONS
            .iter()
            .map(|s| s.to_string())
            .collect(),
    }
}

// ── Public Health Routes ──────────────────────────────────────────────────────

pub async fn health_live() -> impl Responder {
    HttpResponse::Ok().body("Liveness probe OK")
}

pub async fn health_ready() -> impl Responder {
    HttpResponse::Ok().body("Readiness probe OK")
}

// ── Blog Routes ──────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct BlogQuery {
    pub page: Option<usize>,
}

pub async fn blog_index(
    query: web::Query<BlogQuery>,
    memory: web::Data<Option<Arc<memory::SurrealClient>>>,
    config: web::Data<AppConfig>,
) -> impl Responder {
    let Some(mc) = memory.as_ref() else {
        return HttpResponse::InternalServerError().body("SurrealDB service not initialized");
    };

    let page = query.page.unwrap_or(1);
    let per_page = config.blog.posts_per_page;

    let posts = match blog::list_posts(&mc.db, page, per_page).await {
        Ok(p) => p,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to query posts: {}", e));
        }
    };

    let total_count = match blog::count_posts(&mc.db).await {
        Ok(c) => c,
        Err(_) => 0,
    };

    let total_pages = if total_count == 0 {
        1
    } else {
        (total_count + per_page - 1) / per_page
    };

    let html = blog::render_index(&posts, page, total_pages, &config.blog);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn blog_post(
    path: web::Path<String>,
    memory: web::Data<Option<Arc<memory::SurrealClient>>>,
    config: web::Data<AppConfig>,
) -> impl Responder {
    let Some(mc) = memory.as_ref() else {
        return HttpResponse::InternalServerError().body("SurrealDB service not initialized");
    };

    let id = path.into_inner();
    match blog::get_post(&mc.db, &id).await {
        Ok(Some(post)) => {
            let html = blog::render_post(&post, &config.blog);
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html)
        }
        Ok(None) => HttpResponse::NotFound().body("پست مورد نظر یافت نشد"),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to retrieve post: {}", e))
        }
    }
}

pub async fn blog_tag(
    path: web::Path<String>,
    memory: web::Data<Option<Arc<memory::SurrealClient>>>,
    config: web::Data<AppConfig>,
) -> impl Responder {
    let Some(mc) = memory.as_ref() else {
        return HttpResponse::InternalServerError().body("SurrealDB service not initialized");
    };

    let tag = path.into_inner();
    match blog::list_by_tag(&mc.db, &tag).await {
        Ok(posts) => {
            let html = blog::render_tag_page(&tag, &posts, &config.blog);
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to list posts: {}", e)),
    }
}

pub async fn blog_feed(
    memory: web::Data<Option<Arc<memory::SurrealClient>>>,
    config: web::Data<AppConfig>,
) -> impl Responder {
    let Some(mc) = memory.as_ref() else {
        return HttpResponse::InternalServerError().body("SurrealDB service not initialized");
    };

    let base_url = format!(
        "http://{}:{}{}",
        config.gateway.host, config.gateway.port, config.blog.base_path
    );
    match blog::get_rss_feed(
        &mc.db,
        &config.blog.site_name,
        &config.blog.description,
        &base_url,
    )
    .await
    {
        Ok(xml) => HttpResponse::Ok()
            .content_type("application/atom+xml; charset=utf-8")
            .body(xml),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to generate feed: {}", e))
        }
    }
}

pub async fn blog_api_posts(
    memory: web::Data<Option<Arc<memory::SurrealClient>>>,
) -> impl Responder {
    let Some(mc) = memory.as_ref() else {
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({ "error": "SurrealDB service not initialized" }));
    };

    match blog::list_posts(&mc.db, 1, 100).await {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() }))
        }
    }
}

pub async fn update_knowledge(
    claims: Claims,
    memory: web::Data<Option<Arc<memory::SurrealClient>>>,
    config: web::Data<AppConfig>,
) -> actix_web::Result<impl Responder> {
    claims.enforce_role(Role::Developer)?;
    let Some(mc) = memory.as_ref() else {
        return Ok(HttpResponse::InternalServerError().body("SurrealDB service not initialized"));
    };

    let db = mc.db.clone();
    let config_clone = (*config).clone();

    tokio::spawn(async move {
        tracing::info!("Manual knowledge update triggered by user.");
        if let Err(e) = scheduler::run_update_cycle(&config_clone, &db).await {
            tracing::error!("Manual update cycle failed: {}", e);
        }
    });

    Ok(HttpResponse::Accepted().body("Manual update cycle started in the background"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::{Claims, Role};
    use actix_web::body::to_bytes;

    #[test]
    fn test_role_parsing() {
        assert_eq!(Role::from_str("admin"), Role::Admin);
        assert_eq!(Role::from_str("Admin"), Role::Admin);
        assert_eq!(Role::from_str("developer"), Role::Developer);
        assert_eq!(Role::from_str("viewer"), Role::Viewer);
        assert_eq!(Role::from_str("unknown"), Role::Viewer);
    }

    #[test]
    fn test_enforce_role() {
        let admin_claims = Claims {
            sub: "admin_user".to_string(),
            role: Role::Admin,
            exp: 0,
        };
        let dev_claims = Claims {
            sub: "dev_user".to_string(),
            role: Role::Developer,
            exp: 0,
        };
        let viewer_claims = Claims {
            sub: "viewer_user".to_string(),
            role: Role::Viewer,
            exp: 0,
        };

        // Admin can access everything
        assert!(admin_claims.enforce_role(Role::Admin).is_ok());
        assert!(admin_claims.enforce_role(Role::Developer).is_ok());
        assert!(admin_claims.enforce_role(Role::Viewer).is_ok());

        // Developer can access Developer/Viewer but not Admin
        assert!(dev_claims.enforce_role(Role::Admin).is_err());
        assert!(dev_claims.enforce_role(Role::Developer).is_ok());
        assert!(dev_claims.enforce_role(Role::Viewer).is_ok());

        // Viewer can access Viewer only
        assert!(viewer_claims.enforce_role(Role::Admin).is_err());
        assert!(viewer_claims.enforce_role(Role::Developer).is_err());
        assert!(viewer_claims.enforce_role(Role::Viewer).is_ok());
    }

    #[tokio::test]
    async fn test_health_routes() {
        let live_res = health_live()
            .await
            .respond_to(&actix_web::test::TestRequest::default().to_http_request());
        assert_eq!(live_res.status(), actix_web::http::StatusCode::OK);
        let live_body = to_bytes(live_res.into_body())
            .await
            .map_err(|_| "body error")
            .unwrap();
        assert_eq!(live_body, "Liveness probe OK");

        let ready_res = health_ready()
            .await
            .respond_to(&actix_web::test::TestRequest::default().to_http_request());
        assert_eq!(ready_res.status(), actix_web::http::StatusCode::OK);
        let ready_body = to_bytes(ready_res.into_body())
            .await
            .map_err(|_| "body error")
            .unwrap();
        assert_eq!(ready_body, "Readiness probe OK");
    }
}
