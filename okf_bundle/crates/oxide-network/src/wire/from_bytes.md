---
okf_version: "0.2"
type: Function
title: from_bytes
description: Parse from bytes
resource: crates/oxide-network/src/wire.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:34Z"
concept_id: crates/oxide-network/src/wire/from_bytes
language: rust
---

# from_bytes

Parse from bytes

## Signature

```rust
impl WirePacket { pub fn from_bytes(bytes: &[u8]) -> Result<Self, OxideError> }
```

## Visibility

- `pub`

## Docstring

Parse from bytes

## Source
Lines 108–126 in `crates/oxide-network/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-network/src/wire.md) |
