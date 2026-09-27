---
okf_version: "0.2"
type: Class
title: KvCacheConfig
description: Configuration parameters for tiered KV cache allocation
resource: crates/oxide-engines/src/tiered_kv_cache.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:04:39Z"
concept_id: crates/oxide-engines/src/tiered_kv_cache/KvCacheConfig
language: rust
---

# KvCacheConfig

Configuration parameters for tiered KV cache allocation

## Signature

```rust
pub struct KvCacheConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Configuration parameters for tiered KV cache allocation
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `attention_sink_tokens`
- `sliding_window_tokens`
- `page_size_tokens`
- `layer_count`
- `kv_head_count`
- `head_dim`
- `bytes_per_elem`
- `max_sequence_len`

## Source
Lines 27–44 in `crates/oxide-engines/src/tiered_kv_cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tiered_kv_cache](/crates/oxide-engines/src/tiered_kv_cache.md) |
