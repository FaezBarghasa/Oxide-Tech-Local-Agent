# tiered_kv_cache

## Classs

- [KvCacheConfig](KvCacheConfig.md) — Configuration parameters for tiered KV cache allocation
- [KvMemoryTier](KvMemoryTier.md) — Storage tier for intermediate KV-cache pages
- [KvPage](KvPage.md) — A discrete page of Key-Value tensor blocks
- [KvTierMetrics](KvTierMetrics.md) — Telemetry metrics for KV cache memory footprint
- [TieredKvCacheManager](TieredKvCacheManager.md) — Virtual Page Table & DMA manager for Tiered KV-cache

## Functions

- [append_tokens](append_tokens.md) — Appends new tokens to the KV-cache and orchestrates automatic DDR5 eviction
- [append_tokens](append_tokens_1.md) — Appends new tokens to the KV-cache and orchestrates automatic DDR5 eviction
- [bytes_per_page](bytes_per_page.md) — Calculate byte size for a single page
- [bytes_per_page](bytes_per_page_1.md) — Calculate byte size for a single page
- [bytes_per_token](bytes_per_token.md) — Calculate byte size for a single token's KV cache across all layers
- [bytes_per_token](bytes_per_token_1.md) — Calculate byte size for a single token's KV cache across all layers
- [default](default.md)
- [default](default_1.md)
- [get_metrics](get_metrics.md) — Returns current telemetry snapshot
- [get_metrics](get_metrics_1.md) — Returns current telemetry snapshot
- [new](new.md)
- [new](new_1.md)
- [prefetch_token_range](prefetch_token_range.md) — Prefetches a range of historical tokens from DDR5 back into VRAM
- [prefetch_token_range](prefetch_token_range_1.md) — Prefetches a range of historical tokens from DDR5 back into VRAM
- [rebalance_tiers](rebalance_tiers.md) — Rebalances memory tiers based on Attention Sinks [0..32] and Sliding Window [S-4096..S]
- [rebalance_tiers](rebalance_tiers_1.md) — Rebalances memory tiers based on Attention Sinks [0..32] and Sliding Window [S-4096..S]
- [test_tiered_kv_cache_1m_rollover_and_vram_bounds](test_tiered_kv_cache_1m_rollover_and_vram_bounds.md) — [tokio::test]
