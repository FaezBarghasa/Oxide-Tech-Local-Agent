---
okf_version: "0.2"
type: Function
title: load_library
description: Load a native shared library into memory.
resource: crates/oxide-core/src/dynamic_loader.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-core"
  - "git:branch:master"
  - "git:repo:Oxide-Tech-Local-Agent"
timestamp: "2026-09-21T11:45:48Z"
concept_id: crates/oxide-core/src/dynamic_loader/load_library_1
language: rust
---

# load_library

Load a native shared library into memory.

## Signature

```rust
pub fn load_library(
        &self,
        name: &str,
        path: P,
    ) -> Result<(), OxideError>
```

## Type Parameters

- `P: AsRef<Path`

## Visibility

- `pub`

## Docstring

Load a native shared library into memory.

## Source
Lines 30–51 in `crates/oxide-core/src/dynamic_loader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dynamic_loader](/crates/oxide-core/src/dynamic_loader.md) |
