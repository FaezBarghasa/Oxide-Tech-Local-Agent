---
okf_version: "0.2"
type: Function
title: get_predecessors
description: Get all causal predecessors of an action node.
resource: crates/oxide-state/src/action_graph.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/action_graph/get_predecessors
language: rust
---

# get_predecessors

Get all causal predecessors of an action node.

## Signature

```rust
impl ActionGraph { pub fn get_predecessors(&self, action_id: &str) -> Vec<&ActionNode> }
```

## Visibility

- `pub`

## Docstring

Get all causal predecessors of an action node.

## Source
Lines 78–89 in `crates/oxide-state/src/action_graph.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [action_graph](/crates/oxide-state/src/action_graph.md) |
