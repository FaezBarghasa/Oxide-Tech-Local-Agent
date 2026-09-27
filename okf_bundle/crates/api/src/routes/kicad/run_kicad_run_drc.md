---
okf_version: "0.2"
type: Function
title: run_kicad_run_drc
resource: crates/api/src/routes/kicad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/api/src/routes/kicad/run_kicad_run_drc
language: rust
---

# run_kicad_run_drc

## Signature

```rust
pub fn run_kicad_run_drc(req: KicadRequest) -> Result<serde_json::Value, String>
```

## Visibility

- `pub`

## Source
Lines 39–73 in `crates/api/src/routes/kicad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [kicad](/crates/api/src/routes/kicad.md) |
| called_by | [handle_kicad_run_drc](/crates/api/src/routes/kicad/handle_kicad_run_drc.md) |
