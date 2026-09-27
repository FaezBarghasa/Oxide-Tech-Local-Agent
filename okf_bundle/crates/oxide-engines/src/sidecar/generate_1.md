---
okf_version: "0.2"
type: Function
title: generate
resource: crates/oxide-engines/src/sidecar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-engines/src/sidecar/generate_1
language: rust
---

# generate

## Signature

```rust
fn generate(
        &self,
        prompt: Vec<ChatMessage>,
        params: GenerationParams,
        token_tx: mpsc::Sender<String>,
    ) -> Result<(), OxideError>
```

## Source
Lines 29–100 in `crates/oxide-engines/src/sidecar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sidecar](/crates/oxide-engines/src/sidecar.md) |
