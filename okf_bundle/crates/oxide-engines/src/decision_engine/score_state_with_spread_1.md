---
okf_version: "0.2"
type: Function
title: score_state_with_spread
description: Score state and evaluate speculative cascade thresholds
resource: crates/oxide-engines/src/decision_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T21:48:01Z"
concept_id: crates/oxide-engines/src/decision_engine/score_state_with_spread_1
language: rust
---

# score_state_with_spread

Score state and evaluate speculative cascade thresholds

## Signature

```rust
pub fn score_state_with_spread(
        &self,
        state_embedding: &[f32],
        config: &SpeculativeCascadeConfig,
    ) -> Option<DecisionSpreadResult>
```

## Visibility

- `pub`

## Docstring

Score state and evaluate speculative cascade thresholds

## Source
Lines 225–249 in `crates/oxide-engines/src/decision_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [decision_engine](/crates/oxide-engines/src/decision_engine.md) |
