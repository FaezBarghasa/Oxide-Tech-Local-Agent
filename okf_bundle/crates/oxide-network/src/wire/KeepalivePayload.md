---
okf_version: "0.2"
type: Class
title: KeepalivePayload
description: Control keepalive payload
resource: crates/oxide-network/src/wire.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:34Z"
concept_id: crates/oxide-network/src/wire/KeepalivePayload
language: rust
---

# KeepalivePayload

Control keepalive payload

## Signature

```rust
pub struct KeepalivePayload
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Control keepalive payload
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `node_id`
- `timestamp`
- `capabilities`
- `endpoints`

## Source
Lines 280–285 in `crates/oxide-network/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-network/src/wire.md) |
