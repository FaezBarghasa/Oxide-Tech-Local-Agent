---
okf_version: "0.2"
type: Class
title: PrunedSubgraph
description: "A pruned, self-contained sub-graph suitable for LLM prompt context injection"
resource: crates/knowledge/src/subgraph_pruner.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/knowledge/src/subgraph_pruner/PrunedSubgraph
language: rust
---

# PrunedSubgraph

A pruned, self-contained sub-graph suitable for LLM prompt context injection

## Signature

```rust
pub struct PrunedSubgraph
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

A pruned, self-contained sub-graph suitable for LLM prompt context injection
[derive(Debug, Clone)]

## Methods

- `center_node`
- `neighbor_nodes`
- `internal_edges`
- `estimated_tokens`

## Source
Lines 6–11 in `crates/knowledge/src/subgraph_pruner.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subgraph_pruner](/crates/knowledge/src/subgraph_pruner.md) |
