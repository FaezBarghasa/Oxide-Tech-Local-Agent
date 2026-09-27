---
okf_version: "0.2"
type: Function
title: serialize_to_kicad_sch
description: "Serialize a verified `CircuitGraph` into KiCad 8 `.kicad_sch` S-expression format."
resource: crates/circuit-forge/src/kicad_serializer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:circuit-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/circuit-forge/src/kicad_serializer/serialize_to_kicad_sch
language: rust
---

# serialize_to_kicad_sch

Serialize a verified `CircuitGraph` into KiCad 8 `.kicad_sch` S-expression format.

## Signature

```rust
pub fn serialize_to_kicad_sch(graph: &CircuitGraph) -> String
```

## Visibility

- `pub`

## Docstring

Serialize a verified `CircuitGraph` into KiCad 8 `.kicad_sch` S-expression format.

## Source
Lines 38–138 in `crates/circuit-forge/src/kicad_serializer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [kicad_serializer](/crates/circuit-forge/src/kicad_serializer.md) |
| calls | [calculate_grid_layout](/crates/circuit-forge/src/kicad_serializer/calculate_grid_layout.md) |
| called_by | [test_kicad_s_expression_serialization](/crates/circuit-forge/tests/circuit_tests/test_kicad_s_expression_serialization.md) |
