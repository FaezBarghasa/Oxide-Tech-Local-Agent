---
okf_version: "0.2"
type: Class
title: AdapterManifest
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/oxide-protocol/src/manifest.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-18T18:40:25Z"
concept_id: crates/oxide-protocol/src/manifest/AdapterManifest
language: rust
---

# AdapterManifest

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct AdapterManifest
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
- `target_domain`
- `lora_rank`
- `lora_alpha`
- `file_path`
- `checksum_blake3`
- `minisign_signature`

## Source
Lines 18–27 in `crates/oxide-protocol/src/manifest.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [manifest](/crates/oxide-protocol/src/manifest.md) |
