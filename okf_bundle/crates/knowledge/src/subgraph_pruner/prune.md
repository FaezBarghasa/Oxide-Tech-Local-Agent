---
okf_version: "0.2"
type: Function
title: prune
description: Extract a minimal induced k-hop subgraph around target_id
resource: crates/knowledge/src/subgraph_pruner.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:knowledge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/knowledge/src/subgraph_pruner/prune
language: rust
---

# prune

Extract a minimal induced k-hop subgraph around target_id

## Signature

```rust
impl SubgraphPruner { pub fn prune(graph: &MultiModalCodeGraph, target_id: &str, k_hops: usize) -> PrunedSubgraph }
```

## Visibility

- `pub`

## Docstring

Extract a minimal induced k-hop subgraph around target_id

## Source
Lines 60–103 in `crates/knowledge/src/subgraph_pruner.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subgraph_pruner](/crates/knowledge/src/subgraph_pruner.md) |
