---
okf_version: "0.2"
type: Function
title: admit_work
description: Check admission before accepting new agent turns or heavy allocations
resource: crates/oxide-security/src/resource_gater.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:18:58Z"
concept_id: crates/oxide-security/src/resource_gater/admit_work
language: rust
---

# admit_work

Check admission before accepting new agent turns or heavy allocations

## Signature

```rust
impl ResourceGater { pub fn admit_work(&self, free_vram_mb: Option<u64>) -> Result<(), GateRejection> }
```

## Visibility

- `pub`

## Docstring

Check admission before accepting new agent turns or heavy allocations

## Source
Lines 117–144 in `crates/oxide-security/src/resource_gater.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [resource_gater](/crates/oxide-security/src/resource_gater.md) |
