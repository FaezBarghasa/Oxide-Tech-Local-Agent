---
okf_version: "0.2"
type: Class
title: MeshStatusDto
description: Active status snapshot of the local Remote Access Mesh
resource: crates/oxide-network/src/remote_access.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:54Z"
concept_id: crates/oxide-network/src/remote_access/MeshStatusDto
language: rust
---

# MeshStatusDto

Active status snapshot of the local Remote Access Mesh

## Signature

```rust
pub struct MeshStatusDto
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Active status snapshot of the local Remote Access Mesh
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `node_id`
- `mesh_name`
- `overlay_ip`
- `active_peers_count`
- `active_routes_count`
- `uptime_secs`
- `transport_stats`
- `dns_healthy`

## Source
Lines 37–46 in `crates/oxide-network/src/remote_access.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [remote_access](/crates/oxide-network/src/remote_access.md) |
