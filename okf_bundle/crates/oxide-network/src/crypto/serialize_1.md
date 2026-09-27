---
okf_version: "0.2"
type: Function
title: serialize
resource: crates/oxide-network/src/crypto.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:08:21Z"
concept_id: crates/oxide-network/src/crypto/serialize_1
language: rust
---

# serialize

## Signature

```rust
fn serialize(&self, serializer: S) -> Result<S::Ok, S::Error>
```

## Type Parameters

- `S`

## Source
Lines 109–114 in `crates/oxide-network/src/crypto.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [crypto](/crates/oxide-network/src/crypto.md) |
| calls | [encode](/crates/oxide-network/src/crypto/encode.md) |
