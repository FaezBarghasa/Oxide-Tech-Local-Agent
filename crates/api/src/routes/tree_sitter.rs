use actix_web::{post, web, HttpResponse, Responder};
use std::fs;
use std::path::{Path, PathBuf};
use surrealdb_service::client::SurrealClient;
use tree_sitter_service::parser::parse_file;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ParseRequest {
    pub workspace_path: Option<String>,
}

fn get_rs_files(dir: &Path, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name == "target" || name.starts_with('.') {
                    continue;
                }
                get_rs_files(&path, files);
            } else if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "rs" {
                        files.push(path);
                    }
                }
            }
        }
    }
}

pub async fn run_tree_sitter_parse(req: ParseRequest) -> Result<serde_json::Value, String> {
    let workspace_path = req
        .workspace_path
        .unwrap_or_else(|| "/home/jrad/RustroverProjects/Oxide-Tech-Local-Agent".to_string());

    let path = PathBuf::from(&workspace_path);
    if !path.exists() {
        return Err(format!("Workspace path does not exist: {}", workspace_path));
    }

    let (all_symbols, files_parsed) = tokio::task::spawn_blocking(move || {
        let mut files = Vec::new();
        get_rs_files(&path, &mut files);

        let mut all_symbols = Vec::new();
        let mut files_parsed = 0;

        for file_path in &files {
            if let Ok(content) = fs::read_to_string(file_path) {
                let fp_str = file_path.to_string_lossy().to_string();
                if let Ok(symbols) = parse_file(&content, &fp_str) {
                    all_symbols.extend(symbols);
                    files_parsed += 1;
                }
            }
        }
        (all_symbols, files_parsed)
    })
    .await
    .map_err(|e| format!("Parsing thread panicked: {}", e))?;

    let client = SurrealClient::new()
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    client
        .save_symbols(&all_symbols)
        .await
        .map_err(|e| format!("Failed to save symbols to database: {}", e))?;

    Ok(serde_json::json!({
        "status": "success",
        "files_parsed": files_parsed,
        "symbols_indexed": all_symbols.len()
    }))
}

#[post("/api/tree-sitter/parse")]
pub async fn handle_tree_sitter_parse(req: web::Json<ParseRequest>) -> impl Responder {
    match run_tree_sitter_parse(req.into_inner()).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "message": e
        })),
    }
}
