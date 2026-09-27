---
okf_version: "0.2"
type: Function
title: voltage_to_mcu
description: "Converts SPICE analog voltage to MCU digital state with 70%/30% hysteresis"
resource: crates/cross-domain-verifier/src/mcu_spice_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:cross-domain-verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:48:24Z"
concept_id: crates/cross-domain-verifier/src/mcu_spice_bridge/voltage_to_mcu_1
language: rust
---

# voltage_to_mcu

Converts SPICE analog voltage to MCU digital state with 70%/30% hysteresis

## Signature

```rust
pub fn voltage_to_mcu(&self, voltage: f64) -> Option<bool>
```

## Visibility

- `pub`

## Docstring

Converts SPICE analog voltage to MCU digital state with 70%/30% hysteresis

## Source
Lines 47–58 in `crates/cross-domain-verifier/src/mcu_spice_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mcu_spice_bridge](/crates/cross-domain-verifier/src/mcu_spice_bridge.md) |
