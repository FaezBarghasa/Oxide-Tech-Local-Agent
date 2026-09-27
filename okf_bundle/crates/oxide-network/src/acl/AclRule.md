---
okf_version: "0.2"
type: Class
title: AclRule
description: A discrete Zero-Trust ACL Rule
resource: crates/oxide-network/src/acl.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:06:47Z"
concept_id: crates/oxide-network/src/acl/AclRule
language: rust
---

# AclRule

A discrete Zero-Trust ACL Rule

## Signature

```rust
pub struct AclRule
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

A discrete Zero-Trust ACL Rule
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `name`
- `action`
- `src_prefix`
- `dst_prefix`
- `protocol`
- `port_range`
- `direction`

## Source
Lines 132–140 in `crates/oxide-network/src/acl.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [acl](/crates/oxide-network/src/acl.md) |
