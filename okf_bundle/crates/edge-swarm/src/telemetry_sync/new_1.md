---
okf_version: "0.2"
type: Function
title: new
resource: crates/edge-swarm/src/telemetry_sync.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:edge-swarm"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/edge-swarm/src/telemetry_sync/new_1
language: rust
---

# new

## Signature

```rust
pub fn new(
        client_id: impl Into<String>,
        broker_host: impl Into<String>,
        broker_port: u16,
    ) -> Self
```

## Visibility

- `pub`

## Source
Lines 12–22 in `crates/edge-swarm/src/telemetry_sync.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [telemetry_sync](/crates/edge-swarm/src/telemetry_sync.md) |
