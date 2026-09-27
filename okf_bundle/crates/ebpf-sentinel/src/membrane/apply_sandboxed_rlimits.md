---
okf_version: "0.2"
type: Function
title: apply_sandboxed_rlimits
description: Apply POSIX resource limits (memory and open file descriptors) to the calling process.
resource: crates/ebpf-sentinel/src/membrane.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:ebpf-sentinel"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:54:15Z"
concept_id: crates/ebpf-sentinel/src/membrane/apply_sandboxed_rlimits
language: rust
---

# apply_sandboxed_rlimits

Apply POSIX resource limits (memory and open file descriptors) to the calling process.

## Signature

```rust
impl ConstitutionalMembrane { pub fn apply_sandboxed_rlimits(
        max_address_space_bytes: u64,
        max_open_fds: u64,
    ) -> Result<(), String> }
```

## Visibility

- `pub`

## Docstring

Apply POSIX resource limits (memory and open file descriptors) to the calling process.
[cfg(target_os = "linux")]

## Source
Lines 75–109 in `crates/ebpf-sentinel/src/membrane.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [membrane](/crates/ebpf-sentinel/src/membrane.md) |
