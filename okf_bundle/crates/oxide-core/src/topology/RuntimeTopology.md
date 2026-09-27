---
okf_version: "0.2"
type: Class
title: RuntimeTopology
description: Hardware-aware runtime configuration for Tokio execution engines.
resource: crates/oxide-core/src/topology.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-core/src/topology/RuntimeTopology
language: rust
---

# RuntimeTopology

Hardware-aware runtime configuration for Tokio execution engines.

## Signature

```rust
pub struct RuntimeTopology
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Hardware-aware runtime configuration for Tokio execution engines.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `worker_threads`
- `stack_size_bytes`
- `thread_prefix`
- `enable_core_pinning`

## Source
Lines 6–15 in `crates/oxide-core/src/topology.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-core/src/topology.md) |
