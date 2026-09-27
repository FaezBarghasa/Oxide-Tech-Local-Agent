---
okf_version: "0.2"
type: Class
title: Ddr5TierManager
description: "Dynamic DDR5 RAM Tier Manager that automatically offloads layers, optimizer states,"
resource: crates/model-trainer/src/ddr5_offload.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:model-trainer"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:09:52Z"
concept_id: crates/model-trainer/src/ddr5_offload/Ddr5TierManager
language: rust
---

# Ddr5TierManager

Dynamic DDR5 RAM Tier Manager that automatically offloads layers, optimizer states,

## Signature

```rust
pub struct Ddr5TierManager
```

## Visibility

- `pub`

## Docstring

Dynamic DDR5 RAM Tier Manager that automatically offloads layers, optimizer states,
and KV-cache blocks when GPU VRAM hits threshold.

## Methods

- `vram_headroom_threshold_mb`
- `max_ddr5_budget_mb`
- `buffers`

## Source
Lines 31–38 in `crates/model-trainer/src/ddr5_offload.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ddr5_offload](/crates/model-trainer/src/ddr5_offload.md) |
