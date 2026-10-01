use std::sync::atomic::{AtomicU64, AtomicU32, Ordering};
use std::sync::Arc;
use dashmap::DashMap;

#[derive(Debug, Clone)]
pub struct ProviderRateState {
    pub consecutive_429s: Arc<AtomicU32>,
    pub cooldown_until_ms: Arc<AtomicU64>,
    pub base_backoff_secs: u64,
    pub max_backoff_secs: u64,
}

impl ProviderRateState {
    pub fn new() -> Self {
        Self {
            consecutive_429s: Arc::new(AtomicU32::new(0)),
            cooldown_until_ms: Arc::new(AtomicU64::new(0)),
            base_backoff_secs: 2,
            max_backoff_secs: 120,
        }
    }

    pub fn is_available(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        now >= self.cooldown_until_ms.load(Ordering::Relaxed)
    }

    pub fn record_success(&self) {
        self.consecutive_429s.store(0, Ordering::Relaxed);
    }

    pub fn record_429(&self) -> u64 {
        let count = self.consecutive_429s.fetch_add(1, Ordering::Relaxed) + 1;
        let factor = 2u64.pow(count.min(6));
        let backoff_secs = (self.base_backoff_secs * factor).min(self.max_backoff_secs);
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        let cooldown_until = now + (backoff_secs * 1000);
        self.cooldown_until_ms.store(cooldown_until, Ordering::Relaxed);

        backoff_secs
    }
}

pub struct RatePacer {
    providers: Arc<DashMap<String, ProviderRateState>>,
}

impl RatePacer {
    pub fn new() -> Self {
        Self {
            providers: Arc::new(DashMap::new()),
        }
    }

    pub fn is_provider_available(&self, provider: &str) -> bool {
        let entry = self.providers.entry(provider.to_string()).or_insert_with(ProviderRateState::new);
        entry.is_available()
    }

    pub fn report_429(&self, provider: &str) -> u64 {
        let entry = self.providers.entry(provider.to_string()).or_insert_with(ProviderRateState::new);
        entry.record_429()
    }

    pub fn report_success(&self, provider: &str) {
        if let Some(entry) = self.providers.get(provider) {
            entry.record_success();
        }
    }
}
