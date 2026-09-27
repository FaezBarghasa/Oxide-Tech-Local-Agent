---
okf_version: "0.2"
type: Function
title: extract_reasoning_trace
description: "Extracts `<think>...</think>` reasoning trace blocks from raw model generation"
resource: crates/formal-verify/src/trace_validator.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:formal-verify"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T11:45:10Z"
concept_id: crates/formal-verify/src/trace_validator/extract_reasoning_trace
language: rust
---

# extract_reasoning_trace

Extracts `<think>...</think>` reasoning trace blocks from raw model generation

## Signature

```rust
impl TraceValidator { pub fn extract_reasoning_trace(text: &str) -> Option<ReasoningTrace> }
```

## Visibility

- `pub`

## Docstring

Extracts `<think>...</think>` reasoning trace blocks from raw model generation

## Source
Lines 26–64 in `crates/formal-verify/src/trace_validator.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [trace_validator](/crates/formal-verify/src/trace_validator.md) |
