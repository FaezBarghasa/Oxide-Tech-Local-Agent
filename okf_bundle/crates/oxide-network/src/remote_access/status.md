---
okf_version: "0.2"
type: Function
title: status
description: "Fetch real-time status and telemetry for diagnostics & UI"
resource: crates/oxide-network/src/remote_access.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:54Z"
concept_id: crates/oxide-network/src/remote_access/status
language: rust
---

# status

Fetch real-time status and telemetry for diagnostics & UI

## Signature

```rust
impl RemoteAccessMeshEngine { pub fn status(&self) -> MeshStatusDto }
```

## Visibility

- `pub`

## Docstring

Fetch real-time status and telemetry for diagnostics & UI

## Source
Lines 308–324 in `crates/oxide-network/src/remote_access.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [remote_access](/crates/oxide-network/src/remote_access.md) |
