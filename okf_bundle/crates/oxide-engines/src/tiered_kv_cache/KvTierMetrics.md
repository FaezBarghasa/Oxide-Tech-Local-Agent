---
okf_version: "0.2"
type: Class
title: KvTierMetrics
description: Telemetry metrics for KV cache memory footprint
resource: crates/oxide-engines/src/tiered_kv_cache.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:04:39Z"
concept_id: crates/oxide-engines/src/tiered_kv_cache/KvTierMetrics
language: rust
---

# KvTierMetrics

Telemetry metrics for KV cache memory footprint

## Signature

```rust
pub struct KvTierMetrics
```

## Decorators

- `derive(Debug, Clone, Default, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Telemetry metrics for KV cache memory footprint
[derive(Debug, Clone, Default, Serialize, Deserialize)]

## Methods

- `total_tokens`
- `gpu_vram_tokens`
- `host_ddr5_tokens`
- `gpu_vram_bytes`
- `host_ddr5_bytes`
- `total_pages`
- `paged_out_count`
- `prefetch_hit_count`

## Source
Lines 89–98 in `crates/oxide-engines/src/tiered_kv_cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tiered_kv_cache](/crates/oxide-engines/src/tiered_kv_cache.md) |
