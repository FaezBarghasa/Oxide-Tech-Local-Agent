use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use tracing::{info, warn};

use config_loader::AppConfig;
use vllm_client::{LlmRouterClient, ThinkerClient};

use crate::probe::is_reachable;
use crate::quality::QualityGate;

// ── Backend enum ──────────────────────────────────────────────────────────────

/// Which coder backend the router has selected for the current request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoderBackend {
    /// Primary online coder (e.g. Groq Llama-3.3-70B).
    OnlinePrimary,
    /// Secondary online coder fallback (e.g. Mistral Codestral).
    OnlineSecondary,
    /// Local Ollama coder (always available, no network needed).
    Local,
}

impl std::fmt::Display for CoderBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoderBackend::OnlinePrimary => write!(f, "Online/Primary"),
            CoderBackend::OnlineSecondary => write!(f, "Online/Secondary"),
            CoderBackend::Local => write!(f, "Local/Ollama"),
        }
    }
}

// ── Atomic state ──────────────────────────────────────────────────────────────

/// Internal routing state encoded as a u8 for atomic updates:
/// 0 = OnlinePrimary, 1 = OnlineSecondary, 2 = Local
const STATE_ONLINE_PRIMARY: u8 = 0;
const STATE_ONLINE_SECONDARY: u8 = 1;
const STATE_LOCAL: u8 = 2;

// ── GatewayRouter ─────────────────────────────────────────────────────────────

/// The smart routing controller.
///
/// Holds all three coder clients and the thinker client.  On each generation
/// request it:
/// 1. Probes the primary online endpoint for latency.
/// 2. If too slow or unreachable, downgrades to secondary or local.
/// 3. After a cargo-check pass, scores the output with `QualityGate`.
/// 4. If quality is below threshold it flips to the other coder.
pub struct GatewayRouter {
    pub thinker: ThinkerClient,
    pub online_primary: LlmRouterClient,
    pub online_secondary: LlmRouterClient,
    pub local: LlmRouterClient,
    quality_gate: QualityGate,
    latency_threshold_ms: u64,
    /// Tracks which backend is currently "preferred".  Atomic so it can be
    /// updated from the verification loop without a Mutex.
    current_backend: Arc<AtomicU8>,
}

impl GatewayRouter {
    /// Construct the router from the loaded `AppConfig`.
    pub fn from_config(cfg: &AppConfig) -> Self {
        let thinker = ThinkerClient::from_config(&cfg.thinker);
        let online_primary = LlmRouterClient::from_config(&cfg.coder.online.primary);
        let online_secondary = LlmRouterClient::from_config(&cfg.coder.online.secondary);
        let local = LlmRouterClient::from_config(&cfg.coder.local);
        let quality_gate = QualityGate::new(cfg.gateway.quality_threshold);

        Self {
            thinker,
            online_primary,
            online_secondary,
            local,
            quality_gate,
            latency_threshold_ms: cfg.gateway.latency_threshold_ms,
            current_backend: Arc::new(AtomicU8::new(STATE_ONLINE_PRIMARY)),
        }
    }

    /// Probe the network and return the best available coder backend.
    ///
    /// Decision tree:
    ///   1. Probe primary online endpoint.
    ///   2. If latency OK → `OnlinePrimary`.
    ///   3. If primary is slow/down → probe secondary.
    ///   4. If secondary OK → `OnlineSecondary`.
    ///   5. Otherwise → `Local`.
    pub async fn select_backend(
        &self,
        primary_url: &str,
        secondary_url: &str,
    ) -> CoderBackend {
        if is_reachable(primary_url, self.latency_threshold_ms).await {
            info!("Router: primary online coder reachable — using OnlinePrimary");
            self.current_backend
                .store(STATE_ONLINE_PRIMARY, Ordering::Relaxed);
            return CoderBackend::OnlinePrimary;
        }

        warn!(
            "Router: primary online coder unreachable or too slow (>{} ms) — trying secondary",
            self.latency_threshold_ms
        );

        if is_reachable(secondary_url, self.latency_threshold_ms).await {
            info!("Router: secondary online coder reachable — using OnlineSecondary");
            self.current_backend
                .store(STATE_ONLINE_SECONDARY, Ordering::Relaxed);
            return CoderBackend::OnlineSecondary;
        }

        warn!("Router: both online coders unreachable — falling back to Local");
        self.current_backend.store(STATE_LOCAL, Ordering::Relaxed);
        CoderBackend::Local
    }

    /// Get the `LlmRouterClient` reference for the given backend.
    pub fn client_for(&self, backend: CoderBackend) -> &LlmRouterClient {
        match backend {
            CoderBackend::OnlinePrimary => &self.online_primary,
            CoderBackend::OnlineSecondary => &self.online_secondary,
            CoderBackend::Local => &self.local,
        }
    }

    /// Evaluate cargo-check output and decide whether to flip the backend.
    ///
    /// Returns `Some(CoderBackend)` with a recommended fallback if the current
    /// backend is deemed "unsatisfying", or `None` if quality is acceptable.
    pub fn evaluate_quality(
        &self,
        stderr: &str,
        current: CoderBackend,
    ) -> Option<CoderBackend> {
        if self.quality_gate.is_satisfying(stderr) {
            return None;
        }

        let (score, error_count) = self.quality_gate.score(stderr);
        warn!(
            score = score,
            error_count = error_count,
            current_backend = %current,
            "QualityGate: output below threshold — recommending backend flip"
        );

        // Flip strategy: Primary → Secondary → Local (never go backwards
        // automatically — let the next probe decide if online recovers).
        let next = match current {
            CoderBackend::OnlinePrimary => CoderBackend::OnlineSecondary,
            CoderBackend::OnlineSecondary => CoderBackend::Local,
            CoderBackend::Local => CoderBackend::Local, // already at floor
        };

        if next == current {
            warn!("Router: already at local coder, no further fallback available");
            None
        } else {
            info!("Router: flipping backend to {}", next);
            Some(next)
        }
    }

    /// Returns a human-readable summary of the current routing state for the
    /// `/api/status` endpoint.
    pub fn status_summary(&self) -> serde_json::Value {
        let backend_str = match self.current_backend.load(Ordering::Relaxed) {
            STATE_ONLINE_PRIMARY => "OnlinePrimary",
            STATE_ONLINE_SECONDARY => "OnlineSecondary",
            _ => "Local",
        };

        serde_json::json!({
            "current_backend": backend_str,
            "latency_threshold_ms": self.latency_threshold_ms,
        })
    }
}
