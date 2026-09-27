---
okf_version: "0.2"
type: Function
title: liveness_probe
description: "[get(\"/health/live\")]"
resource: crates/api/src/routes/health.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/api/src/routes/health/liveness_probe
language: rust
---

# liveness_probe

[get("/health/live")]

## Signature

```rust
pub fn liveness_probe() -> impl Responder
```

## Decorators

- `get("/health/live")`

## Visibility

- `pub`

## Docstring

[get("/health/live")]

## Source
Lines 7–12 in `crates/api/src/routes/health.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [health](/crates/api/src/routes/health.md) |
