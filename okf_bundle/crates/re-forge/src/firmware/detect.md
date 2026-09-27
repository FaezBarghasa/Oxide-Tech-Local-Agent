---
okf_version: "0.2"
type: Function
title: detect
resource: crates/re-forge/src/firmware.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:re-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/re-forge/src/firmware/detect
language: rust
---

# detect

## Signature

```rust
impl RtosDetector { pub fn detect(bytes: &[u8]) -> RtosDetectionResult }
```

## Visibility

- `pub`

## Source
Lines 200–241 in `crates/re-forge/src/firmware.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [firmware](/crates/re-forge/src/firmware.md) |
