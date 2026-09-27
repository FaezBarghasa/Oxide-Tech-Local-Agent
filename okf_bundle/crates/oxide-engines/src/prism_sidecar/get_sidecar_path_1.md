---
okf_version: "0.2"
type: Function
title: get_sidecar_path
description: Locate the prism-llama-server binary from environment or local search paths.
resource: crates/oxide-engines/src/prism_sidecar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engines"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-engines/src/prism_sidecar/get_sidecar_path_1
language: rust
---

# get_sidecar_path

Locate the prism-llama-server binary from environment or local search paths.

## Signature

```rust
pub fn get_sidecar_path(binary_name: &str) -> PathBuf
```

## Visibility

- `pub`

## Docstring

Locate the prism-llama-server binary from environment or local search paths.

## Source
Lines 24–44 in `crates/oxide-engines/src/prism_sidecar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [prism_sidecar](/crates/oxide-engines/src/prism_sidecar.md) |
