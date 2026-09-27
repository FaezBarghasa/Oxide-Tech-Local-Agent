---
okf_version: "0.2"
type: Class
title: SimThermalFeaResult
description: "Output of `sim.run_fea_thermal`"
resource: crates/oxide-protocol/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T14:30:29Z"
concept_id: crates/oxide-protocol/src/lib/SimThermalFeaResult
language: rust
---

# SimThermalFeaResult

Output of `sim.run_fea_thermal`

## Signature

```rust
pub struct SimThermalFeaResult
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Output of `sim.run_fea_thermal`
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `max_temperature_c`
- `silicon_junction_temp_c`
- `hot_spots`
- `passes_threshold`

## Source
Lines 163–168 in `crates/oxide-protocol/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-protocol/src/lib.md) |
