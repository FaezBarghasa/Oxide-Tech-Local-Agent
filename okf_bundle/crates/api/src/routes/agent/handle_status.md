---
okf_version: "0.2"
type: Function
title: handle_status
description: "[get(\"/api/status\")]"
resource: crates/api/src/routes/agent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T23:18:37Z"
concept_id: crates/api/src/routes/agent/handle_status
language: rust
---

# handle_status

[get("/api/status")]

## Signature

```rust
pub fn handle_status(cfg: web::Data<AppConfig>) -> impl Responder
```

## Decorators

- `get("/api/status")`

## Visibility

- `pub`

## Docstring

[get("/api/status")]

## Source
Lines 384–387 in `crates/api/src/routes/agent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [agent](/crates/api/src/routes/agent.md) |
