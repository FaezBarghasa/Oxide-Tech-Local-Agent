---
okf_version: "0.2"
type: Function
title: add_component_with_footprint
description: Add a component to the circuit graph with an explicit PCB footprint.
resource: crates/circuit-forge/src/builder.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:circuit-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/circuit-forge/src/builder/add_component_with_footprint
language: rust
---

# add_component_with_footprint

Add a component to the circuit graph with an explicit PCB footprint.

## Signature

```rust
impl CircuitBuilder { pub fn add_component_with_footprint(
        &mut self,
        ref_des: &str,
        lib_id: &str,
        value: &str,
        footprint: &str,
    ) -> &mut Self }
```

## Visibility

- `pub`

## Docstring

Add a component to the circuit graph with an explicit PCB footprint.

## Source
Lines 67–82 in `crates/circuit-forge/src/builder.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [builder](/crates/circuit-forge/src/builder.md) |
