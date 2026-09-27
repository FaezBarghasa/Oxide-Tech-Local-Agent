---
okf_version: "0.2"
type: Class
title: DtxId
description: Time-ordered UUIDv7 Distributed Transaction ID
resource: crates/oxide-protocol/src/dtx.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-18T18:47:08Z"
concept_id: crates/oxide-protocol/src/dtx/DtxId
language: rust
---

# DtxId

Time-ordered UUIDv7 Distributed Transaction ID

## Signature

```rust
pub struct DtxId
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)`
- `serde(transparent)`

## Visibility

- `pub`

## Docstring

Time-ordered UUIDv7 Distributed Transaction ID
[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
[serde(transparent)]

## Source
Lines 7–7 in `crates/oxide-protocol/src/dtx.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dtx](/crates/oxide-protocol/src/dtx.md) |
