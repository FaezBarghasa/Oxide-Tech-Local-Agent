---
okf_version: "0.2"
type: Module
title: agent_fsm
description: "# Agent Finite State Machine (FSM)"
resource: crates/router/src/agent_fsm.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:router"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/router/src/agent_fsm
language: rust
---

# agent_fsm

# Agent Finite State Machine (FSM)

## Docstring

# Agent Finite State Machine (FSM)

Enables cyclic dynamic routing inspired by LangGraph conditional edges.
Rather than executing a static linear DAG, agents evaluate state after each step
and dynamically determine the next role or terminate.

## Relationships

| Type | Target |
|------|--------|
| related | [RoutingDecision](/crates/router/src/agent_fsm/RoutingDecision.md) |
| related | [AgentFsmRouter](/crates/router/src/agent_fsm/AgentFsmRouter.md) |
| related | [default](/crates/router/src/agent_fsm/default.md) |
| related | [default](/crates/router/src/agent_fsm/default.md) |
| related | [new](/crates/router/src/agent_fsm/new.md) |
| related | [evaluate_transition](/crates/router/src/agent_fsm/evaluate_transition.md) |
| related | [new](/crates/router/src/agent_fsm/new.md) |
| related | [evaluate_transition](/crates/router/src/agent_fsm/evaluate_transition.md) |
| related | [test_fsm_happy_path_transitions](/crates/router/src/agent_fsm/test_fsm_happy_path_transitions.md) |
| related | [test_fsm_error_triggers_debugger_or_retry](/crates/router/src/agent_fsm/test_fsm_error_triggers_debugger_or_retry.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
