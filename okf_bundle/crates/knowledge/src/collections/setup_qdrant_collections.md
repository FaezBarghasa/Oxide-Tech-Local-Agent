---
okf_version: "0.2"
type: Function
title: setup_qdrant_collections
resource: crates/knowledge/src/collections.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/knowledge/src/collections/setup_qdrant_collections
language: rust
---

# setup_qdrant_collections

## Signature

```rust
pub fn setup_qdrant_collections(client: &Qdrant) -> Result<(), anyhow::Error>
```

## Visibility

- `pub`

## Source
Lines 17–38 in `crates/knowledge/src/collections.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [collections](/crates/knowledge/src/collections.md) |
| called_by | [new](/crates/knowledge/src/client/new.md) |
