---
okf_version: "0.2"
type: Class
title: ResourceGater
description: Dynamic Disk and VRAM circuit breaker
resource: crates/oxide-security/src/resource_gater.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:18:58Z"
concept_id: crates/oxide-security/src/resource_gater/ResourceGater
language: rust
---

# ResourceGater

Dynamic Disk and VRAM circuit breaker

## Signature

```rust
pub struct ResourceGater
```

## Visibility

- `pub`

## Docstring

Dynamic Disk and VRAM circuit breaker

## Methods

- `mount_path`
- `min_disk_free_mb`
- `resume_disk_free_mb`
- `min_vram_free_mb`
- `status`

## Source
Lines 31–37 in `crates/oxide-security/src/resource_gater.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [resource_gater](/crates/oxide-security/src/resource_gater.md) |
