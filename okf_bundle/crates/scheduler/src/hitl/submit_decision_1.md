---
okf_version: "0.2"
type: Function
title: submit_decision
description: "Resolve a pending HITL request with the operator's decision."
resource: crates/scheduler/src/hitl.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:scheduler"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-20T06:03:07Z"
concept_id: crates/scheduler/src/hitl/submit_decision_1
language: rust
---

# submit_decision

Resolve a pending HITL request with the operator's decision.

## Signature

```rust
pub fn submit_decision(&self, req_id: &str, decision: HitlDecision) -> bool
```

## Visibility

- `pub`

## Docstring

Resolve a pending HITL request with the operator's decision.

## Source
Lines 92–99 in `crates/scheduler/src/hitl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hitl](/crates/scheduler/src/hitl.md) |
