---
okf_version: "0.2"
type: Function
title: prefetch_token_range
description: Prefetches a range of historical tokens from DDR5 back into VRAM
resource: crates/oxide-engines/src/tiered_kv_cache.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:04:39Z"
concept_id: crates/oxide-engines/src/tiered_kv_cache/prefetch_token_range
language: rust
---

# prefetch_token_range

Prefetches a range of historical tokens from DDR5 back into VRAM

## Signature

```rust
impl TieredKvCacheManager { pub fn prefetch_token_range(
        &self,
        start_token: usize,
        end_token: usize,
    ) -> Result<usize, String> }
```

## Visibility

- `pub`

## Docstring

Prefetches a range of historical tokens from DDR5 back into VRAM

## Source
Lines 223–248 in `crates/oxide-engines/src/tiered_kv_cache.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tiered_kv_cache](/crates/oxide-engines/src/tiered_kv_cache.md) |
