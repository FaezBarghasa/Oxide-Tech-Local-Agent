//! In-memory zero-copy custom protocol streamer for Tauri v2.
//! Streams frontend assets directly from executable .rodata memory slices.

use crate::assets::WebAssets;
use tauri::http::{Request, Response};

#[allow(dead_code)]
pub fn handle_oxide_protocol(
    request: &Request<Vec<u8>>,
) -> Result<Response<Vec<u8>>, Box<dyn std::error::Error>> {
    let path = request.uri().path().trim_start_matches('/');
    let target_path = if path.is_empty() { "index.html" } else { path };

    match WebAssets::get(target_path) {
        Some(asset) => {
            let mime = mime_guess::from_path(target_path).first_or_octet_stream();
            let data = asset.data.into_owned();
            let len = data.len().to_string();
            Ok(Response::builder()
                .status(200)
                .header("Content-Type", mime.as_ref())
                .header("Content-Length", len)
                .header("Access-Control-Allow-Origin", "*")
                .header("Cache-Control", "no-cache")
                .body(data)?)
        }
        None => {
            if let Some(fallback) = WebAssets::get("index.html") {
                let data = fallback.data.into_owned();
                let len = data.len().to_string();
                Ok(Response::builder()
                    .status(200)
                    .header("Content-Type", "text/html")
                    .header("Content-Length", len)
                    .header("Access-Control-Allow-Origin", "*")
                    .body(data)?)
            } else {
                Ok(Response::builder()
                    .status(404)
                    .header("Content-Type", "text/plain")
                    .header("Access-Control-Allow-Origin", "*")
                    .body(b"Asset not found in embedded bundle".to_vec())?)
            }
        }
    }
}
