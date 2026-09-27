---
okf_version: "0.2"
type: Function
title: handle_kicad_run_drc
description: "[post(\"/api/kicad/run-drc\")]"
resource: crates/api/src/routes/kicad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/api/src/routes/kicad/handle_kicad_run_drc
language: rust
---

# handle_kicad_run_drc

[post("/api/kicad/run-drc")]

## Signature

```rust
pub fn handle_kicad_run_drc(req: web::Json<KicadRequest>) -> impl Responder
```

## Decorators

- `post("/api/kicad/run-drc")`

## Visibility

- `pub`

## Docstring

[post("/api/kicad/run-drc")]

## Source
Lines 87–95 in `crates/api/src/routes/kicad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [kicad](/crates/api/src/routes/kicad.md) |
| calls | [run_kicad_run_drc](/crates/api/src/routes/kicad/run_kicad_run_drc.md) |
