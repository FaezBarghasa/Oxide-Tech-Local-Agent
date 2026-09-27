---
okf_version: "0.2"
type: Function
title: connect
description: Create an engine handle connected to an already running Prism sidecar.
resource: crates/oxide-engines/src/prism_sidecar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-engines/src/prism_sidecar/connect
language: rust
---

# connect

Create an engine handle connected to an already running Prism sidecar.

## Signature

```rust
impl PrismBonsaiEngine { pub fn connect(base_url: impl Into<String>, port: u16) -> Self }
```

## Visibility

- `pub`

## Docstring

Create an engine handle connected to an already running Prism sidecar.

## Source
Lines 99–107 in `crates/oxide-engines/src/prism_sidecar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prism_sidecar](/crates/oxide-engines/src/prism_sidecar.md) |
