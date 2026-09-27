---
okf_version: "0.2"
type: Function
title: step_simulation
description: Step simulation timeline by delta_t and advance cycles based on MCU frequency
resource: crates/cross-domain-verifier/src/mcu_spice_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:cross-domain-verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:48:24Z"
concept_id: crates/cross-domain-verifier/src/mcu_spice_bridge/step_simulation
language: rust
---

# step_simulation

Step simulation timeline by delta_t and advance cycles based on MCU frequency

## Signature

```rust
impl McuSpiceBridge { pub fn step_simulation(&mut self, delta_t: f64, mcu_freq_hz: f64) }
```

## Visibility

- `pub`

## Docstring

Step simulation timeline by delta_t and advance cycles based on MCU frequency

## Source
Lines 97–101 in `crates/cross-domain-verifier/src/mcu_spice_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mcu_spice_bridge](/crates/cross-domain-verifier/src/mcu_spice_bridge.md) |
