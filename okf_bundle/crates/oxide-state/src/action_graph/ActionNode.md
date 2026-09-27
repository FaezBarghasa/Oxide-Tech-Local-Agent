---
okf_version: "0.2"
type: Class
title: ActionNode
description: An atomic action performed by the agent or sandbox.
resource: crates/oxide-state/src/action_graph.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/action_graph/ActionNode
language: rust
---

# ActionNode

An atomic action performed by the agent or sandbox.

## Signature

```rust
pub struct ActionNode
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

An atomic action performed by the agent or sandbox.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `id`
- `action_type`
- `description`
- `payload`
- `success`
- `timestamp`

## Source
Lines 10–17 in `crates/oxide-state/src/action_graph.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [action_graph](/crates/oxide-state/src/action_graph.md) |
