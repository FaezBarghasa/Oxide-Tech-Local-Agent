---
okf_version: "0.2"
type: Class
title: PacketMeta
description: Extracted metadata from an IP packet
resource: crates/oxide-network/src/acl.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:06:47Z"
concept_id: crates/oxide-network/src/acl/PacketMeta
language: rust
---

# PacketMeta

Extracted metadata from an IP packet

## Signature

```rust
pub struct PacketMeta
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Extracted metadata from an IP packet
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Methods

- `src_ip`
- `dst_ip`
- `src_port`
- `dst_port`
- `protocol`
- `direction`

## Source
Lines 45–52 in `crates/oxide-network/src/acl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [acl](/crates/oxide-network/src/acl.md) |
