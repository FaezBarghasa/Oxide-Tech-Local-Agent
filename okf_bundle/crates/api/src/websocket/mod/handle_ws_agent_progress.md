---
okf_version: "0.2"
type: Function
title: handle_ws_agent_progress
description: "[get(\"/ws/agent-progress\")]"
resource: crates/api/src/websocket/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/api/src/websocket/mod/handle_ws_agent_progress
language: rust
---

# handle_ws_agent_progress

[get("/ws/agent-progress")]

## Signature

```rust
pub fn handle_ws_agent_progress(
    req: HttpRequest,
    body: web::Payload,
    broadcaster: web::Data<Arc<WsBroadcaster>>,
) -> Result<HttpResponse, Error>
```

## Decorators

- `get("/ws/agent-progress")`

## Visibility

- `pub`

## Docstring

[get("/ws/agent-progress")]

## Source
Lines 61–89 in `crates/api/src/websocket/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [websocket](/crates/api/src/websocket/mod.md) |
