---
okf_version: "0.2"
type: Function
title: from_config
description: "Construct from a `ModelConfig` slice (parsed from `config.toml`)."
resource: crates/vllm-client/src/client.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-14T14:17:24Z"
concept_id: crates/vllm-client/src/client/from_config
language: rust
---

# from_config

Construct from a `ModelConfig` slice (parsed from `config.toml`).

## Signature

```rust
impl LlmRouterClient { pub fn from_config(cfg: &ModelConfig) -> Self }
```

## Visibility

- `pub`

## Docstring

Construct from a `ModelConfig` slice (parsed from `config.toml`).

## Source
Lines 168–179 in `crates/vllm-client/src/client.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [client](/crates/vllm-client/src/client.md) |
