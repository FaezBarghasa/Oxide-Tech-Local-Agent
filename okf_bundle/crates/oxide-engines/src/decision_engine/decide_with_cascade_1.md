---
okf_version: "0.2"
type: Function
title: decide_with_cascade
description: Synchronous decision call with explicit speculative cascading policy
resource: crates/oxide-engines/src/decision_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:48:01Z"
concept_id: crates/oxide-engines/src/decision_engine/decide_with_cascade_1
language: rust
---

# decide_with_cascade

Synchronous decision call with explicit speculative cascading policy

## Signature

```rust
pub fn decide_with_cascade(
        &self,
        input: DecisionInput,
        cascade_config: SpeculativeCascadeConfig,
    ) -> Result<DecisionOutput, String>
```

## Visibility

- `pub`

## Docstring

Synchronous decision call with explicit speculative cascading policy

## Source
Lines 100–119 in `crates/oxide-engines/src/decision_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [decision_engine](/crates/oxide-engines/src/decision_engine.md) |
