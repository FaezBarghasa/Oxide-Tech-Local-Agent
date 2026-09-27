---
okf_version: "0.2"
type: Function
title: handle_cargo_clippy
description: "[post(\"/api/cargo/clippy\")]"
resource: crates/api/src/routes/cargo.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/api/src/routes/cargo/handle_cargo_clippy
language: rust
---

# handle_cargo_clippy

[post("/api/cargo/clippy")]

## Signature

```rust
pub fn handle_cargo_clippy(req: web::Json<CargoRequest>) -> impl Responder
```

## Decorators

- `post("/api/cargo/clippy")`

## Visibility

- `pub`

## Docstring

[post("/api/cargo/clippy")]

## Source
Lines 55–63 in `crates/api/src/routes/cargo.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cargo](/crates/api/src/routes/cargo.md) |
| calls | [run_cargo_clippy](/crates/api/src/routes/cargo/run_cargo_clippy.md) |
