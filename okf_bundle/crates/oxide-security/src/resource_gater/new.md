---
okf_version: "0.2"
type: Function
title: new
resource: crates/oxide-security/src/resource_gater.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-security"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T12:18:58Z"
concept_id: crates/oxide-security/src/resource_gater/new
language: rust
---

# new

## Signature

```rust
impl ResourceGater { pub fn new(
        mount_path: impl AsRef<Path>,
        min_disk_free_mb: u64,
        resume_disk_free_mb: u64,
        min_vram_free_mb: u64,
    ) -> Self }
```

## Visibility

- `pub`

## Source
Lines 46–59 in `crates/oxide-security/src/resource_gater.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [resource_gater](/crates/oxide-security/src/resource_gater.md) |
