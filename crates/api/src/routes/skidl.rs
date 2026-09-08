use actix_web::{post, web, HttpResponse, Responder};
use sandbox::execution::execute_in_sandbox;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct SkidlRequest {
    pub script_path: String,
    pub workspace_path: Option<String>,
}

pub async fn run_skidl_generate(req: SkidlRequest) -> Result<serde_json::Value, String> {
    let workspace_path = req
        .workspace_path
        .unwrap_or_else(|| "/home/jrad/RustroverProjects/Oxide-Tech-Local-Agent".to_string());

    let cmd = ["python3", &req.script_path];

    let res = execute_in_sandbox(&cmd, &workspace_path)
        .await
        .map_err(|e| format!("Sandbox execution failed: {}", e))?;

    Ok(serde_json::json!({
        "status": "success",
        "exit_code": res.exit_code,
        "stdout": res.stdout,
        "stderr": res.stderr
    }))
}

#[post("/api/skidl/generate")]
pub async fn handle_skidl_generate(req: web::Json<SkidlRequest>) -> impl Responder {
    match run_skidl_generate(req.into_inner()).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": e
        })),
    }
}
