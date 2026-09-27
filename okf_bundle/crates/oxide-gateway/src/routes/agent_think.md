---
okf_version: "0.2"
type: Function
title: agent_think
resource: crates/oxide-gateway/src/routes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T16:38:18Z"
concept_id: crates/oxide-gateway/src/routes/agent_think
language: rust
---

# agent_think

## Signature

```rust
pub fn agent_think(
    state: web::Data<Arc<AppState>>,
    req: web::Json<ThinkRequest>,
) -> impl Responder
```

## Visibility

- `pub`

## Source
Lines 258–316 in `crates/oxide-gateway/src/routes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routes](/crates/oxide-gateway/src/routes.md) |
