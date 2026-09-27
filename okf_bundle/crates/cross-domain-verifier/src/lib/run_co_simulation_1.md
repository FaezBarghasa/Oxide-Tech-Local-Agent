---
okf_version: "0.2"
type: Function
title: run_co_simulation
description: Run the full Electro-Thermal-Mechanical Co-Simulation fixed-point iteration loop
resource: crates/cross-domain-verifier/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:cross-domain-verifier"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:48:30Z"
concept_id: crates/cross-domain-verifier/src/lib/run_co_simulation_1
language: rust
---

# run_co_simulation

Run the full Electro-Thermal-Mechanical Co-Simulation fixed-point iteration loop

## Signature

```rust
pub fn run_co_simulation(
        &self,
        dtx_id: DtxId,
        firmware_duty_cycle: f64, // 0.0 to 1.0
        mcu_base_watts: f64,
        enclosure_material: &str,
    ) -> Result<CrossDomainVerificationReport, CrossDomainVerifierError>
```

## Visibility

- `pub`

## Docstring

Run the full Electro-Thermal-Mechanical Co-Simulation fixed-point iteration loop

## Source
Lines 73–181 in `crates/cross-domain-verifier/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/cross-domain-verifier/src/lib.md) |
