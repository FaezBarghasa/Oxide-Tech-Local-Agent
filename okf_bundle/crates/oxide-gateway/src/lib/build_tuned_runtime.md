---
okf_version: "0.2"
type: Function
title: build_tuned_runtime
description: Construct a tuned multi-threaded Tokio runtime with CPU topology awareness and core pinning.
resource: crates/oxide-gateway/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gateway"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-25T16:34:00Z"
concept_id: crates/oxide-gateway/src/lib/build_tuned_runtime
language: rust
---

# build_tuned_runtime

Construct a tuned multi-threaded Tokio runtime with CPU topology awareness and core pinning.

## Signature

```rust
pub fn build_tuned_runtime(
    topology: Option<RuntimeTopology>,
) -> std::io::Result<tokio::runtime::Runtime>
```

## Visibility

- `pub`

## Docstring

Construct a tuned multi-threaded Tokio runtime with CPU topology awareness and core pinning.

## Source
Lines 21–49 in `crates/oxide-gateway/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-gateway/src/lib.md) |
