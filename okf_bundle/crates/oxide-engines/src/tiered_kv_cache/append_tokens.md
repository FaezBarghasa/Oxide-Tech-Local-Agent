---
okf_version: "0.2"
type: Function
title: append_tokens
description: Appends new tokens to the KV-cache and orchestrates automatic DDR5 eviction
resource: crates/oxide-engines/src/tiered_kv_cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:04:39Z"
concept_id: crates/oxide-engines/src/tiered_kv_cache/append_tokens
language: rust
---

# append_tokens

Appends new tokens to the KV-cache and orchestrates automatic DDR5 eviction

## Signature

```rust
impl TieredKvCacheManager { pub fn append_tokens(&self, num_tokens: usize) -> Result<(), String> }
```

## Visibility

- `pub`

## Docstring

Appends new tokens to the KV-cache and orchestrates automatic DDR5 eviction

## Source
Lines 119–166 in `crates/oxide-engines/src/tiered_kv_cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tiered_kv_cache](/crates/oxide-engines/src/tiered_kv_cache.md) |
