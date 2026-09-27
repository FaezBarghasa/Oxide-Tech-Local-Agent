---
okf_version: "0.2"
type: Class
title: McuPowerBudget
description: "[derive(Debug, Clone, Serialize, Deserialize)]"
resource: crates/formal-verify/src/hardware_rules.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:formal-verify"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:35:11Z"
concept_id: crates/formal-verify/src/hardware_rules/McuPowerBudget
language: rust
---

# McuPowerBudget

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct McuPowerBudget
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `vdd_voltage`
- `max_total_vdd_current_ma`
- `max_per_pin_current_ma`

## Source
Lines 27–31 in `crates/formal-verify/src/hardware_rules.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hardware_rules](/crates/formal-verify/src/hardware_rules.md) |
