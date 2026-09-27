---
okf_version: "0.2"
type: Function
title: calculate_ipc2152_max_current
description: "Calculate maximum trace current capacity using IPC-2152 formula:"
resource: crates/circuit-forge/src/erc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:circuit-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:31:54Z"
concept_id: crates/circuit-forge/src/erc/calculate_ipc2152_max_current
language: rust
---

# calculate_ipc2152_max_current

Calculate maximum trace current capacity using IPC-2152 formula:

## Signature

```rust
pub fn calculate_ipc2152_max_current(
    trace_width_mils: f64,
    copper_thickness_oz: f64,
    temp_rise_c: f64,
) -> f64
```

## Visibility

- `pub`

## Docstring

Calculate maximum trace current capacity using IPC-2152 formula:
I = k * (delta_t)^beta * (area)^gamma
For external layers: k = 0.048, beta = 0.44, gamma = 0.725

## Source
Lines 268–279 in `crates/circuit-forge/src/erc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [erc](/crates/circuit-forge/src/erc.md) |
| called_by | [test_ipc2152_current_calculation](/crates/circuit-forge/src/erc/test_ipc2152_current_calculation.md) |
