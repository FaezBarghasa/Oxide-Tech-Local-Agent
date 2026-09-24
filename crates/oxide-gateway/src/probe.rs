use reqwest::Client;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, warn};

/// Connection-pooled latency prober that avoids allocating a new reqwest::Client on each call.
pub struct LatencyProber {
    client: Client,
}

impl Default for LatencyProber {
    fn default() -> Self {
        Self::new()
    }
}

impl LatencyProber {
    pub fn new() -> Self {
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(2))
            .timeout(Duration::from_secs(3))
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(10)
            .tcp_nodelay(true)
            .build()
            .unwrap_or_else(|_| Client::new());
        Self { client }
    }

    /// Probe the online API endpoint for reachability and measure round-trip latency.
    pub async fn probe_latency(&self, base_url: &str) -> Result<Duration, anyhow::Error> {
        let start = Instant::now();
        let resp = self.client.head(base_url).send().await;
        let elapsed = start.elapsed();

        match resp {
            Ok(r) => {
                debug!(
                    status = %r.status(),
                    latency_ms = elapsed.as_millis(),
                    url = %base_url,
                    "Latency probe succeeded"
                );
                Ok(elapsed)
            }
            Err(e) => {
                warn!(error = %e, url = %base_url, "Latency probe failed");
                Err(anyhow::anyhow!("Probe failed for {}: {}", base_url, e))
            }
        }
    }

    /// Returns `true` if the endpoint is reachable within `timeout_ms`.
    pub async fn is_reachable(&self, base_url: &str, timeout_ms: u64) -> bool {
        let threshold = Duration::from_millis(timeout_ms);
        match self.probe_latency(base_url).await {
            Ok(latency) => latency < threshold,
            Err(_) => false,
        }
    }
}

/// A background ticker that periodically tests endpoint health to allow zero-latency routing lookups.
pub struct BackgroundHealthMonitor {
    prober: Arc<LatencyProber>,
    primary_reachable: Arc<AtomicBool>,
    secondary_reachable: Arc<AtomicBool>,
    primary_latency_ms: Arc<AtomicU64>,
    secondary_latency_ms: Arc<AtomicU64>,
}

impl BackgroundHealthMonitor {
    pub fn new(
        primary_url: String,
        secondary_url: String,
        threshold_ms: u64,
        check_interval: Duration,
    ) -> (Self, tokio::task::JoinHandle<()>) {
        let prober = Arc::new(LatencyProber::new());
        let primary_reachable = Arc::new(AtomicBool::new(true));
        let secondary_reachable = Arc::new(AtomicBool::new(true));
        let primary_latency_ms = Arc::new(AtomicU64::new(0));
        let secondary_latency_ms = Arc::new(AtomicU64::new(0));

        let p_prober = Arc::clone(&prober);
        let p_reach = Arc::clone(&primary_reachable);
        let s_reach = Arc::clone(&secondary_reachable);
        let p_lat = Arc::clone(&primary_latency_ms);
        let s_lat = Arc::clone(&secondary_latency_ms);

        let handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(check_interval);
            loop {
                interval.tick().await;

                // Concurrent ping using join
                let (p_res, s_res) = tokio::join!(
                    p_prober.probe_latency(&primary_url),
                    p_prober.probe_latency(&secondary_url)
                );

                if let Ok(lat) = p_res {
                    let ms = lat.as_millis() as u64;
                    p_lat.store(ms, Ordering::Release);
                    p_reach.store(ms < threshold_ms, Ordering::Release);
                } else {
                    p_reach.store(false, Ordering::Release);
                }

                if let Ok(lat) = s_res {
                    let ms = lat.as_millis() as u64;
                    s_lat.store(ms, Ordering::Release);
                    s_reach.store(ms < threshold_ms, Ordering::Release);
                } else {
                    s_reach.store(false, Ordering::Release);
                }
            }
        });

        (
            Self {
                prober,
                primary_reachable,
                secondary_reachable,
                primary_latency_ms,
                secondary_latency_ms,
            },
            handle,
        )
    }

    #[inline(always)]
    pub fn is_primary_healthy(&self) -> bool {
        self.primary_reachable.load(Ordering::Acquire)
    }

    #[inline(always)]
    pub fn is_secondary_healthy(&self) -> bool {
        self.secondary_reachable.load(Ordering::Acquire)
    }

    pub fn primary_latency(&self) -> u64 {
        self.primary_latency_ms.load(Ordering::Acquire)
    }

    pub fn secondary_latency(&self) -> u64 {
        self.secondary_latency_ms.load(Ordering::Acquire)
    }

    pub fn prober(&self) -> &LatencyProber {
        &self.prober
    }
}

static GLOBAL_PROBER: std::sync::OnceLock<LatencyProber> = std::sync::OnceLock::new();

fn get_prober() -> &'static LatencyProber {
    GLOBAL_PROBER.get_or_init(LatencyProber::new)
}

/// Backwards compatible functions using the pooled prober
pub async fn probe_latency(base_url: &str) -> Result<Duration, anyhow::Error> {
    get_prober().probe_latency(base_url).await
}

pub async fn is_reachable(base_url: &str, timeout_ms: u64) -> bool {
    get_prober().is_reachable(base_url, timeout_ms).await
}
