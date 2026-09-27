---
okf_version: "0.2"
type: Function
title: readiness_probe
description: "[get(\"/health/ready\")]"
resource: crates/api/src/routes/health.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/api/src/routes/health/readiness_probe
language: rust
---

# readiness_probe

[get("/health/ready")]

## Signature

```rust
pub fn readiness_probe() -> impl Responder
```

## Decorators

- `get("/health/ready")`

## Visibility

- `pub`

## Docstring

[get("/health/ready")]

## Source
Lines 15–79 in `crates/api/src/routes/health.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [health](/crates/api/src/routes/health.md) |
