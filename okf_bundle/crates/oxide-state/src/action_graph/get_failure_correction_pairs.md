---
okf_version: "0.2"
type: Function
title: get_failure_correction_pairs
description: Find all actions that failed and their subsequent corrections.
resource: crates/oxide-state/src/action_graph.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/action_graph/get_failure_correction_pairs
language: rust
---

# get_failure_correction_pairs

Find all actions that failed and their subsequent corrections.

## Signature

```rust
impl ActionGraph { pub fn get_failure_correction_pairs(&self) -> Vec<(&ActionNode, &ActionNode)> }
```

## Visibility

- `pub`

## Docstring

Find all actions that failed and their subsequent corrections.

## Source
Lines 97–110 in `crates/oxide-state/src/action_graph.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [action_graph](/crates/oxide-state/src/action_graph.md) |
