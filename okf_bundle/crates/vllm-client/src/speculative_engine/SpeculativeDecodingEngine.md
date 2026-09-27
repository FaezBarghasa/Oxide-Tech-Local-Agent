---
okf_version: "0.2"
type: Class
title: SpeculativeDecodingEngine
description: Orchestrator for local speculative decoding pairing a fast draft model with a larger target model.
resource: crates/vllm-client/src/speculative_engine.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/speculative_engine/SpeculativeDecodingEngine
language: rust
---

# SpeculativeDecodingEngine

Orchestrator for local speculative decoding pairing a fast draft model with a larger target model.

## Signature

```rust
pub struct SpeculativeDecodingEngine
```

## Visibility

- `pub`

## Docstring

Orchestrator for local speculative decoding pairing a fast draft model with a larger target model.

## Methods

- `draft_provider`
- `target_provider`
- `draft_steps`

## Source
Lines 7–11 in `crates/vllm-client/src/speculative_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [speculative_engine](/crates/vllm-client/src/speculative_engine.md) |
