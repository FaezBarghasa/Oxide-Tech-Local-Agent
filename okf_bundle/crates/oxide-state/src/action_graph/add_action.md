---
okf_version: "0.2"
type: Function
title: add_action
description: Add an action node to the graph.
resource: crates/oxide-state/src/action_graph.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/action_graph/add_action
language: rust
---

# add_action

Add an action node to the graph.

## Signature

```rust
impl ActionGraph { pub fn add_action(&mut self, action: ActionNode) -> NodeIndex }
```

## Visibility

- `pub`

## Docstring

Add an action node to the graph.

## Source
Lines 48–53 in `crates/oxide-state/src/action_graph.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [action_graph](/crates/oxide-state/src/action_graph.md) |
