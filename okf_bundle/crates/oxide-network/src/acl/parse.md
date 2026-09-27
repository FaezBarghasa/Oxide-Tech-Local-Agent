---
okf_version: "0.2"
type: Function
title: parse
description: Parse packet metadata from raw IPv4/IPv6 packet buffer
resource: crates/oxide-network/src/acl.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:06:47Z"
concept_id: crates/oxide-network/src/acl/parse
language: rust
---

# parse

Parse packet metadata from raw IPv4/IPv6 packet buffer

## Signature

```rust
impl PacketMeta { pub fn parse(buffer: &[u8], direction: AclDirection) -> Option<Self> }
```

## Visibility

- `pub`

## Docstring

Parse packet metadata from raw IPv4/IPv6 packet buffer

## Source
Lines 56–127 in `crates/oxide-network/src/acl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [acl](/crates/oxide-network/src/acl.md) |
