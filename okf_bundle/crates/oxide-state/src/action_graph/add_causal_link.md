---
okf_version: "0.2"
type: Function
title: add_causal_link
description: Link two actions causally.
resource: crates/oxide-state/src/action_graph.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-state"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T22:58:09Z"
concept_id: crates/oxide-state/src/action_graph/add_causal_link
language: rust
---

# add_causal_link

Link two actions causally.

## Signature

```rust
impl ActionGraph { pub fn add_causal_link(
        &mut self,
        from_id: &str,
        to_id: &str,
        edge_type: ActionEdgeType,
    ) -> Result<()> }
```

## Visibility

- `pub`

## Docstring

Link two actions causally.

## Source
Lines 56–75 in `crates/oxide-state/src/action_graph.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [action_graph](/crates/oxide-state/src/action_graph.md) |
