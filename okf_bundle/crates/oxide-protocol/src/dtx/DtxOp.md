---
okf_version: "0.2"
type: Class
title: DtxOp
description: Operation payload in a Distributed Transaction
resource: crates/oxide-protocol/src/dtx.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-18T18:47:08Z"
concept_id: crates/oxide-protocol/src/dtx/DtxOp
language: rust
---

# DtxOp

Operation payload in a Distributed Transaction

## Signature

```rust
pub enum DtxOp
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Operation payload in a Distributed Transaction
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `participants`
- `participant`
- `vote`
- `participant`
- `participant`
- `reason`
- `participant`
- `action`
- `participant`
- `backup_ref`

## Source
Lines 91–114 in `crates/oxide-protocol/src/dtx.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dtx](/crates/oxide-protocol/src/dtx.md) |
