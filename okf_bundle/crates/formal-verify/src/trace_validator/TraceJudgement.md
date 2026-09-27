---
okf_version: "0.2"
type: Class
title: TraceJudgement
description: Judgement outcome from TraceValidator (LLM-as-judge)
resource: crates/formal-verify/src/trace_validator.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:formal-verify"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T11:45:10Z"
concept_id: crates/formal-verify/src/trace_validator/TraceJudgement
language: rust
---

# TraceJudgement

Judgement outcome from TraceValidator (LLM-as-judge)

## Signature

```rust
pub struct TraceJudgement
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Judgement outcome from TraceValidator (LLM-as-judge)
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `is_sound`
- `logical_coherence_score`
- `identified_hallucinations`
- `rationale`

## Source
Lines 15–20 in `crates/formal-verify/src/trace_validator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [trace_validator](/crates/formal-verify/src/trace_validator.md) |
