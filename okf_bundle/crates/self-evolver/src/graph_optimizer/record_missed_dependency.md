---
okf_version: "0.2"
type: Function
title: record_missed_dependency
description: Record a missed dependency event and auto-tune traversal weights
resource: crates/self-evolver/src/graph_optimizer.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:self-evolver"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/self-evolver/src/graph_optimizer/record_missed_dependency
language: rust
---

# record_missed_dependency

Record a missed dependency event and auto-tune traversal weights

## Signature

```rust
impl GraphTraversalOptimizer { pub fn record_missed_dependency(&mut self, missed_edge_type: &str) }
```

## Visibility

- `pub`

## Docstring

Record a missed dependency event and auto-tune traversal weights

## Source
Lines 41–64 in `crates/self-evolver/src/graph_optimizer.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [graph_optimizer](/crates/self-evolver/src/graph_optimizer.md) |
