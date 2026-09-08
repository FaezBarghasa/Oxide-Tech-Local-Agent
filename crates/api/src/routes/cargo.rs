use actix_web::{post, web, HttpResponse, Responder};
use sandbox::execution::execute_in_sandbox;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CargoRequest {
    pub workspace_path: Option<String>,
}

pub async fn run_cargo_check(req: CargoRequest) -> Result<serde_json::Value, String> {
    let workspace_path = req
        .workspace_path
        .unwrap_or_else(|| "/home/jrad/RustroverProjects/Oxide-Tech-Local-Agent".to_string());

    let res = execute_in_sandbox(&["cargo", "check"], &workspace_path)
        .await
        .map_err(|e| format!("Sandbox execution failed: {}", e))?;

    Ok(serde_json::json!({
        "status": "success",
        "exit_code": res.exit_code,
        "stdout": res.stdout,
        "stderr": res.stderr
    }))
}

pub async fn run_cargo_clippy(req: CargoRequest) -> Result<serde_json::Value, String> {
    let workspace_path = req
        .workspace_path
        .unwrap_or_else(|| "/home/jrad/RustroverProjects/Oxide-Tech-Local-Agent".to_string());

    let res = execute_in_sandbox(&["cargo", "clippy"], &workspace_path)
        .await
        .map_err(|e| format!("Sandbox execution failed: {}", e))?;

    Ok(serde_json::json!({
        "status": "success",
        "exit_code": res.exit_code,
        "stdout": res.stdout,
        "stderr": res.stderr
    }))
}

#[post("/api/cargo/check")]
pub async fn handle_cargo_check(req: web::Json<CargoRequest>) -> impl Responder {
    match run_cargo_check(req.into_inner()).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": e
        })),
    }
}

#[post("/api/cargo/clippy")]
pub async fn handle_cargo_clippy(req: web::Json<CargoRequest>) -> impl Responder {
    match run_cargo_clippy(req.into_inner()).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": e
        })),
    }
}
