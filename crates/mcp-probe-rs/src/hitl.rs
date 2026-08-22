use tokio::net::UnixListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::path::PathBuf;
use std::time::Duration;
use tokio::time::timeout;
use tracing::{info, warn};

/// Manages a simple UNIX‑domain‑socket based Human‑In‑The‑Loop (HITL) confirmation.
///
/// The server creates a listener at the configured `socket_path`. When a gated
/// operation is requested, `wait_for_confirmation` blocks until a client writes
/// a line containing `CONFIRM` (case‑insensitive) or the timeout expires.
pub struct HitlGate {
    socket_path: PathBuf,
    timeout_secs: u64,
}

impl HitlGate {
    pub fn new(socket_path: PathBuf, timeout_secs: u64) -> Self {
        Self { socket_path, timeout_secs }
    }

    /// Starts the background listener. It runs forever, accepting connections
    /// and discarding any data – the client only needs to connect so the
    /// kernel creates the socket file.
    pub async fn spawn_listener(&self) {
        // Remove any stale socket file.
        let _ = std::fs::remove_file(&self.socket_path);
        let listener = match UnixListener::bind(&self.socket_path) {
            Ok(l) => l,
            Err(e) => {
                warn!("Failed to bind HITL socket {}: {}", self.socket_path.display(), e);
                return;
            }
        };
        info!("HITL socket listening at {}", self.socket_path.display());
        loop {
            match listener.accept().await {
                Ok((_stream, _addr)) => {
                    // We don't need to keep the connection alive – just accept.
                }
                Err(e) => {
                    warn!("HITL listener accept error: {}", e);
                    break;
                }
            }
        }
    }

    /// Blocks until a client sends a line containing "CONFIRM" or the timeout
    /// expires. Returns `true` on confirmation, `false` otherwise.
    pub async fn wait_for_confirmation(&self) -> bool {
        // Connect to the socket – the client side (e.g., a physical button
        // script) will write "CONFIRM\n" when the button is pressed.
        let connect_fut = tokio::net::UnixStream::connect(&self.socket_path);
        let stream = match timeout(Duration::from_secs(self.timeout_secs), connect_fut).await {
            Ok(Ok(s)) => s,
            _ => {
                warn!("HITL confirmation timed out after {} seconds", self.timeout_secs);
                return false;
            }
        };
        let mut reader = BufReader::new(stream);
        let mut line = String::new();
        match timeout(Duration::from_secs(self.timeout_secs), reader.read_line(&mut line)).await {
            Ok(Ok(_)) => {
                if line.trim().eq_ignore_ascii_case("CONFIRM") {
                    info!("HITL confirmation received");
                    return true;
                }
                warn!("HITL received unexpected line: {}", line.trim());
                false
            }
            _ => {
                warn!("HITL confirmation read timed out");
                false
            }
        }
    }
}
