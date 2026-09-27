---
okf_version: "0.2"
type: Class
title: ArmVectorTable
description: "ARM Cortex-M Interrupt Vector Table (IVT) parsed from flash origin (e.g., 0x08000000)"
resource: crates/re-forge/src/firmware.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:re-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/re-forge/src/firmware/ArmVectorTable
language: rust
---

# ArmVectorTable

ARM Cortex-M Interrupt Vector Table (IVT) parsed from flash origin (e.g., 0x08000000)

## Signature

```rust
pub struct ArmVectorTable
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

ARM Cortex-M Interrupt Vector Table (IVT) parsed from flash origin (e.g., 0x08000000)
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `initial_sp`
- `reset_handler`
- `nmi_handler`
- `hardfault_handler`
- `memmanage_handler`
- `busfault_handler`
- `usagefault_handler`
- `svcall_handler`
- `pendsv_handler`
- `systick_handler`
- `external_irqs`

## Source
Lines 6–18 in `crates/re-forge/src/firmware.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [firmware](/crates/re-forge/src/firmware.md) |
