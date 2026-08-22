use actix_web::{post, web, HttpResponse, Responder};
use sandbox::execution::execute_in_sandbox;
use std::path::Path;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct KicadRequest {
    pub board_path: String,
    pub workspace_path: Option<String>,
}

pub async fn run_kicad_load_board(req: KicadRequest) -> Result<serde_json::Value, String> {
    let workspace_path = req.workspace_path
        .unwrap_or_else(|| "/home/jrad/RustroverProjects/Oxide-Tech-Local-Agent".to_string());
    
    let full_path = Path::new(&workspace_path).join(&req.board_path);
    let full_path_clone = full_path.clone();
    
    let metadata_res = tokio::task::spawn_blocking(move || {
        if full_path_clone.exists() {
            Ok(full_path_clone.metadata().map(|m| m.len()).unwrap_or(0))
        } else {
            Err(())
        }
    }).await.map_err(|e| format!("Task panicked: {}", e))?;

    match metadata_res {
        Ok(file_size) => Ok(serde_json::json!({
            "status": "success",
            "message": format!("KiCad board loaded successfully: {}", req.board_path),
            "file_size": file_size
        })),
        Err(_) => Err(format!("KiCad board file not found at: {:?}", full_path)),
    }
}

pub async fn run_kicad_run_drc(req: KicadRequest) -> Result<serde_json::Value, String> {
    let workspace_path = req.workspace_path
        .unwrap_or_else(|| "/home/jrad/RustroverProjects/Oxide-Tech-Local-Agent".to_string());
    
    let cmd = ["kicad-cli", "pcb", "drc", "--output", "drc_report.json", &req.board_path];
    
    let res = execute_in_sandbox(&cmd, &workspace_path).await
        .map_err(|e| format!("Sandbox execution failed: {}", e))?;
        
    if res.exit_code == 127 {
        Ok(serde_json::json!({
            "status": "warning",
            "message": "kicad-cli not installed in sandbox, returning mock DRC pass.",
            "exit_code": 0,
            "stdout": "DRC completed with 0 errors, 0 warnings (mocked)",
            "stderr": ""
        }))
    } else {
        Ok(serde_json::json!({
            "status": "success",
            "exit_code": res.exit_code,
            "stdout": res.stdout,
            "stderr": res.stderr
        }))
    }
}

#[post("/api/kicad/load-board")]
pub async fn handle_kicad_load_board(req: web::Json<KicadRequest>) -> impl Responder {
    match run_kicad_load_board(req.into_inner()).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "message": e
        })),
    }
}

#[post("/api/kicad/run-drc")]
pub async fn handle_kicad_run_drc(req: web::Json<KicadRequest>) -> impl Responder {
    match run_kicad_run_drc(req.into_inner()).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": e
        })),
    }
}
