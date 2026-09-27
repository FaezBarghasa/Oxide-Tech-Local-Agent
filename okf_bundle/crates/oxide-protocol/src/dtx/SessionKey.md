---
okf_version: "0.2"
type: Class
title: SessionKey
description: "Session & Tenant identifier"
resource: crates/oxide-protocol/src/dtx.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-18T18:47:08Z"
concept_id: crates/oxide-protocol/src/dtx/SessionKey
language: rust
---

# SessionKey

Session & Tenant identifier

## Signature

```rust
pub struct SessionKey
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Session & Tenant identifier
[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]

## Methods

- `user_id`
- `session_id`
- `namespace`

## Source
Lines 29–33 in `crates/oxide-protocol/src/dtx.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dtx](/crates/oxide-protocol/src/dtx.md) |
