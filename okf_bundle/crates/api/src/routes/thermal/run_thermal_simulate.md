---
okf_version: "0.2"
type: Function
title: run_thermal_simulate
resource: crates/api/src/routes/thermal.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/api/src/routes/thermal/run_thermal_simulate
language: rust
---

# run_thermal_simulate

## Signature

```rust
pub fn run_thermal_simulate(req: ThermalRequest) -> Result<serde_json::Value, String>
```

## Visibility

- `pub`

## Source
Lines 10–37 in `crates/api/src/routes/thermal.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [thermal](/crates/api/src/routes/thermal.md) |
| called_by | [handle_thermal_simulate](/crates/api/src/routes/thermal/handle_thermal_simulate.md) |
