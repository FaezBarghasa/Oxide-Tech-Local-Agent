---
okf_version: "0.2"
type: Function
title: plan_goal
resource: crates/optio/src/persona_loop.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:optio"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/optio/src/persona_loop/plan_goal
language: rust
---

# plan_goal

## Signature

```rust
impl PersonaOrchestrator { pub fn plan_goal(&self, goal: &str, pruned_context: &str) -> Result<PlanOutput> }
```

## Visibility

- `pub`

## Source
Lines 32–78 in `crates/optio/src/persona_loop.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [persona_loop](/crates/optio/src/persona_loop.md) |
