---
okf_version: "0.2"
type: Class
title: OxideMcpRequest
description: Standard JSON-RPC 2.0 Request for Oxide-MCP
resource: crates/oxide-protocol/src/lib.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-protocol"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T14:30:29Z"
concept_id: crates/oxide-protocol/src/lib/OxideMcpRequest
language: rust
---

# OxideMcpRequest

Standard JSON-RPC 2.0 Request for Oxide-MCP

## Signature

```rust
pub struct OxideMcpRequest
```

## Type Parameters

- `T = serde_json::Value`

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Standard JSON-RPC 2.0 Request for Oxide-MCP
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `jsonrpc`
- `id`
- `method`
- `params`

## Source
Lines 86–91 in `crates/oxide-protocol/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-protocol/src/lib.md) |
