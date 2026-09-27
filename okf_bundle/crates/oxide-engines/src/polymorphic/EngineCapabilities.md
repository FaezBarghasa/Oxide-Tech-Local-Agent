---
okf_version: "0.2"
type: Class
title: EngineCapabilities
description: "[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]"
resource: crates/oxide-engines/src/polymorphic.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:06:13Z"
concept_id: crates/oxide-engines/src/polymorphic/EngineCapabilities
language: rust
---

# EngineCapabilities

[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]

## Signature

```rust
pub struct EngineCapabilities
```

## Decorators

- `derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]

## Methods

- `supports_streaming`
- `supports_lora`
- `supports_speculative`
- `max_context_tokens`
- `supported_quantizations`

## Source
Lines 63–69 in `crates/oxide-engines/src/polymorphic.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [polymorphic](/crates/oxide-engines/src/polymorphic.md) |
