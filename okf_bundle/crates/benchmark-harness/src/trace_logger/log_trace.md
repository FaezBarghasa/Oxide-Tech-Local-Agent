---
okf_version: "0.2"
type: Function
title: log_trace
resource: crates/benchmark-harness/src/trace_logger.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:benchmark-harness"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T11:48:08Z"
concept_id: crates/benchmark-harness/src/trace_logger/log_trace
language: rust
---

# log_trace

## Signature

```rust
impl TraceLogger { pub fn log_trace(&self, trace: &BenchmarkTrace) -> anyhow::Result<()> }
```

## Visibility

- `pub`

## Source
Lines 25–39 in `crates/benchmark-harness/src/trace_logger.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [trace_logger](/crates/benchmark-harness/src/trace_logger.md) |
