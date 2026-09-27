---
okf_version: "0.2"
type: Class
title: PathDiscoveryPayload
description: Path discovery probe payload
resource: crates/oxide-network/src/wire.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:34Z"
concept_id: crates/oxide-network/src/wire/PathDiscoveryPayload
language: rust
---

# PathDiscoveryPayload

Path discovery probe payload

## Signature

```rust
pub struct PathDiscoveryPayload
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Path discovery probe payload
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `probe_id`
- `src_node`
- `dst_node`
- `path_mtu`
- `timestamp`

## Source
Lines 289–295 in `crates/oxide-network/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-network/src/wire.md) |
