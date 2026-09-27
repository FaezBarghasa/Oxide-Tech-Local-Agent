---
okf_version: "0.2"
type: Function
title: evaluate_transition
description: Evaluates current execution context and routes to the next state.
resource: crates/router/src/agent_fsm.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/agent_fsm/evaluate_transition_1
language: rust
---

# evaluate_transition

Evaluates current execution context and routes to the next state.

## Signature

```rust
pub fn evaluate_transition(
        &self,
        current_role: SubAgentRole,
        task_id: &str,
        result: Result<&TaskResult, &str>,
        retry_count: usize,
    ) -> RoutingDecision
```

## Visibility

- `pub`

## Docstring

Evaluates current execution context and routes to the next state.

## Source
Lines 55–140 in `crates/router/src/agent_fsm.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [agent_fsm](/crates/router/src/agent_fsm.md) |
