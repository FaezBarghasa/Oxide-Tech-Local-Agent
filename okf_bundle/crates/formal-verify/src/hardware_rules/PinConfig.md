---
okf_version: "0.2"
type: Class
title: PinConfig
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
concept_id: crates/formal-verify/src/hardware_rules/PinConfig
language: rust
---

# PinConfig

[derive(Debug, Clone, Serialize, Deserialize)]

## Signature

```rust
pub struct PinConfig
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `pin_name`
- `net_name`
- `mode`
- `estimated_current_ma`
- `is_5v_tolerant`

## Source
Lines 18–24 in `crates/formal-verify/src/hardware_rules.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hardware_rules](/crates/formal-verify/src/hardware_rules.md) |
