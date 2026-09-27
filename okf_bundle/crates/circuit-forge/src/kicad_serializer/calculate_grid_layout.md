---
okf_version: "0.2"
type: Function
title: calculate_grid_layout
description: "Calculate collision-free grid coordinates (x, y) for all components in the circuit."
resource: crates/circuit-forge/src/kicad_serializer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:circuit-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/circuit-forge/src/kicad_serializer/calculate_grid_layout
language: rust
---

# calculate_grid_layout

Calculate collision-free grid coordinates (x, y) for all components in the circuit.

## Signature

```rust
pub fn calculate_grid_layout(graph: &CircuitGraph) -> HashMap<NodeIndex, (f64, f64)>
```

## Visibility

- `pub`

## Docstring

Calculate collision-free grid coordinates (x, y) for all components in the circuit.

## Source
Lines 16–35 in `crates/circuit-forge/src/kicad_serializer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [kicad_serializer](/crates/circuit-forge/src/kicad_serializer.md) |
| called_by | [serialize_to_kicad_sch](/crates/circuit-forge/src/kicad_serializer/serialize_to_kicad_sch.md) |
| called_by | [test_kicad_s_expression_serialization](/crates/circuit-forge/tests/circuit_tests/test_kicad_s_expression_serialization.md) |
