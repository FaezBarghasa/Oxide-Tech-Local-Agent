---
okf_version: "0.2"
type: Function
title: route_agent_states
description: Route messages from N agents through H hubs to other agents
resource: crates/router/src/hub_routing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/hub_routing/route_agent_states
language: rust
---

# route_agent_states

Route messages from N agents through H hubs to other agents

## Signature

```rust
impl SparseHubRouter { pub fn route_agent_states(
        &self,
        agent_states: &Array2<f32>,
        hub_weights: &Array2<f32>,
    ) -> Array2<f32> }
```

## Visibility

- `pub`

## Docstring

Route messages from N agents through H hubs to other agents

## Source
Lines 17–37 in `crates/router/src/hub_routing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hub_routing](/crates/router/src/hub_routing.md) |
| calls | [Axis](/crates/cad-forge/src/kernel/Axis.md) |
