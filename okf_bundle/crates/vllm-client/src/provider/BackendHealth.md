---
okf_version: "0.2"
type: Class
title: BackendHealth
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/vllm-client/src/provider.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:vllm-client"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-15T10:57:55Z"
concept_id: crates/vllm-client/src/provider/BackendHealth
language: rust
---

# BackendHealth

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct BackendHealth
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `healthy`
- `provider_name`
- `active_model`
- `memory_used_mb`
- `vram_used_mb`
- `available_slots`
- `queue_depth`

## Source
Lines 202–210 in `crates/vllm-client/src/provider.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [provider](/crates/vllm-client/src/provider.md) |
