---
okf_version: "0.2"
type: Function
title: health_ready
resource: crates/oxide-gateway/src/routes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T16:38:18Z"
concept_id: crates/oxide-gateway/src/routes/health_ready
language: rust
---

# health_ready

## Signature

```rust
pub fn health_ready(state: web::Data<Arc<AppState>>) -> impl Responder
```

## Visibility

- `pub`

## Source
Lines 209–214 in `crates/oxide-gateway/src/routes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routes](/crates/oxide-gateway/src/routes.md) |
