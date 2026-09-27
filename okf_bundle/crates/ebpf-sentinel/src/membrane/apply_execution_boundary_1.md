---
okf_version: "0.2"
type: Function
title: apply_execution_boundary
description: Apply execution boundary based on negotiated isolation tier
resource: crates/ebpf-sentinel/src/membrane.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:ebpf-sentinel"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-24T15:54:15Z"
concept_id: crates/ebpf-sentinel/src/membrane/apply_execution_boundary_1
language: rust
---

# apply_execution_boundary

Apply execution boundary based on negotiated isolation tier

## Signature

```rust
pub fn apply_execution_boundary(&self, pid: u32) -> Result<IsolationTier, String>
```

## Visibility

- `pub`

## Docstring

Apply execution boundary based on negotiated isolation tier

## Source
Lines 161–177 in `crates/ebpf-sentinel/src/membrane.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [membrane](/crates/ebpf-sentinel/src/membrane.md) |
