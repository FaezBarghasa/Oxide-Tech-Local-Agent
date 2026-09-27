---
okf_version: "0.2"
type: Class
title: GraphWeightProfile
description: "Graph Traversal Weight & Edge-Priority Optimizer"
resource: crates/self-evolver/src/graph_optimizer.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/self-evolver/src/graph_optimizer/GraphWeightProfile
language: rust
---

# GraphWeightProfile

Graph Traversal Weight & Edge-Priority Optimizer

## Signature

```rust
pub struct GraphWeightProfile
```

## Decorators

- `derive(Debug, Serialize, Deserialize, Clone)`

## Visibility

- `pub`

## Docstring

Graph Traversal Weight & Edge-Priority Optimizer
[derive(Debug, Serialize, Deserialize, Clone)]

## Methods

- `call_edge_weight`
- `data_flow_weight`
- `dependency_weight`
- `ast_scope_weight`
- `max_traversal_hops`

## Source
Lines 7–13 in `crates/self-evolver/src/graph_optimizer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [graph_optimizer](/crates/self-evolver/src/graph_optimizer.md) |
