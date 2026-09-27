---
okf_version: "0.2"
type: Function
title: new
description: Initialize a new Remote Access Mesh node
resource: crates/oxide-network/src/remote_access.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:54Z"
concept_id: crates/oxide-network/src/remote_access/new_1
language: rust
---

# new

Initialize a new Remote Access Mesh node

## Signature

```rust
pub fn new(
        mesh_name: &str,
        overlay_ip: OverlayIp,
    ) -> Result<Self, OxideError>
```

## Visibility

- `pub`

## Docstring

Initialize a new Remote Access Mesh node

## Source
Lines 66–112 in `crates/oxide-network/src/remote_access.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [remote_access](/crates/oxide-network/src/remote_access.md) |
