---
okf_version: "0.2"
type: Class
title: ReasoningTrace
description: Extracted reasoning trace and step breakdown
resource: crates/formal-verify/src/trace_validator.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:formal-verify"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T11:45:10Z"
concept_id: crates/formal-verify/src/trace_validator/ReasoningTrace
language: rust
---

# ReasoningTrace

Extracted reasoning trace and step breakdown

## Signature

```rust
pub struct ReasoningTrace
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Extracted reasoning trace and step breakdown
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `raw_thinking`
- `step_sequence`
- `token_estimate`
- `has_reflection`
- `has_backtracking`

## Source
Lines 5–11 in `crates/formal-verify/src/trace_validator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [trace_validator](/crates/formal-verify/src/trace_validator.md) |
