---
okf_version: "0.2"
type: Function
title: search_datasheets
resource: crates/qdrant-service/src/collections/datasheets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:qdrant-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/qdrant-service/src/collections/datasheets/search_datasheets_1
language: rust
---

# search_datasheets

## Signature

```rust
pub fn search_datasheets(
        &self,
        dense_vec: Vec<f32>,
        _mcu_type: &str,
        top_k: usize,
    ) -> Result<Vec<serde_json::Value>, anyhow::Error>
```

## Visibility

- `pub`

## Source
Lines 36–60 in `crates/qdrant-service/src/collections/datasheets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [datasheets](/crates/qdrant-service/src/collections/datasheets.md) |
