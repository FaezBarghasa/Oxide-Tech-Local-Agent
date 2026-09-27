---
okf_version: "0.2"
type: Function
title: health_check
resource: crates/qdrant-service/src/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:qdrant-service"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T16:28:56Z"
concept_id: crates/qdrant-service/src/client/health_check
language: rust
---

# health_check

## Signature

```rust
impl QdrantServiceClient { pub fn health_check(&self) -> Result<(), anyhow::Error> }
```

## Visibility

- `pub`

## Source
Lines 30–33 in `crates/qdrant-service/src/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/qdrant-service/src/client.md) |
