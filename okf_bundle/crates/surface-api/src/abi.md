---
okf_version: "0.2"
type: Module
title: abi
description: "# `repr(C)` ABI Definition"
resource: crates/surface-api/src/abi.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:surface-api"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/surface-api/src/abi
language: rust
---

# abi

# `repr(C)` ABI Definition

## Docstring

# `repr(C)` ABI Definition

FROZEN. `repr(C)` ONLY. No candle. No generics. No heap allocations.
Host owns all tensor memory and candle types behind opaque handles.

## Relationships

| Type | Target |
|------|--------|
| related | [ModelHandle](/crates/surface-api/src/abi/ModelHandle.md) |
| related | [TensorHandle](/crates/surface-api/src/abi/TensorHandle.md) |
| related | [OptimHandle](/crates/surface-api/src/abi/OptimHandle.md) |
| related | [DTypeTag](/crates/surface-api/src/abi/DTypeTag.md) |
| related | [Dims](/crates/surface-api/src/abi/Dims.md) |
| related | [StepCtx](/crates/surface-api/src/abi/StepCtx.md) |
| related | [HookActionTag](/crates/surface-api/src/abi/HookActionTag.md) |
| related | [HookActionResult](/crates/surface-api/src/abi/HookActionResult.md) |
| related | [HostOps](/crates/surface-api/src/abi/HostOps.md) |
| related | [SurfaceVtable](/crates/surface-api/src/abi/SurfaceVtable.md) |
