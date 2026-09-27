---
okf_version: "0.2"
type: Function
title: from_config
description: "Build from a `ModelConfig` (usually `config.thinker`)."
resource: crates/vllm-client/src/thinker.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T10:48:50Z"
concept_id: crates/vllm-client/src/thinker/from_config
language: rust
---

# from_config

Build from a `ModelConfig` (usually `config.thinker`).

## Signature

```rust
impl ThinkerClient { pub fn from_config(cfg: &ModelConfig) -> Self }
```

## Visibility

- `pub`

## Docstring

Build from a `ModelConfig` (usually `config.thinker`).

## Source
Lines 94–98 in `crates/vllm-client/src/thinker.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [thinker](/crates/vllm-client/src/thinker.md) |
