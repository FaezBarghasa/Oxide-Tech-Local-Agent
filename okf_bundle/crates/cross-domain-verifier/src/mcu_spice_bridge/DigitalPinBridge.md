---
okf_version: "0.2"
type: Class
title: DigitalPinBridge
description: Dynamic digital pin bridge linking QEMU GPIOs and analog SPICE nodes
resource: crates/cross-domain-verifier/src/mcu_spice_bridge.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:cross-domain-verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:48:24Z"
concept_id: crates/cross-domain-verifier/src/mcu_spice_bridge/DigitalPinBridge
language: rust
---

# DigitalPinBridge

Dynamic digital pin bridge linking QEMU GPIOs and analog SPICE nodes

## Signature

```rust
pub struct DigitalPinBridge
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Dynamic digital pin bridge linking QEMU GPIOs and analog SPICE nodes
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `mcu_pin_name`
- `spice_net_name`
- `direction`
- `logic_high_voltage`
- `logic_low_voltage`

## Source
Lines 14–20 in `crates/cross-domain-verifier/src/mcu_spice_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mcu_spice_bridge](/crates/cross-domain-verifier/src/mcu_spice_bridge.md) |
