---
okf_version: "0.2"
type: Class
title: BenchmarkScore
description: Metric scores obtained on a benchmark suite.
resource: crates/benchmark-harness/src/harness.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:benchmark-harness"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T11:43:50Z"
concept_id: crates/benchmark-harness/src/harness/BenchmarkScore
language: rust
---

# BenchmarkScore

Metric scores obtained on a benchmark suite.

## Signature

```rust
pub struct BenchmarkScore
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Metric scores obtained on a benchmark suite.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `total_tasks`
- `passed_tasks`
- `accuracy`
- `avg_latency_ms`
- `avg_tokens_used`
- `invariants`

## Source
Lines 20–28 in `crates/benchmark-harness/src/harness.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [harness](/crates/benchmark-harness/src/harness.md) |
