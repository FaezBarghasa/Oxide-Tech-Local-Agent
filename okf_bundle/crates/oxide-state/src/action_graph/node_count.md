---
okf_version: "0.2"
type: Function
title: node_count
description: Return total nodes in the action graph.
resource: crates/oxide-state/src/action_graph.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/action_graph/node_count
language: rust
---

# node_count

Return total nodes in the action graph.

## Signature

```rust
impl ActionGraph { pub fn node_count(&self) -> usize }
```

## Visibility

- `pub`

## Docstring

Return total nodes in the action graph.

## Source
Lines 92–94 in `crates/oxide-state/src/action_graph.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [action_graph](/crates/oxide-state/src/action_graph.md) |
