---
okf_version: "0.2"
type: Class
title: EvidenceBundle
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/oxide-protocol/src/evidence.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-protocol/src/evidence/EvidenceBundle
language: rust
---

# EvidenceBundle

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct EvidenceBundle
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `bundle_id`
- `dtx_id`
- `inputs_hash`
- `tool_versions`
- `proofs`
- `delta`
- `generated_at`
- `ed25519_signature`

## Source
Lines 33–42 in `crates/oxide-protocol/src/evidence.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [evidence](/crates/oxide-protocol/src/evidence.md) |
