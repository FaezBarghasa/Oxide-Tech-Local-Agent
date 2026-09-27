---
okf_version: "0.2"
type: Function
title: run_erc
description: Run deterministic Electrical Rule Checking (ERC) on a circuit bipartite graph.
resource: crates/circuit-forge/src/erc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:circuit-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:31:54Z"
concept_id: crates/circuit-forge/src/erc/run_erc
language: rust
---

# run_erc

Run deterministic Electrical Rule Checking (ERC) on a circuit bipartite graph.

## Signature

```rust
pub fn run_erc(graph: &CircuitGraph) -> ErcReport
```

## Visibility

- `pub`

## Docstring

Run deterministic Electrical Rule Checking (ERC) on a circuit bipartite graph.

## Source
Lines 71–263 in `crates/circuit-forge/src/erc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [erc](/crates/circuit-forge/src/erc.md) |
| called_by | [test_erc_floating_net_detection](/crates/circuit-forge/tests/circuit_tests/test_erc_floating_net_detection.md) |
| called_by | [test_erc_isolated_component](/crates/circuit-forge/tests/circuit_tests/test_erc_isolated_component.md) |
| called_by | [test_erc_missing_decoupling_cap](/crates/circuit-forge/tests/circuit_tests/test_erc_missing_decoupling_cap.md) |
| called_by | [test_fluent_builder_and_graph_topology](/crates/circuit-forge/tests/circuit_tests/test_fluent_builder_and_graph_topology.md) |
