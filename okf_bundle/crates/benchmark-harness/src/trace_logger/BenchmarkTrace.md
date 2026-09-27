---
okf_version: "0.2"
type: Class
title: BenchmarkTrace
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/benchmark-harness/src/trace_logger.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:benchmark-harness"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T11:48:08Z"
concept_id: crates/benchmark-harness/src/trace_logger/BenchmarkTrace
language: rust
---

# BenchmarkTrace

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct BenchmarkTrace
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `suite`
- `task_id`
- `steps_taken`
- `passed`
- `tool_calls`
- `timestamp`

## Source
Lines 7–14 in `crates/benchmark-harness/src/trace_logger.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [trace_logger](/crates/benchmark-harness/src/trace_logger.md) |
