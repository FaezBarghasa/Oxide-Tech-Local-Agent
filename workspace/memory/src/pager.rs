use anyhow::{anyhow, Result};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Status of a memory page in the virtual memory hierarchy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PageTier {
    Hot,   // In-memory LRU cache
    Warm,  // SurrealDB working context table
    Cold,  // Qdrant vector archive
}

/// A discrete unit of virtualized conversational or task context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualPage {
    pub page_id: String,
    pub session_id: String,
    pub tier: PageTier,
    pub token_count: usize,
    pub content: String,
    pub last_accessed: i64,
}

/// Manages hierarchical virtual paging between fast memory and archival stores.
pub struct MemoryPager {
    max_hot_tokens: usize,
    current_hot_tokens: usize,
    lru_cache: LruCache<String, VirtualPage>,
}

impl MemoryPager {
    pub fn new(max_hot_tokens: usize, max_hot_pages: usize) -> Self {
        let capacity = NonZeroUsize::new(max_hot_pages).unwrap_or(NonZeroUsize::new(32).unwrap());
        Self {
            max_hot_tokens,
            current_hot_tokens: 0,
            lru_cache: LruCache::new(capacity),
        }
    }

    /// Load or retrieve a page into the Hot tier, evicting older pages if token budget exceeded.
    pub fn access_page(&mut self, page_id: &str) -> Option<VirtualPage> {
        if let Some(page) = self.lru_cache.get_mut(page_id) {
            page.last_accessed = chrono::Utc::now().timestamp();
            return Some(page.clone());
        }
        None
    }

    /// Insert or promote a page to the Hot tier.
    pub fn put_hot_page(&mut self, mut page: VirtualPage) -> Vec<VirtualPage> {
        let mut evicted = Vec::new();

        // Evict until we have space for the new page
        while self.current_hot_tokens + page.token_count > self.max_hot_tokens && !self.lru_cache.is_empty() {
            if let Some((_k, mut oldest)) = self.lru_cache.pop_lru() {
                self.current_hot_tokens = self.current_hot_tokens.saturating_sub(oldest.token_count);
                oldest.tier = PageTier::Warm;
                info!("Evicting virtual page {} to Warm tier", oldest.page_id);
                evicted.push(oldest);
            }
        }

        page.tier = PageTier::Hot;
        page.last_accessed = chrono::Utc::now().timestamp();
        self.current_hot_tokens += page.token_count;
        self.lru_cache.put(page.page_id.clone(), page);

        evicted
    }

    pub fn current_tokens(&self) -> usize {
        self.current_hot_tokens
    }

    pub fn hot_pages_count(&self) -> usize {
        self.lru_cache.len()
    }
}

/// Shared thread-safe virtual pager.
pub type SharedMemoryPager = Arc<RwLock<MemoryPager>>;
