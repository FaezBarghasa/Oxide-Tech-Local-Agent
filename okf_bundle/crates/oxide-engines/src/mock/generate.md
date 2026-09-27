---
okf_version: "0.2"
type: Function
title: generate
resource: crates/oxide-engines/src/mock.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-engines/src/mock/generate
language: rust
---

# generate

## Signature

```rust
impl MockProvider { fn generate(
        &self,
        prompt: Vec<ChatMessage>,
        _params: GenerationParams,
        token_tx: mpsc::Sender<String>,
    ) -> Result<(), OxideError> }
```

## Source
Lines 22–54 in `crates/oxide-engines/src/mock.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mock](/crates/oxide-engines/src/mock.md) |
