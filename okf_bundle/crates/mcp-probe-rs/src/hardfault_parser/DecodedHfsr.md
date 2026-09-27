---
okf_version: "0.2"
type: Class
title: DecodedHfsr
description: ARM Cortex-M HardFault Status Register (HFSR) Flags
resource: crates/mcp-probe-rs/src/hardfault_parser.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:mcp-probe-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:35:12Z"
concept_id: crates/mcp-probe-rs/src/hardfault_parser/DecodedHfsr
language: rust
---

# DecodedHfsr

ARM Cortex-M HardFault Status Register (HFSR) Flags

## Signature

```rust
pub struct DecodedHfsr
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

ARM Cortex-M HardFault Status Register (HFSR) Flags
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `vector_table_read_fault`
- `forced_hardfault`
- `debug_event`

## Source
Lines 30–34 in `crates/mcp-probe-rs/src/hardfault_parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hardfault_parser](/crates/mcp-probe-rs/src/hardfault_parser.md) |
