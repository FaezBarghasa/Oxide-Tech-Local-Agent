---
okf_version: "0.2"
type: Class
title: AdapterBuild
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/model-trainer/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T20:59:29Z"
concept_id: crates/model-trainer/src/lib/AdapterBuild
language: rust
---

# AdapterBuild

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct AdapterBuild
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `adapter_id`
- `base_model`
- `kind`
- `weights_path`
- `gguf_adapter_path`
- `checksum_blake3`
- `created_at`
- `metadata`

## Source
Lines 109–118 in `crates/model-trainer/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/model-trainer/src/lib.md) |
