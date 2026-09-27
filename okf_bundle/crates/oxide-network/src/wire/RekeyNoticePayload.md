---
okf_version: "0.2"
type: Class
title: RekeyNoticePayload
description: Rekey notice payload
resource: crates/oxide-network/src/wire.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:34Z"
concept_id: crates/oxide-network/src/wire/RekeyNoticePayload
language: rust
---

# RekeyNoticePayload

Rekey notice payload

## Signature

```rust
pub struct RekeyNoticePayload
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Rekey notice payload
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `node_id`
- `new_session_pub`
- `valid_from`
- `valid_until`

## Source
Lines 299–304 in `crates/oxide-network/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-network/src/wire.md) |
