---
okf_version: "0.2"
type: Function
title: route
resource: crates/router/src/local_first_router.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/local_first_router/route
language: rust
---

# route

## Signature

```rust
impl LocalFirstRouter { pub fn route(&self, req: InferenceRequest) -> Result<serde_json::Value, anyhow::Error> }
```

## Visibility

- `pub`

## Source
Lines 152–205 in `crates/router/src/local_first_router.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [local_first_router](/crates/router/src/local_first_router.md) |
