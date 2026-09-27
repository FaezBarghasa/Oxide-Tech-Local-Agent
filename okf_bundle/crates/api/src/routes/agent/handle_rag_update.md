---
okf_version: "0.2"
type: Function
title: handle_rag_update
description: "[post(\"/api/rag/update\")]"
resource: crates/api/src/routes/agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:18:37Z"
concept_id: crates/api/src/routes/agent/handle_rag_update
language: rust
---

# handle_rag_update

[post("/api/rag/update")]

## Signature

```rust
pub fn handle_rag_update(
    cfg: web::Data<AppConfig>,
    rag: web::Data<Option<Arc<RagPipeline>>>,
) -> impl Responder
```

## Decorators

- `post("/api/rag/update")`

## Visibility

- `pub`

## Docstring

[post("/api/rag/update")]

## Source
Lines 355–381 in `crates/api/src/routes/agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [agent](/crates/api/src/routes/agent.md) |
| calls | [check_and_update_crates](/crates/rag-pipeline/src/updater/check_and_update_crates.md) |
