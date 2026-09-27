---
okf_version: "0.2"
type: Function
title: detect_optimal_hardware
description: "Dynamically detect host hardware: check for CUDA runtime or AMD Zen CPU topology"
resource: crates/vllm-client/src/candle_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/vllm-client/src/candle_engine/detect_optimal_hardware
language: rust
---

# detect_optimal_hardware

Dynamically detect host hardware: check for CUDA runtime or AMD Zen CPU topology

## Signature

```rust
impl ComputeDevice { pub fn detect_optimal_hardware() -> Self }
```

## Visibility

- `pub`

## Docstring

Dynamically detect host hardware: check for CUDA runtime or AMD Zen CPU topology

## Source
Lines 28–59 in `crates/vllm-client/src/candle_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [candle_engine](/crates/vllm-client/src/candle_engine.md) |
