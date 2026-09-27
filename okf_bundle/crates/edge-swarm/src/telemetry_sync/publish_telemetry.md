---
okf_version: "0.2"
type: Function
title: publish_telemetry
description: Dispatch telemetry packet over MQTT v5 broker to synchronize digital twin.
resource: crates/edge-swarm/src/telemetry_sync.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:edge-swarm"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T19:19:18Z"
concept_id: crates/edge-swarm/src/telemetry_sync/publish_telemetry
language: rust
---

# publish_telemetry

Dispatch telemetry packet over MQTT v5 broker to synchronize digital twin.

## Signature

```rust
impl EdgeSwarmBridge { pub fn publish_telemetry(
        &self,
        topic: &str,
        packet: &HardwareTelemetryPacket,
    ) -> Result<(), String> }
```

## Visibility

- `pub`

## Docstring

Dispatch telemetry packet over MQTT v5 broker to synchronize digital twin.

## Source
Lines 25–51 in `crates/edge-swarm/src/telemetry_sync.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [telemetry_sync](/crates/edge-swarm/src/telemetry_sync.md) |
