---
okf_version: "0.2"
type: Function
title: for_inference
description: Create a high-performance topology tailored for compute-heavy local inference.
resource: crates/oxide-core/src/topology.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-core/src/topology/for_inference
language: rust
---

# for_inference

Create a high-performance topology tailored for compute-heavy local inference.

## Signature

```rust
impl RuntimeTopology { pub fn for_inference(worker_count: Option<usize>) -> Self }
```

## Visibility

- `pub`

## Docstring

Create a high-performance topology tailored for compute-heavy local inference.

## Source
Lines 34–41 in `crates/oxide-core/src/topology.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [topology](/crates/oxide-core/src/topology.md) |
