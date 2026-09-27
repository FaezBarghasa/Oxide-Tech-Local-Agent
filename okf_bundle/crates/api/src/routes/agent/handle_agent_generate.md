---
okf_version: "0.2"
type: Function
title: handle_agent_generate
description: "[post(\"/api/agent/generate\")]"
resource: crates/api/src/routes/agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:18:37Z"
concept_id: crates/api/src/routes/agent/handle_agent_generate
language: rust
---

# handle_agent_generate

[post("/api/agent/generate")]

## Signature

```rust
pub fn handle_agent_generate(
    req: web::Json<GenerateRequest>,
    cfg: web::Data<AppConfig>,
    rag: web::Data<Option<Arc<RagPipeline>>>,
) -> impl Responder
```

## Decorators

- `post("/api/agent/generate")`

## Visibility

- `pub`

## Docstring

[post("/api/agent/generate")]

## Source
Lines 339–352 in `crates/api/src/routes/agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [agent](/crates/api/src/routes/agent.md) |
| calls | [run_agent_generate](/crates/api/src/routes/agent/run_agent_generate.md) |
