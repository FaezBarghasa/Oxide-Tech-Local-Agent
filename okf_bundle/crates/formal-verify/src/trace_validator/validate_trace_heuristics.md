---
okf_version: "0.2"
type: Function
title: validate_trace_heuristics
description: Evaluates the soundness and coherence of a reasoning trace against domain constraints
resource: crates/formal-verify/src/trace_validator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:formal-verify"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T11:45:10Z"
concept_id: crates/formal-verify/src/trace_validator/validate_trace_heuristics
language: rust
---

# validate_trace_heuristics

Evaluates the soundness and coherence of a reasoning trace against domain constraints

## Signature

```rust
impl TraceValidator { pub fn validate_trace_heuristics(trace: &ReasoningTrace, domain: &str) -> TraceJudgement }
```

## Visibility

- `pub`

## Docstring

Evaluates the soundness and coherence of a reasoning trace against domain constraints

## Source
Lines 67–107 in `crates/formal-verify/src/trace_validator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [trace_validator](/crates/formal-verify/src/trace_validator.md) |
