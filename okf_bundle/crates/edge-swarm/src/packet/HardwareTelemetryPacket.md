---
okf_version: "0.2"
type: Class
title: HardwareTelemetryPacket
description: "High-throughput, `#![no_std]` binary compatible telemetry packet for microcontrollers."
resource: crates/edge-swarm/src/packet.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:edge-swarm"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-08T18:51:10Z"
concept_id: crates/edge-swarm/src/packet/HardwareTelemetryPacket
language: rust
---

# HardwareTelemetryPacket

High-throughput, `#![no_std]` binary compatible telemetry packet for microcontrollers.

## Signature

```rust
pub struct HardwareTelemetryPacket
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize, PartialEq)`

## Visibility

- `pub`

## Docstring

High-throughput, `#![no_std]` binary compatible telemetry packet for microcontrollers.
[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

## Methods

- `device_id`
- `timestamp_ms`
- `cpu_frequency_mhz`
- `vdda_millivolts`
- `core_temperature_c`
- `rtt_log_snippet`

## Source
Lines 5–12 in `crates/edge-swarm/src/packet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [packet](/crates/edge-swarm/src/packet.md) |
