---
okf_version: "0.2"
type: Function
title: parse_fault_frame
description: Decode raw ARM register status values into a structured fault diagnostic
resource: crates/mcp-probe-rs/src/hardfault_parser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:mcp-probe-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:35:12Z"
concept_id: crates/mcp-probe-rs/src/hardfault_parser/parse_fault_frame
language: rust
---

# parse_fault_frame

Decode raw ARM register status values into a structured fault diagnostic

## Signature

```rust
impl HardFaultParser { pub fn parse_fault_frame(
        frame: ArmRegisterFrame,
        cfsr: u32,
        hfsr: u32,
        mmar: Option<u32>,
        bfar: Option<u32>,
    ) -> HardFaultReport }
```

## Visibility

- `pub`

## Docstring

Decode raw ARM register status values into a structured fault diagnostic

## Source
Lines 72–168 in `crates/mcp-probe-rs/src/hardfault_parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hardfault_parser](/crates/mcp-probe-rs/src/hardfault_parser.md) |
