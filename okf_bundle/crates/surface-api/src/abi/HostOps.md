---
okf_version: "0.2"
type: Class
title: HostOps
description: Host operations vtable provided to the mutable surface cdylib
resource: crates/surface-api/src/abi.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:surface-api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/surface-api/src/abi/HostOps
language: rust
---

# HostOps

Host operations vtable provided to the mutable surface cdylib

## Signature

```rust
pub struct HostOps
```

## Decorators

- `repr(C)`

## Visibility

- `pub`

## Docstring

Host operations vtable provided to the mutable surface cdylib
[repr(C)]

## Methods

- `create_model`
- `linear`
- `rmsnorm`
- `attention`
- `mlp_swiglu`
- `set_dtype`
- `create_adamw`
- `set_schedule_cosine`
- `adamw_step`
- `log_scalar`
- `log_grad_norms`
- `release_tensor`
- `release_model`
- `release_optim`

## Source
Lines 79–95 in `crates/surface-api/src/abi.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [abi](/crates/surface-api/src/abi.md) |
