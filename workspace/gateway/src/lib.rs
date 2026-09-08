#![allow(
    clippy::collapsible_if,
    clippy::manual_div_ceil,
    clippy::manual_unwrap_or,
    clippy::manual_unwrap_or_default,
    clippy::match_like_matches_macro,
    clippy::should_implement_trait
)]

pub mod auth;
pub mod h3_gateway;
pub mod routes;
pub mod websocket;

use tracing::{info, warn};

pub fn ensure_certs() {
    use std::path::Path;
    if !Path::new("cert.pem").exists() || !Path::new("key.pem").exists() {
        info!("TLS certificates not found. Generating self-signed certificates using openssl...");
        let status = std::process::Command::new("openssl")
            .args([
                "req",
                "-x509",
                "-newkey",
                "rsa:2048",
                "-keyout",
                "key.pem",
                "-out",
                "cert.pem",
                "-sha256",
                "-days",
                "365",
                "-nodes",
                "-subj",
                "/CN=localhost",
            ])
            .status();
        match status {
            Ok(s) if s.success() => info!("Self-signed TLS certificates generated successfully."),
            _ => warn!(
                "Failed to generate self-signed TLS certificates. QUIC/HTTP3 gateway might fail to start."
            ),
        }
    }
}
