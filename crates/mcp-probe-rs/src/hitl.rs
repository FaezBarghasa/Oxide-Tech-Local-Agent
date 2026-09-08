use std::path::PathBuf;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::UnixListener;
use tokio::sync::broadcast;
use tokio::time::timeout;
use tracing::{info, warn};

/// Manages a simple UNIX‑domain‑socket based Human‑In‑The‑Loop (HITL) confirmation.
///
/// The server creates a listener at the configured `socket_path`. When a gated
/// operation is requested, `wait_for_confirmation` blocks until an operator writes
/// `CONFIRM\n` to the socket or the timeout expires.
#[derive(Clone)]
pub struct HitlGate {
    socket_path: PathBuf,
    timeout_secs: u64,
    confirm_tx: broadcast::Sender<()>,
}

impl HitlGate {
    pub fn new(socket_path: PathBuf, timeout_secs: u64) -> Self {
        let (confirm_tx, _) = broadcast::channel(32);
        Self {
            socket_path,
            timeout_secs,
            confirm_tx,
        }
    }

    /// Starts the background listener. It runs forever, accepting connections
    /// and parsing confirmation messages.
    pub async fn spawn_listener(&self) {
        // Remove any stale socket file.
        let _ = std::fs::remove_file(&self.socket_path);
        let listener = match UnixListener::bind(&self.socket_path) {
            Ok(l) => l,
            Err(e) => {
                warn!(
                    "Failed to bind HITL socket {}: {}",
                    self.socket_path.display(),
                    e
                );
                return;
            }
        };
        info!("HITL socket listening at {}", self.socket_path.display());
        let tx = self.confirm_tx.clone();
        loop {
            match listener.accept().await {
                Ok((stream, _addr)) => {
                    let tx_inner = tx.clone();
                    tokio::spawn(async move {
                        let mut reader = BufReader::new(stream);
                        let mut line = String::new();
                        if let Ok(Ok(_)) =
                            timeout(Duration::from_secs(30), reader.read_line(&mut line)).await
                        {
                            if line.trim().eq_ignore_ascii_case("CONFIRM") {
                                info!("HITL CONFIRM token received via UNIX socket");
                                let _ = tx_inner.send(());
                            }
                        }
                    });
                }
                Err(e) => {
                    warn!("HITL listener accept error: {}", e);
                    break;
                }
            }
        }
    }

    /// Blocks until an operator client sends a line containing "CONFIRM" or the timeout
    /// expires. Returns `true` on confirmation, `false` otherwise.
    pub async fn wait_for_confirmation(&self) -> bool {
        let mut rx = self.confirm_tx.subscribe();
        match timeout(Duration::from_secs(self.timeout_secs), rx.recv()).await {
            Ok(Ok(())) => {
                info!("HITL confirmation verified for hardware action");
                true
            }
            _ => {
                warn!(
                    "HITL confirmation timed out after {} seconds",
                    self.timeout_secs
                );
                false
            }
        }
    }
}
