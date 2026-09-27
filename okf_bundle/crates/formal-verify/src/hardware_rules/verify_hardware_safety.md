---
okf_version: "0.2"
type: Function
title: verify_hardware_safety
description: Run comprehensive pre-flight verification on the proposed pinout and netlist configuration
resource: crates/formal-verify/src/hardware_rules.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:formal-verify"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:35:11Z"
concept_id: crates/formal-verify/src/hardware_rules/verify_hardware_safety
language: rust
---

# verify_hardware_safety

Run comprehensive pre-flight verification on the proposed pinout and netlist configuration

## Signature

```rust
impl HardwareSafetyChecker { pub fn verify_hardware_safety(
        &self,
        pins: &[PinConfig],
        net_voltages: &HashMap<String, f32>,
    ) -> Result<(), Vec<HardwareSafetyViolation>> }
```

## Visibility

- `pub`

## Docstring

Run comprehensive pre-flight verification on the proposed pinout and netlist configuration

## Source
Lines 99–185 in `crates/formal-verify/src/hardware_rules.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hardware_rules](/crates/formal-verify/src/hardware_rules.md) |
