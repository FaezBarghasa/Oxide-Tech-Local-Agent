---
okf_version: "0.2"
type: Function
title: generate
description: Generates a new random Ed25519 device key pair
resource: crates/oxide-network/src/crypto.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:08:21Z"
concept_id: crates/oxide-network/src/crypto/generate_1
language: rust
---

# generate

Generates a new random Ed25519 device key pair

## Signature

```rust
pub fn generate() -> Result<Self, OxideError>
```

## Visibility

- `pub`

## Docstring

Generates a new random Ed25519 device key pair

## Source
Lines 23–34 in `crates/oxide-network/src/crypto.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [crypto](/crates/oxide-network/src/crypto.md) |
