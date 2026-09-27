---
okf_version: "0.2"
type: Function
title: new
resource: crates/vllm-client/src/speculative_engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/speculative_engine/new
language: rust
---

# new

## Signature

```rust
impl SpeculativeDecodingEngine { pub fn new(
        draft_provider: Arc<dyn InferenceProvider>,
        target_provider: Arc<dyn InferenceProvider>,
        draft_steps: usize,
    ) -> Self }
```

## Visibility

- `pub`

## Source
Lines 14–24 in `crates/vllm-client/src/speculative_engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [speculative_engine](/crates/vllm-client/src/speculative_engine.md) |
