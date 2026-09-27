---
okf_version: "0.2"
type: Class
title: DecodedCfsr
description: ARM Cortex-M Configurable Fault Status Register (CFSR) Flags
resource: crates/mcp-probe-rs/src/hardfault_parser.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:mcp-probe-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:35:12Z"
concept_id: crates/mcp-probe-rs/src/hardfault_parser/DecodedCfsr
language: rust
---

# DecodedCfsr

ARM Cortex-M Configurable Fault Status Register (CFSR) Flags

## Signature

```rust
pub struct DecodedCfsr
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

ARM Cortex-M Configurable Fault Status Register (CFSR) Flags
[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `mem_instruction_access_violation`
- `mem_data_access_violation`
- `mem_stack_unstack_violation`
- `mem_fault_address_valid`
- `bus_instruction_error`
- `bus_precise_data_error`
- `bus_imprecise_data_error`
- `bus_unstack_error`
- `bus_fault_address_valid`
- `usage_undefined_instruction`
- `usage_invalid_state`
- `usage_invalid_pc_load`
- `usage_no_coprocessor`
- `usage_unaligned_access`
- `usage_divide_by_zero`

## Source
Lines 5–26 in `crates/mcp-probe-rs/src/hardfault_parser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hardfault_parser](/crates/mcp-probe-rs/src/hardfault_parser.md) |
