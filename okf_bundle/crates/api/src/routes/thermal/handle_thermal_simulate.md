---
okf_version: "0.2"
type: Function
title: handle_thermal_simulate
description: "[post(\"/api/thermal/simulate\")]"
resource: crates/api/src/routes/thermal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/api/src/routes/thermal/handle_thermal_simulate
language: rust
---

# handle_thermal_simulate

[post("/api/thermal/simulate")]

## Signature

```rust
pub fn handle_thermal_simulate(req: web::Json<ThermalRequest>) -> impl Responder
```

## Decorators

- `post("/api/thermal/simulate")`

## Visibility

- `pub`

## Docstring

[post("/api/thermal/simulate")]

## Source
Lines 40–48 in `crates/api/src/routes/thermal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [thermal](/crates/api/src/routes/thermal.md) |
| calls | [run_thermal_simulate](/crates/api/src/routes/thermal/run_thermal_simulate.md) |
