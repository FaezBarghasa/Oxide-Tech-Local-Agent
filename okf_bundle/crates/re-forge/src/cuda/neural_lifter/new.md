---
okf_version: "0.2"
type: Function
title: new
resource: crates/re-forge/src/cuda/neural_lifter.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:re-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:15:46Z"
concept_id: crates/re-forge/src/cuda/neural_lifter/new
language: rust
---

# new

## Signature

```rust
impl NeuralCudaLifter { pub fn new(provider: Arc<dyn InferenceProvider>, target_model: Option<String>) -> Self }
```

## Visibility

- `pub`

## Source
Lines 23–28 in `crates/re-forge/src/cuda/neural_lifter.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [neural_lifter](/crates/re-forge/src/cuda/neural_lifter.md) |
