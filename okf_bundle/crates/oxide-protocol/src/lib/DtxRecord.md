---
okf_version: "0.2"
type: Class
title: DtxRecord
description: Distributed Transaction metadata record
resource: crates/oxide-protocol/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T14:30:29Z"
concept_id: crates/oxide-protocol/src/lib/DtxRecord
language: rust
---

# DtxRecord

Distributed Transaction metadata record

## Signature

```rust
pub struct DtxRecord
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Distributed Transaction metadata record
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `dtx_id`
- `title`
- `initiator`
- `domains`
- `status`
- `created_at`
- `updated_at`

## Source
Lines 55–63 in `crates/oxide-protocol/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-protocol/src/lib.md) |
