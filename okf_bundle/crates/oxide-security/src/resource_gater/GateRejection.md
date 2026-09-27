---
okf_version: "0.2"
type: Class
title: GateRejection
description: Rejection reason when work admission fails
resource: crates/oxide-security/src/resource_gater.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:18:58Z"
concept_id: crates/oxide-security/src/resource_gater/GateRejection
language: rust
---

# GateRejection

Rejection reason when work admission fails

## Signature

```rust
pub enum GateRejection
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Rejection reason when work admission fails
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `available_mb`
- `min_required_mb`
- `available_mb`
- `min_required_mb`

## Source
Lines 19–28 in `crates/oxide-security/src/resource_gater.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [resource_gater](/crates/oxide-security/src/resource_gater.md) |
