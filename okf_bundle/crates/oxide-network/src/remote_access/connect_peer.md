---
okf_version: "0.2"
type: Function
title: connect_peer
description: Connect and establish an encrypted session with a peer node
resource: crates/oxide-network/src/remote_access.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-network"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-27T08:09:54Z"
concept_id: crates/oxide-network/src/remote_access/connect_peer
language: rust
---

# connect_peer

Connect and establish an encrypted session with a peer node

## Signature

```rust
impl RemoteAccessMeshEngine { pub fn connect_peer(
        &self,
        peer_id: NodeId,
        peer_pk: DeviceIdentityPublicKey,
        peer_overlay_ip: OverlayIp,
        endpoint: SocketAddr,
        hostname: Option<&str>,
    ) -> Result<(), OxideError> }
```

## Visibility

- `pub`

## Docstring

Connect and establish an encrypted session with a peer node

## Source
Lines 139–195 in `crates/oxide-network/src/remote_access.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [remote_access](/crates/oxide-network/src/remote_access.md) |
