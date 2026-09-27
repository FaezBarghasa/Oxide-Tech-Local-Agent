---
okf_version: "0.2"
type: Class
title: SandboxSpec
description: "[derive(Debug, Clone)]"
resource: crates/sandbox/src/execution.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:sandbox"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/sandbox/src/execution/SandboxSpec
language: rust
---

# SandboxSpec

[derive(Debug, Clone)]

## Signature

```rust
pub struct SandboxSpec
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

[derive(Debug, Clone)]

## Methods

- `ro_binds`
- `rw_binds`
- `net`
- `timeout`
- `memory_limit_bytes`
- `cpu_time_limit_secs`
- `device_allow_list`

## Source
Lines 24–32 in `crates/sandbox/src/execution.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [execution](/crates/sandbox/src/execution.md) |
