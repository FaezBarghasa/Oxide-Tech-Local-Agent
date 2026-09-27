---
okf_version: "0.2"
type: Function
title: deserialize
resource: crates/oxide-network/src/crypto.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:08:21Z"
concept_id: crates/oxide-network/src/crypto/deserialize_1
language: rust
---

# deserialize

## Signature

```rust
fn deserialize(deserializer: D) -> Result<Self, D::Error>
```

## Type Parameters

- `D`

## Source
Lines 118–133 in `crates/oxide-network/src/crypto.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [crypto](/crates/oxide-network/src/crypto.md) |
| calls | [decode](/crates/oxide-network/src/crypto/decode.md) |
