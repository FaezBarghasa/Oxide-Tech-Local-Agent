---
okf_version: "0.2"
type: Function
title: handle_kicad_load_board
description: "[post(\"/api/kicad/load-board\")]"
resource: crates/api/src/routes/kicad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/api/src/routes/kicad/handle_kicad_load_board
language: rust
---

# handle_kicad_load_board

[post("/api/kicad/load-board")]

## Signature

```rust
pub fn handle_kicad_load_board(req: web::Json<KicadRequest>) -> impl Responder
```

## Decorators

- `post("/api/kicad/load-board")`

## Visibility

- `pub`

## Docstring

[post("/api/kicad/load-board")]

## Source
Lines 76–84 in `crates/api/src/routes/kicad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [kicad](/crates/api/src/routes/kicad.md) |
| calls | [run_kicad_load_board](/crates/api/src/routes/kicad/run_kicad_load_board.md) |
