---
okf_version: "0.2"
type: Function
title: handle_cargo_check
description: "[post(\"/api/cargo/check\")]"
resource: crates/api/src/routes/cargo.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/api/src/routes/cargo/handle_cargo_check
language: rust
---

# handle_cargo_check

[post("/api/cargo/check")]

## Signature

```rust
pub fn handle_cargo_check(req: web::Json<CargoRequest>) -> impl Responder
```

## Decorators

- `post("/api/cargo/check")`

## Visibility

- `pub`

## Docstring

[post("/api/cargo/check")]

## Source
Lines 44–52 in `crates/api/src/routes/cargo.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cargo](/crates/api/src/routes/cargo.md) |
| calls | [run_cargo_check](/crates/api/src/routes/cargo/run_cargo_check.md) |
