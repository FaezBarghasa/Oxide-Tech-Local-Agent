---
okf_version: "0.2"
type: Function
title: apply_sandboxed_rlimits
description: "[cfg(not(target_os = \"linux\"))]"
resource: crates/ebpf-sentinel/src/membrane.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:ebpf-sentinel"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:54:15Z"
concept_id: crates/ebpf-sentinel/src/membrane/apply_sandboxed_rlimits_3
language: rust
---

# apply_sandboxed_rlimits

[cfg(not(target_os = "linux"))]

## Signature

```rust
pub fn apply_sandboxed_rlimits(
        _max_address_space_bytes: u64,
        _max_open_fds: u64,
    ) -> Result<(), String>
```

## Decorators

- `cfg(not(target_os = "linux"))`

## Visibility

- `pub`

## Docstring

[cfg(not(target_os = "linux"))]

## Source
Lines 112–117 in `crates/ebpf-sentinel/src/membrane.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [membrane](/crates/ebpf-sentinel/src/membrane.md) |
