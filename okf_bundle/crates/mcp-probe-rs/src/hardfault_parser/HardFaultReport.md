---
okf_version: "0.2"
type: Class
title: HardFaultReport
description: Structured Cortex-M Crash and Panic Report
resource: crates/mcp-probe-rs/src/hardfault_parser.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:mcp-probe-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:35:12Z"
concept_id: crates/mcp-probe-rs/src/hardfault_parser/HardFaultReport
language: rust
---

# HardFaultReport

Structured Cortex-M Crash and Panic Report

## Signature

```rust
pub struct HardFaultReport
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Structured Cortex-M Crash and Panic Report
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `pc`
- `lr`
- `sp`
- `r0`
- `r1`
- `r2`
- `r3`
- `r12`
- `cfsr_raw`
- `hfsr_raw`
- `mmar_raw`
- `bfar_raw`
- `decoded_cfsr`
- `decoded_hfsr`
- `probable_cause`

## Source
Lines 38–54 in `crates/mcp-probe-rs/src/hardfault_parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hardfault_parser](/crates/mcp-probe-rs/src/hardfault_parser.md) |
