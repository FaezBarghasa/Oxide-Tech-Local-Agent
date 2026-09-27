---
okf_version: "0.2"
type: Class
title: PartitionLayout
description: A/B Partition Layout for STM32 / ARM Cortex-M
resource: crates/mcp-probe-rs/src/atomic_flash.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:mcp-probe-rs"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:35:12Z"
concept_id: crates/mcp-probe-rs/src/atomic_flash/PartitionLayout
language: rust
---

# PartitionLayout

A/B Partition Layout for STM32 / ARM Cortex-M

## Signature

```rust
pub struct PartitionLayout
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

A/B Partition Layout for STM32 / ARM Cortex-M
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `slot_a_base`
- `slot_b_base`
- `slot_size_bytes`
- `bootloader_base`

## Source
Lines 53–58 in `crates/mcp-probe-rs/src/atomic_flash.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [atomic_flash](/crates/mcp-probe-rs/src/atomic_flash.md) |
