---
okf_version: "0.2"
type: Class
title: RoutingDecision
description: Next routing decision computed dynamically after a task completes or fails.
resource: crates/router/src/agent_fsm.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/agent_fsm/RoutingDecision
language: rust
---

# RoutingDecision

Next routing decision computed dynamically after a task completes or fails.

## Signature

```rust
pub enum RoutingDecision
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq)`

## Visibility

- `pub`

## Docstring

Next routing decision computed dynamically after a task completes or fails.
[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Methods

- `role`
- `task_id`
- `instruction`
- `role`
- `task_id`
- `feedback`
- `attempt`
- `reason`
- `reason`
- `inbox_id`
- `final_summary`
- `error`

## Source
Lines 14–36 in `crates/router/src/agent_fsm.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [agent_fsm](/crates/router/src/agent_fsm.md) |
