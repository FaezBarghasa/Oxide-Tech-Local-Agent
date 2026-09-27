---
okf_version: "0.2"
type: Function
title: generate
resource: crates/oxide-engines/src/mistral_rs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-engines/src/mistral_rs/generate
language: rust
---

# generate

## Signature

```rust
impl MistralRsProvider { fn generate(
        &self,
        prompt: Vec<ChatMessage>,
        params: GenerationParams,
        token_tx: mpsc::Sender<String>,
    ) -> Result<(), OxideError> }
```

## Source
Lines 57–93 in `crates/oxide-engines/src/mistral_rs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mistral_rs](/crates/oxide-engines/src/mistral_rs.md) |
