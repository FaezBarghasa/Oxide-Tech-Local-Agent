---
okf_version: "0.2"
type: Class
title: AclRuleWire
description: Wire format for ACL rule
resource: crates/oxide-network/src/wire.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:34Z"
concept_id: crates/oxide-network/src/wire/AclRuleWire
language: rust
---

# AclRuleWire

Wire format for ACL rule

## Signature

```rust
pub struct AclRuleWire
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Wire format for ACL rule
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `action`
- `src_identity`
- `dst_prefix`
- `protocol`
- `port_range`
- `direction`

## Source
Lines 315–322 in `crates/oxide-network/src/wire.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [wire](/crates/oxide-network/src/wire.md) |
