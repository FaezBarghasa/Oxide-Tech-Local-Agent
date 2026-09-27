---
okf_version: "0.2"
type: Function
title: scan
resource: crates/re-forge/src/firmware.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:re-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/re-forge/src/firmware/scan
language: rust
---

# scan

## Signature

```rust
impl EntropyScanner { pub fn scan(bytes: &[u8], chunk_size: usize) -> Vec<EntropyChunk> }
```

## Visibility

- `pub`

## Source
Lines 87–114 in `crates/re-forge/src/firmware.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [firmware](/crates/re-forge/src/firmware.md) |
