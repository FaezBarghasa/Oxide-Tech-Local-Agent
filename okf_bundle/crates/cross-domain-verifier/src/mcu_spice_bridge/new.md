---
okf_version: "0.2"
type: Function
title: new
resource: crates/cross-domain-verifier/src/mcu_spice_bridge.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:cross-domain-verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:48:24Z"
concept_id: crates/cross-domain-verifier/src/mcu_spice_bridge/new
language: rust
---

# new

## Signature

```rust
impl DigitalPinBridge { pub fn new(
        mcu_pin_name: impl Into<String>,
        spice_net_name: impl Into<String>,
        direction: PinDirection,
    ) -> Self }
```

## Visibility

- `pub`

## Source
Lines 23–35 in `crates/cross-domain-verifier/src/mcu_spice_bridge.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mcu_spice_bridge](/crates/cross-domain-verifier/src/mcu_spice_bridge.md) |
