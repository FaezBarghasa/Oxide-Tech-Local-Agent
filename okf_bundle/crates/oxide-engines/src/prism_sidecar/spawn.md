---
okf_version: "0.2"
type: Function
title: spawn
description: Spawn a new dedicated PrismML Bonsai llama.cpp sidecar with Flash Attention and GPU offload.
resource: crates/oxide-engines/src/prism_sidecar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-engines/src/prism_sidecar/spawn
language: rust
---

# spawn

Spawn a new dedicated PrismML Bonsai llama.cpp sidecar with Flash Attention and GPU offload.

## Signature

```rust
impl PrismBonsaiEngine { pub fn spawn(
        model_path: P,
        port: u16,
        gpu_layers: u32,
        context_len: usize,
    ) -> Result<Self, OxideError> }
```

## Type Parameters

- `P: AsRef<Path`

## Visibility

- `pub`

## Docstring

Spawn a new dedicated PrismML Bonsai llama.cpp sidecar with Flash Attention and GPU offload.

## Source
Lines 47–96 in `crates/oxide-engines/src/prism_sidecar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prism_sidecar](/crates/oxide-engines/src/prism_sidecar.md) |
