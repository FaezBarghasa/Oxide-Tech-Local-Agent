---
okf_version: "0.2"
type: Function
title: chat_completion
resource: crates/re-forge/tests/cuda_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:re-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:19:06Z"
concept_id: crates/re-forge/tests/cuda_tests/chat_completion
language: rust
---

# chat_completion

## Signature

```rust
impl MockCudaInferenceProvider { fn chat_completion(&self, _req: ChatRequest) -> Result<ChatResponse> }
```

## Source
Lines 29–39 in `crates/re-forge/tests/cuda_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cuda_tests](/crates/re-forge/tests/cuda_tests.md) |
