---
okf_version: "0.2"
type: Function
title: rebalance_tiers
description: "Rebalances memory tiers based on Attention Sinks [0..32] and Sliding Window [S-4096..S]"
resource: crates/oxide-engines/src/tiered_kv_cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:04:39Z"
concept_id: crates/oxide-engines/src/tiered_kv_cache/rebalance_tiers
language: rust
---

# rebalance_tiers

Rebalances memory tiers based on Attention Sinks [0..32] and Sliding Window [S-4096..S]

## Signature

```rust
impl TieredKvCacheManager { fn rebalance_tiers(
        &self,
        pages: &mut HashMap<u32, KvPage>,
        seq_len: usize,
        bytes_per_page: usize,
    ) }
```

## Docstring

Rebalances memory tiers based on Attention Sinks [0..32] and Sliding Window [S-4096..S]

## Source
Lines 169–220 in `crates/oxide-engines/src/tiered_kv_cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tiered_kv_cache](/crates/oxide-engines/src/tiered_kv_cache.md) |
