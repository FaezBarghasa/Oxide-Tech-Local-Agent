---
okf_version: "0.2"
type: Function
title: get_available_disk_mb
description: Read available disk space on the target filesystem in Megabytes using sysinfo
resource: crates/oxide-security/src/resource_gater.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:18:58Z"
concept_id: crates/oxide-security/src/resource_gater/get_available_disk_mb_1
language: rust
---

# get_available_disk_mb

Read available disk space on the target filesystem in Megabytes using sysinfo

## Signature

```rust
pub fn get_available_disk_mb(&self) -> u64
```

## Visibility

- `pub`

## Docstring

Read available disk space on the target filesystem in Megabytes using sysinfo

## Source
Lines 62–74 in `crates/oxide-security/src/resource_gater.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [resource_gater](/crates/oxide-security/src/resource_gater.md) |
