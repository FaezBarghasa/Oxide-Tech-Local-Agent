---
okf_version: "0.2"
type: Class
title: DtxEnvelope
description: Distributed Transaction Envelope carrying causal trace and tenant boundary
resource: crates/oxide-protocol/src/dtx.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-18T18:47:08Z"
concept_id: crates/oxide-protocol/src/dtx/DtxEnvelope
language: rust
---

# DtxEnvelope

Distributed Transaction Envelope carrying causal trace and tenant boundary

## Signature

```rust
pub struct DtxEnvelope
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Distributed Transaction Envelope carrying causal trace and tenant boundary
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `dtx_id`
- `causation_id`
- `session`
- `op`
- `deadline_unix_ms`

## Source
Lines 50–56 in `crates/oxide-protocol/src/dtx.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dtx](/crates/oxide-protocol/src/dtx.md) |
