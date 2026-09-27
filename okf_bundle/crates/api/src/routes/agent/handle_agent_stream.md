---
okf_version: "0.2"
type: Function
title: handle_agent_stream
description: "[post(\"/api/agent/stream\")]"
resource: crates/api/src/routes/agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:18:37Z"
concept_id: crates/api/src/routes/agent/handle_agent_stream
language: rust
---

# handle_agent_stream

[post("/api/agent/stream")]

## Signature

```rust
pub fn handle_agent_stream(
    req: web::Json<GenerateRequest>,
    cfg: web::Data<AppConfig>,
    rag: web::Data<Option<Arc<RagPipeline>>>,
) -> impl Responder
```

## Decorators

- `post("/api/agent/stream")`

## Visibility

- `pub`

## Docstring

[post("/api/agent/stream")]

## Source
Lines 390–426 in `crates/api/src/routes/agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [agent](/crates/api/src/routes/agent.md) |
