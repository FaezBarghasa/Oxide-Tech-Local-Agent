---
okf_version: "0.2"
type: Class
title: RtosDetectionResult
description: RTOS Signature and RTIC/Embassy Detection
resource: crates/re-forge/src/firmware.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:re-forge"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/re-forge/src/firmware/RtosDetectionResult
language: rust
---

# RtosDetectionResult

RTOS Signature and RTIC/Embassy Detection

## Signature

```rust
pub struct RtosDetectionResult
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

RTOS Signature and RTIC/Embassy Detection
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `detected_rtos`
- `confidence`
- `signatures_found`

## Source
Lines 191–195 in `crates/re-forge/src/firmware.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [firmware](/crates/re-forge/src/firmware.md) |
