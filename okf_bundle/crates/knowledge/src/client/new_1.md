---
okf_version: "0.2"
type: Function
title: new
resource: crates/knowledge/src/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T16:29:50Z"
concept_id: crates/knowledge/src/client/new_1
language: rust
---

# new

## Signature

```rust
pub fn new() -> Result<Self>
```

## Visibility

- `pub`

## Source
Lines 34–60 in `crates/knowledge/src/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/knowledge/src/client.md) |
| calls | [setup_qdrant_collections](/crates/knowledge/src/collections/setup_qdrant_collections.md) |
