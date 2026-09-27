---
okf_version: "0.2"
type: Function
title: metrics_endpoint
description: "[get(\"/metrics\")]"
resource: crates/api/src/routes/health.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/api/src/routes/health/metrics_endpoint
language: rust
---

# metrics_endpoint

[get("/metrics")]

## Signature

```rust
pub fn metrics_endpoint() -> impl Responder
```

## Decorators

- `get("/metrics")`

## Visibility

- `pub`

## Docstring

[get("/metrics")]

## Source
Lines 82–94 in `crates/api/src/routes/health.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [health](/crates/api/src/routes/health.md) |
| calls | [encode](/crates/oxide-network/src/crypto/encode.md) |
