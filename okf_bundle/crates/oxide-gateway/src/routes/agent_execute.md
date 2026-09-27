---
okf_version: "0.2"
type: Function
title: agent_execute
resource: crates/oxide-gateway/src/routes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T16:38:18Z"
concept_id: crates/oxide-gateway/src/routes/agent_execute
language: rust
---

# agent_execute

## Signature

```rust
pub fn agent_execute(
    _state: web::Data<Arc<AppState>>,
    req: web::Json<ExecuteRequest>,
) -> impl Responder
```

## Visibility

- `pub`

## Source
Lines 324–334 in `crates/oxide-gateway/src/routes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routes](/crates/oxide-gateway/src/routes.md) |
