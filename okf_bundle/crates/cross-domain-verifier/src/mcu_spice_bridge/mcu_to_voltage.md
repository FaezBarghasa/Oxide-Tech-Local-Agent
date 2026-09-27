---
okf_version: "0.2"
type: Function
title: mcu_to_voltage
description: Converts MCU digital boolean state to SPICE analog voltage
resource: crates/cross-domain-verifier/src/mcu_spice_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:cross-domain-verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:48:24Z"
concept_id: crates/cross-domain-verifier/src/mcu_spice_bridge/mcu_to_voltage
language: rust
---

# mcu_to_voltage

Converts MCU digital boolean state to SPICE analog voltage

## Signature

```rust
impl DigitalPinBridge { pub fn mcu_to_voltage(&self, is_high: bool) -> f64 }
```

## Visibility

- `pub`

## Docstring

Converts MCU digital boolean state to SPICE analog voltage

## Source
Lines 38–44 in `crates/cross-domain-verifier/src/mcu_spice_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mcu_spice_bridge](/crates/cross-domain-verifier/src/mcu_spice_bridge.md) |
