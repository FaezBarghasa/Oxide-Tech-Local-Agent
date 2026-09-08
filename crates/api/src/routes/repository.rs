use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<FileNode>>,
}

#[derive(Debug, Deserialize)]
pub struct RepoStructureRequest {
    pub workspace_path: Option<String>,
}

fn build_tree(dir: &Path, base_path: &Path) -> Option<Vec<FileNode>> {
    let mut nodes = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            if name == "target" || name.starts_with('.') {
                continue;
            }

            let is_dir = path.is_dir();
            let relative_path = path
                .strip_prefix(base_path)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();

            let children = if is_dir {
                build_tree(&path, base_path)
            } else {
                None
            };

            nodes.push(FileNode {
                name,
                path: relative_path,
                is_dir,
                children,
            });
        }
    }

    nodes.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir)
        } else {
            a.name.cmp(&b.name)
        }
    });

    if nodes.is_empty() {
        None
    } else {
        Some(nodes)
    }
}

pub async fn run_repository_structure(workspace_path: String) -> Result<FileNode, String> {
    let base_path = PathBuf::from(&workspace_path);
    if !base_path.exists() {
        return Err(format!("Workspace path does not exist: {}", workspace_path));
    }

    let root_node = tokio::task::spawn_blocking(move || {
        let children = build_tree(&base_path, &base_path);
        let name = base_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("root")
            .to_string();
        FileNode {
            name,
            path: "".to_string(),
            is_dir: true,
            children,
        }
    })
    .await
    .map_err(|e| format!("Tree building task panicked: {}", e))?;

    Ok(root_node)
}

#[get("/api/repository/structure")]
pub async fn handle_repository_structure(
    query: web::Query<RepoStructureRequest>,
) -> impl Responder {
    let workspace_path = query
        .workspace_path
        .clone()
        .unwrap_or_else(|| "/home/jrad/RustroverProjects/Oxide-Tech-Local-Agent".to_string());

    match run_repository_structure(workspace_path).await {
        Ok(tree) => HttpResponse::Ok().json(tree),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "message": e
        })),
    }
}
