---
okf_version: "0.2"
type: Function
title: api_status
resource: crates/oxide-gateway/src/routes.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T16:38:18Z"
concept_id: crates/oxide-gateway/src/routes/api_status
language: rust
---

# api_status

## Signature

```rust
pub fn api_status(state: web::Data<Arc<AppState>>) -> impl Responder
```

## Visibility

- `pub`

## Source
Lines 231–239 in `crates/oxide-gateway/src/routes.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [routes](/crates/oxide-gateway/src/routes.md) |
