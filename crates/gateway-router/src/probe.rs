use std::time::{Duration, Instant};
use tracing::{debug, warn};

/// Probe the online API endpoint for reachability and measure round-trip
/// latency.
///
/// Sends a lightweight HEAD request to `base_url` (or GET if HEAD is not
/// supported).  Returns the RTT, or an error if the host is unreachable.
pub async fn probe_latency(base_url: &str) -> Result<Duration, anyhow::Error> {
    // We use a short connection timeout so slow DNS doesn't stall the agent.
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(3))
        .timeout(Duration::from_secs(5))
        .build()?;

    // Just hit the base URL — we only care about TCP connect + first byte.
    let start = Instant::now();
    let resp = client.head(base_url).send().await;
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

/// Returns `true` if the endpoint is reachable within `timeout`.
pub async fn is_reachable(base_url: &str, timeout_ms: u64) -> bool {
    let threshold = Duration::from_millis(timeout_ms);
    match probe_latency(base_url).await {
        Ok(latency) => latency < threshold,
        Err(_) => false,
    }
}
