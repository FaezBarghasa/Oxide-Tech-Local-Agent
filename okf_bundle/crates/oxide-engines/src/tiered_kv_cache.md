---
okf_version: "0.2"
type: Module
title: tiered_kv_cache
description: "# Tiered DDR5 Host RAM KV-Cache Manager (`crates/oxide-engines/src/tiered_kv_cache.rs`)"
resource: crates/oxide-engines/src/tiered_kv_cache.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:04:39Z"
concept_id: crates/oxide-engines/src/tiered_kv_cache
language: rust
---

# tiered_kv_cache

# Tiered DDR5 Host RAM KV-Cache Manager (`crates/oxide-engines/src/tiered_kv_cache.rs`)

## Docstring

# Tiered DDR5 Host RAM KV-Cache Manager (`crates/oxide-engines/src/tiered_kv_cache.rs`)

Implements hardware-aware memory tiering for high-context autoregressive inference:
- Attention Sinks [0..32] pinned in high-speed GPU VRAM
- Historical tokens [32..S-4096] paged into pinned DDR5 host RAM via async DMA
- Active sliding window [S-4096..S] resident in GPU VRAM for FlashAttention
- Virtual Page Table (VPT) tracking 1,000,000+ token context without VRAM OOM exhaustion.

## Relationships

| Type | Target |
|------|--------|
| related | [KvMemoryTier](/crates/oxide-engines/src/tiered_kv_cache/KvMemoryTier.md) |
| related | [KvCacheConfig](/crates/oxide-engines/src/tiered_kv_cache/KvCacheConfig.md) |
| related | [default](/crates/oxide-engines/src/tiered_kv_cache/default.md) |
| related | [default](/crates/oxide-engines/src/tiered_kv_cache/default.md) |
| related | [bytes_per_token](/crates/oxide-engines/src/tiered_kv_cache/bytes_per_token.md) |
| related | [bytes_per_page](/crates/oxide-engines/src/tiered_kv_cache/bytes_per_page.md) |
| related | [bytes_per_token](/crates/oxide-engines/src/tiered_kv_cache/bytes_per_token.md) |
| related | [bytes_per_page](/crates/oxide-engines/src/tiered_kv_cache/bytes_per_page.md) |
| related | [KvPage](/crates/oxide-engines/src/tiered_kv_cache/KvPage.md) |
| related | [KvTierMetrics](/crates/oxide-engines/src/tiered_kv_cache/KvTierMetrics.md) |
| related | [TieredKvCacheManager](/crates/oxide-engines/src/tiered_kv_cache/TieredKvCacheManager.md) |
| related | [new](/crates/oxide-engines/src/tiered_kv_cache/new.md) |
| related | [append_tokens](/crates/oxide-engines/src/tiered_kv_cache/append_tokens.md) |
| related | [rebalance_tiers](/crates/oxide-engines/src/tiered_kv_cache/rebalance_tiers.md) |
| related | [prefetch_token_range](/crates/oxide-engines/src/tiered_kv_cache/prefetch_token_range.md) |
| related | [get_metrics](/crates/oxide-engines/src/tiered_kv_cache/get_metrics.md) |
| related | [new](/crates/oxide-engines/src/tiered_kv_cache/new.md) |
| related | [append_tokens](/crates/oxide-engines/src/tiered_kv_cache/append_tokens.md) |
| related | [rebalance_tiers](/crates/oxide-engines/src/tiered_kv_cache/rebalance_tiers.md) |
| related | [prefetch_token_range](/crates/oxide-engines/src/tiered_kv_cache/prefetch_token_range.md) |
| related | [get_metrics](/crates/oxide-engines/src/tiered_kv_cache/get_metrics.md) |
| related | [test_tiered_kv_cache_1m_rollover_and_vram_bounds](/crates/oxide-engines/src/tiered_kv_cache/test_tiered_kv_cache_1m_rollover_and_vram_bounds.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
